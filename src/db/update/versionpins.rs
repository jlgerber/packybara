/*
pbk update versionpins --versionpin 22 --distribution 22 --pkgcoord 84 -v 432 -d 22 -p 32
*/
//use itertools::Itertools;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};

/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum UpdateVersionPinsError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No update data supplied"))]
    NoUpdatesError,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VersionPinUpdate {
    pub versionpin_id: IdType,
    pub distribution_id: IdType,
    pub pkgcoord_id: IdType,
}

impl VersionPinUpdate {
    pub fn new(versionpin_id: IdType, distribution_id: IdType, pkgcoord_id: IdType) -> Self {
        Self {
            versionpin_id,
            distribution_id,
            pkgcoord_id,
        }
    }
}
type IdType = i32;
/// Responsible for creating packages
pub struct UpdateVersionPins<'a> {
    pub client: &'a mut Client,
    pub updates: Vec<VersionPinUpdate>,
}

impl<'a> UpdateVersionPins<'a> {
    /// new up an UpdateVersionPins instance
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to a postgres::Client instance, which
    /// stores the connection to the database, and provides crud methods
    /// for us.
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            updates: Vec::new(),
        }
    }

    /// Add a package name to the list of packages we wish to create.
    ///
    /// # Arguments
    ///
    /// * `name` - A package we wish to create in the db. Currently, all
    /// validation is done at creation request time. However, I should consider
    /// making this call fallible, and validating name up front.
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn update(&'a mut self, update: VersionPinUpdate) -> &mut Self {
        self.updates.push(update);
        self
    }

    /// Add an Update instance to the list of Update instances we wish to create.
    ///
    /// # Arguments
    ///
    /// * `name` - A package we wish to create in the db. Currently, all
    /// validation is done at creation request time. However, I should consider
    /// making this call fallible, and validating name up front.
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn update_from_components(
        &'a mut self,
        versionpin_id: IdType,
        distribution_id: IdType,
        pkgcoord_id: IdType,
    ) -> &mut Self {
        let update = VersionPinUpdate::new(versionpin_id, distribution_id, pkgcoord_id);
        self.updates.push(update);
        self
    }

    /// Add a vector of updates to the list of  updates we wish
    /// to create in the database. Like, package, packages is an infallible
    /// call that does no validation. However, I am reconsidering this.
    ///
    /// # Arguments
    /// * `updates` - A list of updates we wish to create in the db.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn updates(&'a mut self, updates: &mut Vec<VersionPinUpdate>) -> &mut Self {
        self.updates.append(updates);
        self
    }

    /// Create previously registered package name(s) in the database. This call is
    /// fallible, and may return either the number of new packages created, or a
    /// relevant error.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns Result
    /// * Ok(u64) | Err(UpdateVersionPinsError)
    pub fn create(&mut self) -> Result<u64, UpdateVersionPinsError> {
        if self.updates.len() == 0 {
            return Err(UpdateVersionPinsError::NoUpdatesError);
        }
        let mut updates_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        self.updates.iter().for_each(|x| {
            updates_ref.push(&x.versionpin_id);
            updates_ref.push(&x.distribution_id);
            updates_ref.push(&x.pkgcoord_id);
        });

        let mut insert_str =
            "INSERT INTO versionpin (id, distribution_id, versionpin_id) VALUES ".to_string();
        let prepared = (1..=self.updates.len())
            .into_iter()
            .map(|x| format!(" (${}, ${}, ${})", x * 3, x * 3 + 1, x * 3 + 2))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (name) DO NOTHING");
        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &updates_ref);
        let results = self
            .client
            .execute(insert_str.as_str(), &updates_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add packages",
            })?;
        Ok(results)
    }
}
