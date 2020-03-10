use crate::traits::TransactionHandler;
use itertools::Itertools;
use log;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
use tokio_postgres::Transaction;
/// An enum which defines the kinds of InvalidLevelErrors we may encounter. .
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidLevelKind {
    TooManyLevels,
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddLevelsError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    /// If we attempt to call create without any levels registered
    /// for creation, we return a NoLevelNamesError
    #[snafu(display("No level names supplied"))]
    NoLevelNamesError,
    /// For various problems with the supplied level input
    #[snafu(display("Invalid level {:?}: {}", kind, level))]
    InvalidLevel {
        level: String,
        kind: InvalidLevelKind,
    },
}
/// The AddLevels struct is responsible for creating levels.
pub struct AddLevels<'a> {
    tx: Option<Transaction<'a>>,
    names: Vec<String>,
    result_cnt: u64,
}

impl<'a, 'b: 'a> TransactionHandler<'a, 'b> for AddLevels<'a> {
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

impl<'a> AddLevels<'a> {
    /// New up an AddLevels instance
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
            names: Vec::new(),
            result_cnt: 0,
        }
    }

    /// Add a level to the levels we will attempt to add to th DB.
    ///
    /// # Arguments
    /// * `name` - A name of a level we wish to create. This will be in
    /// addition to any level names we have already supplied using the level
    /// or levels methods. Name may be any type that implements Into<String>,
    /// so feel free to pass in a &str or a String or....
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn level<I: Into<String>>(mut self, name: I) -> Self {
        self.names.push(name.into());
        self
    }
    /// Add a vector of levels to the existing levels we will attempt to
    /// add to th DB.
    ///
    /// # Arguments
    /// * `names` - A vector of level names that we wish to create.
    /// This will be in  addition to any level names we have already supplied
    /// using the level or levels methods.
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn levels(mut self, names: &mut Vec<String>) -> Self {
        self.names.append(names);
        self
    }
    /// Create level instances in the database, returning the number of
    /// new instances created.
    ///
    /// # Returns
    /// * Ok(&mut Self) | Err(AddLevelsError)
    pub async fn create(mut self) -> Result<Self, AddLevelsError> {
        let mut expand_levels = Vec::new();
        let levels = self
            .names
            .iter()
            .unique()
            .map(|x| x.to_lowercase())
            .collect::<Vec<_>>();
        if levels.len() == 0 {
            return Err(AddLevelsError::NoLevelNamesError);
        }
        for level in &levels {
            if level.matches(".").count() > 2 {
                return Err(AddLevelsError::InvalidLevel {
                    level: level.clone(),
                    kind: InvalidLevelKind::TooManyLevels,
                });
            }
            let mut previous = "facility".to_string();
            for piece in level.split(".") {
                let next = format!("{}.{}", previous, piece);
                expand_levels.push(next.clone());
                previous = next;
            }
        }
        let mut levels_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &expand_levels {
            levels_ref.push(p);
        }
        let mut insert_str = "INSERT INTO level (path) VALUES ".to_string();
        let prepared = (1..=levels_ref.len())
            .into_iter()
            .map(|x| format!(" (text2ltree(${}))", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (path) DO NOTHING");
        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &levels_ref);
        // here we limit the lifetime of tx, so that we can return &mut self

        let results = self
            .tx()
            .unwrap()
            .execute(insert_str.as_str(), &levels_ref[..])
            .await
            .context(TokioPostgresError {
                msg: "failed to add levels",
            })?;
        self.result_cnt = results;
        Ok(self)
    }
}
