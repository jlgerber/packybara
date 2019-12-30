use crate::types::IdType;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};

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
    client: &'a mut Client,
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
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
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
    pub fn create(
        &mut self,
        vpin_id: IdType,
        withs: Vec<String>,
        author: String,
        comment: String,
    ) -> Result<u64, AddWithsError> {
        if withs.len() == 0 {
            return Err(AddWithsError::NoUpdatesError);
        }
        let mut tx = self.client.transaction().context(TokioPostgresError {
            msg: "failed to create transaction",
        })?;
        tx.execute("DELETE FROM withpackage WHERE versionpin = $1", &[&vpin_id])
            .context(TokioPostgresError {
                msg: "failed to delete withs before adding new ones",
            })?;
        let mut cnt = 0;
        let mut failures = Vec::new();
        withs.iter().for_each(|x| {
            let prepared_line =
                "INSERT INTO withpackage (versionpin, package, pinorder) values ($1,$2,$3)"
                    .to_string();
            let prepared_args: &[&(dyn ToSql + std::marker::Sync)] = &[&vpin_id, &x.as_str(), &cnt];
            log::info!("SQL\n{}", prepared_line.as_str());
            log::info!("Prepared\n{:?}", &prepared_args);
            let results = tx.execute(prepared_line.as_str(), &prepared_args[..]);
            if results.is_err() {
                failures.push(results);
            }
            cnt += 1;
        });
        if failures.len() > 0 {
            tx.rollback().context(TokioPostgresError {
                msg: "failed to rollback",
            })?;
            return Err(AddWithsError::TokioPostgresError {
                msg: "failed to update db.",
                source: failures.pop().unwrap().unwrap_err(),
            });
        } else if cnt == 0 {
            tx.rollback().context(TokioPostgresError {
                msg: "failed to rollback",
            })?;
            return Err(AddWithsError::NoUpdatesError);
        } else {
            tx.execute(
                "INSERT INTO REVISION (author, comment) VALUES ($1, $2)",
                &[&author, &comment],
            )
            .context(TokioPostgresError {
                msg: "failed to update revision entity",
            })?;
            tx.commit().context(TokioPostgresError {
                msg: "failed to commit",
            })?;
        }

        Ok(cnt as u64)
    }
}
