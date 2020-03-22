/*
pbk update versionpins --versionpin 22 --distribution 22 --pkgcoord 84 -v 432 -d 22 -p 32
*/
//use itertools::Itertools;
use crate::traits::TransactionHandler;
use crate::types::IdType;
use async_trait::async_trait;
use log;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
//use tokio_postgres::Transaction;
use deadpool_postgres::Transaction;

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

/// Models a change to a versionpin as optional new distribution and/or
/// pkgcoord_ids
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VersionPinChange {
    pub versionpin_id: IdType,
    pub distribution_id: Option<IdType>,
    pub pkgcoord_id: Option<IdType>,
}

impl VersionPinChange {
    /// New up a VersionPinChange given a versionpin id, and
    /// an option wrapped distribution id and pkgcoord id.
    ///
    /// # Arguments
    ///
    /// * `versionpin_id` The database id of the versionpin
    /// * `distribution_id` - The database id of the distribution wrapped in Some, or None
    /// * `pkgcoord_id` - The database id of the pkgcoord wrapped in Some, or None.
    ///
    /// # Returns
    ///
    /// * VersionPinChange instance
    pub fn new(
        versionpin_id: IdType,
        distribution_id: Option<IdType>,
        pkgcoord_id: Option<IdType>,
    ) -> Self {
        Self {
            versionpin_id,
            distribution_id,
            pkgcoord_id,
        }
    }
    /// Detect whether the VersionPinChange instance has Some
    /// distribution id or Some pkgcoord id.
    ///
    /// # Arguments
    /// - None
    ///
    /// # Returns
    /// * bool
    pub fn has_changes(&self) -> bool {
        self.distribution_id.is_some() || self.pkgcoord_id.is_some()
    }
}

/// Responsible for creating packages
pub struct UpdateVersionPins {
    /// vector of VersionPinChanges which will be applied to the database
    pub changes: Vec<VersionPinChange>,
    result_cnt: u64,
}

#[async_trait]
impl TransactionHandler for UpdateVersionPins {
    type Error = UpdateVersionPinsError;

    fn get_result_cnt(&self) -> u64 {
        self.result_cnt
    }

    /// zero out the result count
    fn reset_result_cnt(&mut self) {
        self.result_cnt = 0;
    }
}

impl UpdateVersionPins {
    /// new up an UpdateVersionPins instance
    ///
    /// # Arguments
    ///
    /// * `tx` - A Transaction instance
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            result_cnt: 0,
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
    pub fn change(mut self, update: VersionPinChange) -> Self {
        self.changes.push(update);
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
    pub fn change_from_components(
        mut self,
        versionpin_id: IdType,
        distribution_id: Option<IdType>,
        pkgcoord_id: Option<IdType>,
    ) -> Self {
        let change = VersionPinChange::new(versionpin_id, distribution_id, pkgcoord_id);
        self.changes.push(change);
        self
    }

    /// Add a vector of changes to the list of  changes we wish
    /// to create in the database. Like, package, packages is an infallible
    /// call that does no validation. However, I am reconsidering this.
    ///
    /// # Arguments
    /// * `changes` - A list of changes we wish to create in the db.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn changes(mut self, changes: &mut Vec<VersionPinChange>) -> Self {
        self.changes.append(changes);
        self
    }

    /// Inject updates into the internal transaction. The database update is deferred
    /// until one calls self.commit(...)
    ///
    /// # Arguments
    ///
    /// * `tx` - A mutable reference to a Postgres Transaction instance, which we use to
    ///          execute database updates within a transaction block.
    ///
    /// # Returns
    ///
    /// * Result wrapping a mutable reference to Self if ok; Otherwise, a UpdateVersionPinsError
    pub async fn update(
        &mut self,
        tx: &mut Transaction<'_>,
    ) -> Result<&mut Self, UpdateVersionPinsError> {
        let mut update_cnt: i32 = 0;
        let changes = {
            let mut empty = Vec::new();
            std::mem::swap(&mut empty, &mut self.changes);
            empty
        };

        for x in &changes {
            if x.has_changes() {
                // set up a variable that is either "" or a comma. used to
                // build up a query
                let mut maybe_comma = String::from("");
                // increment the count
                update_cnt += 1;

                // define a vector to hold the prepared statement values
                let mut updates_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut prepared_line = "UPDATE versionpin ".to_string();

                // the first index of the prepared statement. The first one is already spoken for
                let mut pos_idx: i32 = 2;
                updates_ref.push(&x.versionpin_id);
                if let Some(ref dist_id) = x.distribution_id {
                    updates_ref.push(dist_id);
                    prepared_line.push_str(
                        format!("{}SET distribution = ${}", maybe_comma, pos_idx).as_str(),
                    );
                    pos_idx += 1;
                    maybe_comma.push_str(",");
                }
                if let Some(ref pkgcoord_id) = x.pkgcoord_id {
                    updates_ref.push(pkgcoord_id);
                    prepared_line
                        .push_str(format!("{}SET coord = ${}", maybe_comma, pos_idx).as_str());
                }
                prepared_line.push_str(" WHERE id = $1");

                log::info!("SQL\n{}", prepared_line.as_str());
                log::info!("Prepared\n{:?}", &updates_ref);

                // execute the prepared statement in the database
                tx.execute(prepared_line.as_str(), &updates_ref[..])
                    .await
                    .context(TokioPostgresError {
                        msg: "failed to execute statement in transaction",
                    })?;
            }
        }
        self.result_cnt = update_cnt as u64;
        Ok(self)
    }
}
