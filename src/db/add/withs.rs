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
pub struct AddWiths<'a> {
    tx: Option<Transaction<'a>>,
    result_cnt: u64,
}

impl<'a> TransactionHandler<'a> for AddWiths<'a> {
    type Error = tokio_postgres::error::Error;
    /// retrieve an Option wrapped mutable reference to the
    /// transaction
    fn tx(&mut self) -> Option<&mut Transaction<'a>> {
        self.tx.as_mut()
    }
    /// Extract the transaction from Self.
    fn take_tx(&mut self) -> Transaction<'a> {
        self.tx.take().unwrap()
    }

    /// Return the result count to 0
    fn reset_result_cnt(&mut self) {
        self.result_cnt = 0;
    }
    /// Retrieve th result count
    fn get_result_cnt(&self) -> u64 {
        self.result_cnt
    }
}

impl<'a> AddWiths<'a> {
    /// New up an AddWiths instance
    ///
    /// # Arguments
    /// * `client` - a mutable reference to a postgress::Client, which holds
    /// the database connection, among other things.
    ///
    /// # Returns
    /// * An instance of Self
    pub fn new(tx: Transaction<'a>) -> Self {
        Self {
            tx: Some(tx),
            result_cnt: 0,
        }
    }

    /// update previously registered with in the database. This call is
    /// fallible, and may return either the number of new packages created, or a
    /// relevant error.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns Result
    /// * Ok(u64) | Err(AddWithsError)
    pub async fn create(
        mut self,
        vpin_id: IdType,
        withs: Vec<String>,
    ) -> Result<Self, AddWithsError> {
        if withs.len() == 0 {
            return Err(AddWithsError::NoUpdatesError);
        }
        self.tx()
            .await
            .unwrap()
            .execute("DELETE FROM withpackage WHERE versionpin = $1", &[&vpin_id])
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
            self.tx()
                .unwrap()
                .execute(prepared_line.as_str(), &prepared_args[..])
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
