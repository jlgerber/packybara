/*
pbk update versionpins --versionpin 22 --distribution 22 --pkgcoord 84 -v 432 -d 22 -p 32
*/
//use itertools::Itertools;
use crate::traits::{PackratDbError, TransactionHandler};
use crate::types::IdType;
use log;
use postgres::types::ToSql;
use postgres::Client;
use postgres::Transaction;
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
    tx: Option<Transaction<'a>>,
    pub changes: Vec<VersionPinChange>,
    result_cnt: u64,
}

impl<'a> TransactionHandler<'a> for UpdateVersionPins<'a> {
    type Error = tokio_postgres::error::Error;

    fn tx(&mut self) -> Option<&mut Transaction<'a>> {
        self.tx.as_mut()
    }

    fn take_tx(&mut self) -> Transaction<'a> {
        self.tx.take().unwrap()
    }

    fn reset_result_cnt(&mut self) {
        self.result_cnt = 0;
    }

    fn get_result_cnt(&self) -> u64 {
        self.result_cnt
    }
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
    pub fn new(tx: Transaction<'a>) -> Self {
        Self {
            tx: Some(tx),
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
    pub fn change(&mut self, update: VersionPinChange) -> &mut Self {
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
        &mut self,
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
    pub fn changes(&mut self, changes: &mut Vec<VersionPinChange>) -> &mut Self {
        self.changes.append(changes);
        self
    }

    pub fn update(&mut self) -> Result<&mut Self, UpdateVersionPinsError> {
        let mut update_cnt = 0;
        let changes = {
            let mut empty = Vec::new();
            std::mem::swap(&mut empty, &mut self.changes);
            empty
        };
        for x in &changes {
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
                }
                prepared_line.push_str(" WHERE id = $1");
                log::info!("SQL\n{}", prepared_line.as_str());
                log::info!("Prepared\n{:?}", &updates_ref);

                // todo guard against possible emp
                {
                    self.tx()
                        .unwrap()
                        .execute(prepared_line.as_str(), &updates_ref[..])
                        .context(TokioPostgresError {
                            msg: "failed to execute statement in transaction",
                        })?;
                }
            }
        }
        self.result_cnt = update_cnt;
        Ok(self)
    }
}
