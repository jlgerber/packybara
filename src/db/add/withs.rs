use crate::traits::TransactionHandler;
use crate::types::IdType;
use log;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
use tokio_postgres::Transaction;

/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddWithsError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    /// If we attempt to call create without any withs registered
    /// for creation, we return a NoWithNamesError
    #[snafu(display("No with withs supplied"))]
    NoWithNamesError,
    #[snafu(display("No updates supplied"))]
    NoUpdatesError,
}

/// The AddWiths struct is responsible for creating withs.
pub struct AddWiths {
    result_cnt: u64,
}

impl TransactionHandler for AddWiths {
    type Error = AddWithsError;

    fn get_result_cnt(&self) -> u64 {
        self.result_cnt
    }

    /// zero out the result count
    fn reset_result_cnt(&mut self) {
        self.result_cnt = 0;
    }
}

impl AddWiths {
    /// New up an AddWiths instance
    ///
    /// # Arguments
    /// * `client` - a mutable reference to a postgress::Client, which holds
    /// the database connection, among other things.
    ///
    /// # Returns
    /// * An instance of Self
    pub fn new() -> Self {
        Self { result_cnt: 0 }
    }

    /// create withs. This call is
    /// fallible, and may return either the number of new packages created, or a
    /// relevant error.
    ///
    /// # Arguments
    /// * `vpin_id`
    /// * `withs`
    /// * `tx`
    ///
    /// # Returns Result
    /// * Ok(u64) | Err(AddWithsError)
    pub async fn create(
        mut self,
        tx: &mut Transaction<'_>,
        vpin_id: IdType,
        withs: Vec<String>,
    ) -> Result<Self, AddWithsError> {
        if withs.len() == 0 {
            return Err(AddWithsError::NoUpdatesError);
        }
        tx.execute("DELETE FROM withpackage WHERE versionpin = $1", &[&vpin_id])
            .await
            .context(TokioPostgresError {
                msg: "failed to delete withs before adding new ones",
            })?;
        let mut cnt: i32 = 0;
        for x in &withs {
            let prepared_line =
                "INSERT INTO withpackage (versionpin, package, pinorder) values ($1,$2,$3)"
                    .to_string();

            let prepared_args: &[&(dyn ToSql + std::marker::Sync)] = &[&vpin_id, &x.as_str(), &cnt];
            log::info!("SQL\n{}", prepared_line.as_str());
            log::info!("Prepared\n{:?}", &prepared_args);
            tx.execute(prepared_line.as_str(), &prepared_args[..])
                .await
                .context(TokioPostgresError {
                    msg: "problem executing prepared statement",
                })?;
            cnt += 1;
        }
        // technically this could fail / wrap / behave weird.. but
        // that would be highly unlikely in the real world.
        assert!(cnt >= 0);
        self.result_cnt = cnt as u64;

        Ok(self)
    }
}
