use itertools::Itertools;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
//use std::fmt;
use log;

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
    client: &'a mut Client,
    names: Vec<String>,
}

impl<'a> AddPackages<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            names: Vec::new(),
        }
    }

    pub fn package(&'a mut self, name: String) -> &mut Self {
        self.names.push(name);
        self
    }

    pub fn packages(&'a mut self, names: &mut Vec<String>) -> &mut Self {
        self.names.append(names);
        self
    }

    pub fn create(&mut self) -> Result<u64, AddPackagesError> {
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
            .client
            .execute(insert_str.as_str(), &packages_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add packages",
            })?;
        Ok(results)
    }
}
