use itertools::Itertools;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};

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
    /// New up an AddRoles instance, given a mutable reference to a
    /// postgres::Client.
    ///
    /// # Arguments
    /// * `client` - a postgres::Client instance
    ///
    /// # Returns
    /// * instance of Self
    pub fn new(client: &'a mut Client) -> Self {
        Self {
            client,
            names: Vec::new(),
        }
    }

    /// Add the name of a role we wish to create to our list of roles to create.
    ///
    /// # Arguments
    /// * `name` - The name of the role we wish to create. This may be of any type
    /// that implements Into<String> (&str, String, etc)
    ///
    /// # Returns
    /// * Mutable reference of Self
    pub fn role<I>(&'a mut self, name: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.names.push(name.into());
        self
    }
    /// Add a list of roles to our internal list of roles to be created. This function
    /// takes a vector of strings as a mutable reference, and drains it in the process
    ///
    /// # Arguments
    /// * `names` - The vector of role names we wish to create. We add this list to any
    /// previously requested names.
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn roles(&'a mut self, names: &mut Vec<String>) -> &mut Self {
        self.names.append(names);
        self
    }
    // Generate the prepared statement, given the number of variables we
    // need to account for.
    fn generate_prepared_statement(&self, item_count: usize) -> String {
        let mut insert_str = "INSERT INTO role (path) VALUES ".to_string();
        let prepared = (1..=item_count)
            .into_iter()
            .map(|x| format!(" (text2ltree(${}))", x))
            .collect::<Vec<_>>();
        let prepared = prepared.join(",");
        insert_str.push_str(prepared.as_str());
        insert_str.push_str(" ON CONFLICT (path) DO NOTHING");
        insert_str
    }
    /// Attempt to create roles, based on prior invokations of `role` or `roles`
    /// methods. If successful, create returnw the number of new roles added. If
    /// create encounters a problem, it returns an AddRolesError.
    ///
    /// # Returns
    /// * Ok(u64) | Err(AddRolesError)
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

        let insert_str = self.generate_prepared_statement(roles_ref.len());

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
