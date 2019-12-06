use itertools::Itertools;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};

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
    client: &'a mut Client,
    names: Vec<String>,
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
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            names: Vec::new(),
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
    pub fn level<I: Into<String>>(&'a mut self, name: I) -> &mut Self {
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
    pub fn levels(&'a mut self, names: &mut Vec<String>) -> &mut Self {
        self.names.append(names);
        self
    }
    /// Create level instances in the database, returning the number of
    /// new instances created.
    ///
    /// # Returns
    /// * Ok(u64) | Err(AddLevelsError)
    pub fn create(&mut self) -> Result<u64, AddLevelsError> {
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
        let results = self
            .client
            .execute(insert_str.as_str(), &levels_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add levels",
            })?;
        Ok(results)
    }
}
