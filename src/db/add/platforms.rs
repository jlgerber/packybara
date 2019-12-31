use itertools::Itertools;
use postgres::types::ToSql;
//use postgres::Client;
use snafu::{ResultExt, Snafu};
//use std::fmt;
use crate::traits::TransactionHandler;
use log;
use postgres::Transaction;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidPlatformKind {
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddPlatformsError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No platform names supplied"))]
    NoPlatformNamesError,
    #[snafu(display("Invalid platform {:?}: {}", kind, platform))]
    InvalidPlatform {
        platform: String,
        kind: InvalidPlatformKind,
    },
}

/// Responsible for creating platforms
pub struct AddPlatforms<'a> {
    tx: Option<Transaction<'a>>,
    names: Vec<String>,
    result_cnt: u64,
}

impl<'a> TransactionHandler<'a> for AddPlatforms<'a> {
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

impl<'a> AddPlatforms<'a> {
    /// New up an AddPlatforms instance. This function takes a mutable
    /// reference to the postgres::Client, which is responsible for holding
    /// a connection to the database, as well as providing a crud interface.
    ///
    /// # Arguments
    /// * `client` a mutable reference to a postgress:Client
    ///
    /// # Returns
    /// * an instance of AddPlatforms
    pub fn new(tx: Transaction<'a>) -> Self {
        Self {
            tx: Some(tx),
            names: Vec::new(),
            result_cnt: 0,
        }
    }

    /// Add a platform name to the list of platform names that we wish to
    /// create in the database.
    ///
    /// # Arguments
    /// * `name` - The name of a platform we wish to create, provided as any
    /// type which implements Into<String> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn platform<I>(mut self, name: I) -> Self
    where
        I: Into<String>,
    {
        self.names.push(name.into());
        self
    }

    /// Add a vector of platform names to the list of platforms we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `names` - a mutable vector of strings, which we will "consume",
    /// extending our internal list of names to create.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn platforms(mut self, names: &mut Vec<String>) -> Self {
        self.names.append(names);
        self
    }

    // Validate the platform name. Is it Invalid? If it is problematic
    // return true. Otherwise, return false
    fn platform_is_invalid(&self, platform: &str) -> bool {
        platform.matches(".").count() > 1
            || platform.matches(" ").count() > 0
            || platform.matches("__").count() > 0
    }

    // generate the prepared statement, given the number of elements that
    // it will have.
    fn generate_prepared_statement(&self, item_count: usize) -> String {
        let mut insert_str = "INSERT INTO platform (path) VALUES ".to_string();
        let prepared = (1..=item_count)
            .into_iter()
            .map(|x| format!(" (text2ltree(${}))", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (path) DO NOTHING");
        insert_str
    }

    /// Create the platforms we have previously identified with the
    /// `platform` and/or `platforms` methods, returning the number of
    /// novel platforms created, if successful, or an error if unsuccessful.
    ///
    /// # Returns
    /// * Ok(u64) | Err(AddPlatformsError)
    pub fn create(mut self) -> Result<Self, AddPlatformsError> {
        // convert the self.names of platforms to lowercase after
        // making sure the list is unique, and prefixing with 'any.'
        let platforms = self
            .names
            .iter()
            .unique()
            .map(|x| format!("any.{}", x.to_lowercase()))
            .collect::<Vec<_>>();
        // If the user has not thought to actually add names before calling
        // create, that is bad. Lets return an Error.
        if platforms.len() == 0 {
            return Err(AddPlatformsError::NoPlatformNamesError);
        }
        // Validate that the platform names are valid minimally. If
        //we encounter a problem, lets bail. Note to self, I should consider
        // moving this validation
        for platform in &platforms {
            if self.platform_is_invalid(platform) {
                return Err(AddPlatformsError::InvalidPlatform {
                    platform: platform.clone(),
                    kind: InvalidPlatformKind::InvalidCharacter,
                });
            }
        }
        let mut platforms_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &platforms {
            platforms_ref.push(p);
        }

        let insert_str = self.generate_prepared_statement(platforms_ref.len());

        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &platforms_ref);
        // execute the query, capture the results and provide a failure context.
        let results = self
            .tx()
            .unwrap()
            .execute(insert_str.as_str(), &platforms_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add platforms",
            })?;
        self.result_cnt = results;
        Ok(self)
    }
}
