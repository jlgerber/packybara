/*
pbk update versionpins --versionpin 22 --distribution 22 --pkgcoord 84 -v 432 -d 22 -p 32
*/
//use itertools::Itertools;
use crate::types::IdType;
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
    /// * `versionpin_id` The database id of the versionpin
    /// * `distribution_id` - The database id of the distribution wrapped in Some, or None
    /// * `pkgcoord_id` - The database id of the pkgcoord wrapped in Some, or None.
    ///
    /// # Returns
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
pub struct UpdateVersionPins<'a> {
    pub client: &'a mut Client,
    pub changes: Vec<VersionPinChange>,
    pub comment: &'a str,
    pub author: &'a str,
}

impl<'a> UpdateVersionPins<'a> {
    /// new up an UpdateVersionPins instance
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to a postgres::Client instance, which
    /// stores the connection to the database, and provides crud methods
    /// for us.
    ///
    pub fn new(client: &'a mut Client, comment: &'a str, author: &'a str) -> Self {
        Self {
            client,
            changes: Vec::new(),
            comment,
            author,
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
    pub fn change(&'a mut self, update: VersionPinChange) -> &mut Self {
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
        &'a mut self,
        versionpin_id: IdType,
        distribution_id: Option<IdType>,
        pkgcoord_id: Option<IdType>,
    ) -> &mut Self {
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
    pub fn changes(&'a mut self, changes: &mut Vec<VersionPinChange>) -> &mut Self {
        self.changes.append(changes);
        self
    }

    /// update previously registered versionpin in the database. This call is
    /// fallible, and may return either the number of new packages created, or a
    /// relevant error.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns Result
    /// * Ok(u64) | Err(UpdateVersionPinsError)
    pub fn update(&mut self) -> Result<usize, UpdateVersionPinsError> {
        if self.changes.len() == 0 {
            return Err(UpdateVersionPinsError::NoUpdatesError);
        }
        let mut cnt = 0;
        let mut tx = self.client.transaction().context(TokioPostgresError {
            msg: "failed to create transaction",
        })?;
        let mut update_cnt = 0;
        let mut failures = Vec::new();
        self.changes.iter().for_each(|x| {
            if x.has_changes() {
                let mut maybe_comma = String::from("");
                update_cnt += 1;
                let mut updates_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
                let mut prepared_line = "UPDATE versionpin ".to_string();
                let mut pos_idx = 2;
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
                    //pos_idx += 1;
                }
                prepared_line.push_str(" WHERE id = $1");
                log::info!("SQL\n{}", prepared_line.as_str());
                log::info!("Prepared\n{:?}", &updates_ref);
                let results = tx.execute(prepared_line.as_str(), &updates_ref[..]);
                if results.is_err() {
                    failures.push(results);
                }
                cnt += 1;
            }
        });
        if failures.len() > 0 {
            tx.rollback().context(TokioPostgresError {
                msg: "failed to rollback",
            })?;
            return Err(UpdateVersionPinsError::TokioPostgresError {
                msg: "failed to update db.",
                source: failures.pop().unwrap().unwrap_err(),
            });
        } else if update_cnt == 0 {
            tx.rollback().context(TokioPostgresError {
                msg: "failed to rollback",
            })?;
            return Err(UpdateVersionPinsError::NoUpdatesError);
        } else {
            tx.execute(
                "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                &[&self.author, &self.comment],
            )
            .context(TokioPostgresError {
                msg: "failed to update revision entity",
            })?;
            tx.commit().context(TokioPostgresError {
                msg: "failed to commit",
            })?;
        }

        Ok(update_cnt)
    }
}
