use itertools::Itertools;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
//use std::fmt;
use crate::traits::TransactionHandler;
use log;
//use tokio_postgres::Transaction;
use deadpool_postgres::Transaction;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidSiteKind {
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddSitesError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No site names supplied"))]
    NoSiteNamesError,
    #[snafu(display("Invalid site {:?}: {}", kind, site))]
    InvalidSite { site: String, kind: InvalidSiteKind },
}

/// Responsible for creating sites
pub struct AddSites {
    names: Vec<String>,
    result_cnt: u64,
}

impl TransactionHandler for AddSites {
    type Error = AddSitesError;

    fn get_result_cnt(&self) -> u64 {
        self.result_cnt
    }

    /// zero out the result count
    fn reset_result_cnt(&mut self) {
        self.result_cnt = 0;
    }
}

impl AddSites {
    /// New up an AddSites instance. This function takes a mutable
    /// reference to the tokio_postgres::Client, which is responsible for holding
    /// a connection to the database, as well as providing a crud interface.
    ///
    /// # Arguments
    /// * `client` a mutable reference to a postgress:Client
    ///
    /// # Returns
    /// * an instance of AddSites
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            result_cnt: 0,
        }
    }

    /// Add a site name to the list of site names that we wish to
    /// create in the database.
    ///
    /// # Arguments
    /// * `name` - The name of a site we wish to create, provided as any
    /// type which implements Into<String> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn site<I>(mut self, name: I) -> Self
    where
        I: Into<String>,
    {
        self.names.push(name.into());
        self
    }

    /// Add a vector of site names to the list of sites we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `names` - a mutable vector of strings, which we will "consume",
    /// extending our internal list of names to create.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn sites(mut self, names: &mut Vec<String>) -> Self {
        self.names.append(names);
        self
    }

    // Validate the site name. Is it Invalid? If it is problematic
    // return true. Otherwise, return false
    fn site_is_invalid(&self, site: &str) -> bool {
        site.matches(".").count() > 1
            || site.matches(" ").count() > 0
            || site.matches("__").count() > 0
    }

    // generate the prepared statement, given the number of elements that
    // it will have.
    fn generate_prepared_statement(&self, item_count: usize) -> String {
        let mut insert_str = "INSERT INTO site (path) VALUES ".to_string();
        let prepared = (1..=item_count)
            .into_iter()
            .map(|x| format!(" (text2ltree(${}))", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (path) DO NOTHING");
        insert_str
    }

    /// Create the sites we have previously identified with the
    /// `site` and/or `sites` methods, returning the number of
    /// novel sites created, if successful, or an error if unsuccessful.
    ///
    /// # Returns
    /// * Ok(u64) | Err(AddSitesError)
    pub async fn create(mut self, tx: &mut Transaction<'_>) -> Result<Self, AddSitesError> {
        // convert the self.names of sites to lowercase after
        // making sure the list is unique, and prefixing with 'any.'
        let sites = self
            .names
            .iter()
            .unique()
            .map(|x| format!("any.{}", x.to_lowercase()))
            .collect::<Vec<_>>();
        // If the user has not thought to actually add names before calling
        // create, that is bad. Lets return an Error.
        if sites.len() == 0 {
            return Err(AddSitesError::NoSiteNamesError);
        }
        // Validate that the site names are valid minimally. If
        //we encounter a problem, lets bail. Note to self, I should consider
        // moving this validation
        for site in &sites {
            if self.site_is_invalid(site) {
                return Err(AddSitesError::InvalidSite {
                    site: site.clone(),
                    kind: InvalidSiteKind::InvalidCharacter,
                });
            }
        }
        let mut sites_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &sites {
            sites_ref.push(p);
        }

        let insert_str = self.generate_prepared_statement(sites_ref.len());

        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &sites_ref);
        // execute the query, capture the results and provide a failure context.
        let results = tx
            .execute(insert_str.as_str(), &sites_ref[..])
            .await
            .context(TokioPostgresError {
                msg: "failed to add sites",
            })?;
        self.result_cnt = results;
        Ok(self)
    }
}
