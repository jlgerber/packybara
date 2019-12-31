use crate::traits::TransactionHandler;
use itertools::Itertools;
use log;
use postgres::types::ToSql;
use postgres::Transaction;
use snafu::{ResultExt, Snafu};

/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddPackagesError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No package names supplied"))]
    NoPackageNamesError,
}

/// Responsible for creating packages
pub struct AddPackages<'a> {
    tx: Option<Transaction<'a>>,
    names: Vec<String>,
    result_cnt: u64,
}

impl<'a> TransactionHandler<'a> for AddPackages<'a> {
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

impl<'a> AddPackages<'a> {
    /// new up an AddPackages instance
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to a postgres::Client instance, which
    /// stores the connection to the database, and provides crud methods
    /// for us.
    pub fn new(tx: Transaction<'a>) -> Self {
        Self {
            tx: Some(tx),
            names: Vec::new(),
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
    pub fn package<I: Into<String>>(mut self, name: I) -> Self {
        self.names.push(name.into());
        self
    }

    /// Add a vector of package names to the list of package names we wish
    /// to create in the database. Like, package, packages is an infallible
    /// call that does no validation. However, I am reconsidering this.
    ///
    /// # Arguments
    /// * `names` - A list of names we wish to create in the db.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn packages(mut self, names: &mut Vec<String>) -> Self {
        self.names.append(names);
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
    /// * Ok(u64) | Err(AddPackagesError)
    pub fn create(mut self) -> Result<Self, AddPackagesError> {
        let packages = self.names.iter().unique().cloned().collect::<Vec<String>>();
        if packages.len() == 0 {
            return Err(AddPackagesError::NoPackageNamesError);
        }
        let mut packages_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &packages {
            packages_ref.push(p);
        }
        let mut insert_str = "INSERT INTO package (name) VALUES ".to_string();
        let prepared = (1..=packages_ref.len())
            .into_iter()
            .map(|x| format!(" (${})", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (name) DO NOTHING");

        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &packages_ref);
        let results = self
            .tx()
            .unwrap()
            .execute(insert_str.as_str(), &packages_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add packages",
            })?;
        self.result_cnt = results;
        Ok(self)
    }
}
