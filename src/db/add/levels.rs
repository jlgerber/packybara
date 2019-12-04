use itertools::Itertools;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
//use std::fmt;
use log;

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
    #[snafu(display("No level names supplied"))]
    NoLevelNamesError,
    #[snafu(display("Invalid level {:?}: {}", kind, level))]
    InvalidLevel {
        level: String,
        kind: InvalidLevelKind,
    },
}

/// Responsible for creating levels
pub struct AddLevels<'a> {
    client: &'a mut Client,
    names: Vec<String>,
}

impl<'a> AddLevels<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            names: Vec::new(),
        }
    }

    pub fn level(&'a mut self, name: String) -> &mut Self {
        self.names.push(name);
        self
    }

    pub fn levels(&'a mut self, names: &mut Vec<String>) -> &mut Self {
        self.names.append(names);
        self
    }

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
