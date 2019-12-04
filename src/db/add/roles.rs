use itertools::Itertools;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
//use std::fmt;
use log;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidRoleKind {
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddRolesError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No role names supplied"))]
    NoRoleNamesError,
    #[snafu(display("Invalid role {:?}: {}", kind, role))]
    InvalidRole { role: String, kind: InvalidRoleKind },
}

/// Responsible for creating roles
pub struct AddRoles<'a> {
    client: &'a mut Client,
    names: Vec<String>,
}

impl<'a> AddRoles<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            names: Vec::new(),
        }
    }

    pub fn role(&'a mut self, name: String) -> &mut Self {
        self.names.push(name);
        self
    }

    pub fn roles(&'a mut self, names: &mut Vec<String>) -> &mut Self {
        self.names.append(names);
        self
    }

    pub fn create(&mut self) -> Result<u64, AddRolesError> {
        let mut expand_roles = Vec::new();
        let roles = self
            .names
            .iter()
            .unique()
            .map(|x| x.to_lowercase())
            .collect::<Vec<_>>();
        if roles.len() == 0 {
            return Err(AddRolesError::NoRoleNamesError);
        }
        for role in &roles {
            if role.matches(".").count() > 0
                || role.matches(" ").count() > 0
                || role.matches("__").count() > 0
            {
                return Err(AddRolesError::InvalidRole {
                    role: role.clone(),
                    kind: InvalidRoleKind::InvalidCharacter,
                });
            }
            let mut previous = "any".to_string();
            for piece in role.split("_") {
                let next = format!("{}.{}", previous, piece);
                expand_roles.push(next.clone());
                previous = next;
            }
        }
        let mut roles_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &expand_roles {
            roles_ref.push(p);
        }
        let mut insert_str = "INSERT INTO role (path) VALUES ".to_string();
        let prepared = (1..=roles_ref.len())
            .into_iter()
            .map(|x| format!(" (text2ltree(${}))", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (path) DO NOTHING");

        log::info!("SQL\n{}", insert_str.as_str());
        log::info!("Prepared\n{:?}", &roles_ref);
        let results = self
            .client
            .execute(insert_str.as_str(), &roles_ref[..])
            .context(TokioPostgresError {
                msg: "failed to add roles",
            })?;
        Ok(results)
    }
}
