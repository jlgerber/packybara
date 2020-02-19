use itertools::Itertools;
use postgres::types::ToSql;
use snafu::{ResultExt, Snafu};
//use std::fmt;
use crate::traits::TransactionHandler;
use crate::{Level, Platform, Role, Site};
use log;
use postgres::Transaction;
use std::convert::TryInto;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidPlatformKind {
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddVersionpinsError {
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {:#?}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    #[snafu(display("No levels supplied"))]
    NoLevelsError,
    #[snafu(display("No roles supplied"))]
    NoRolesError,
    #[snafu(display("No platforms supplied"))]
    NoPlatformsError,
    #[snafu(display("No sites supplied"))]
    NoSitesError,
    #[snafu(display("Invalid platform {:?}: {}", kind, platform))]
    InvalidPlatform {
        platform: String,
        kind: InvalidPlatformKind,
    },
}

/// Responsible for creating version pins for the given package, version
/// level, list of roles, platform, and site
pub struct AddVersionPins<'a> {
    tx: Option<Transaction<'a>>,
    package: String,
    version: String,
    levels: Vec<Level>,
    roles: Vec<Role>,
    platforms: Vec<Platform>,
    sites: Vec<Site>,
    result_cnt: u64,
}

impl<'a> TransactionHandler<'a> for AddVersionPins<'a> {
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

impl<'a> AddVersionPins<'a> {
    /// New up an AddVersionPins instance. This function takes a mutable
    /// reference to the postgres::Client, which is responsible for holding
    /// a connection to the database, as well as providing a crud interface.
    ///
    /// # Arguments
    /// * `client` a mutable reference to a postgress:Client
    ///
    /// # Returns
    /// * an instance of AddVersionPins
    pub fn new(tx: Transaction<'a>, package: String, version: String) -> Self {
        Self {
            tx: Some(tx),
            package,
            version,
            roles: Vec::new(),
            levels: Vec::new(),
            platforms: Vec::new(),
            sites: Vec::new(),
            result_cnt: 0,
        }
    }

    fn _role<I>(&mut self, role: I)
    where
        I: TryInto<Role>,
        I::Error: std::fmt::Debug,
    {
        let irole = match role.try_into() {
            Ok(role) => role,
            Err(e) => {
                log::error!("Unable to convert into Role: {:?}. Using Role::Any", e);
                Role::Any
            }
        };
        if !self.roles.contains(&irole) {
            self.roles.push(irole);
        }
    }

    // private function that does the heavy lifting of adding a level
    fn _level<I>(&mut self, lvl: I)
    where
        I: TryInto<Level>,
        I::Error: std::fmt::Debug,
    {
        let level = match lvl.try_into() {
            Ok(level) => level,
            Err(e) => {
                log::error!(
                    "Unable to convert into Level: {:?}. Using Level::Facility",
                    e
                );
                Level::Facility
            }
        };
        if !self.levels.contains(&level) {
            self.levels.push(level);
        }
    }

    fn _platform<I>(&mut self, pltfrm: I)
    where
        I: TryInto<Platform>,
        I::Error: std::fmt::Debug,
    {
        let platform = match pltfrm.try_into() {
            Ok(platform) => platform,
            Err(e) => {
                log::error!(
                    "Unable to convert into Platform: {:?}. Using Platform::Any",
                    e
                );
                Platform::Any
            }
        };
        if !self.platforms.contains(&platform) {
            self.platforms.push(platform);
        }
    }

    fn _site<I>(&mut self, site: I)
    where
        I: TryInto<Site>,
        I::Error: std::fmt::Debug,
    {
        let isite = match site.try_into() {
            Ok(site) => site,
            Err(e) => {
                log::error!("Unable to convert into site: {:?}.Using Site::Any", e);
                Site::Any
            }
        };
        if !self.sites.contains(&isite) {
            self.sites.push(isite);
        }
    }

    /// Add a level to the list of levels that we wish to
    /// create versionpins for in the database.
    ///
    /// # Arguments
    /// * `level` - a level we wish to create as versionpin for, provided as any
    /// type which implements TryInto<Platform> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn level<I>(mut self, level: I) -> Self
    where
        I: TryInto<Level>,
        I::Error: std::fmt::Debug,
    {
        self._level(level);
        self
    }

    /// Add a role to the list of roles that we wish to
    /// create versionpins for in the database.
    ///
    /// # Arguments
    /// * `role` - a role we wish to create as versionpin for, provided as any
    /// type which implements TryInto<Platform> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn role<I>(mut self, role: I) -> Self
    where
        I: TryInto<Role>,
        I::Error: std::fmt::Debug,
    {
        self._role(role);
        self
    }

    /// Add a platform name to the list of platforms that we wish to
    /// create versionpins for in the database.
    ///
    /// # Arguments
    /// * `platform` - The platform we wish to create as versionpin for, provided as any
    /// type which implements TryInto<Platform> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn platform<I>(mut self, platform: I) -> Self
    where
        I: TryInto<Platform>,
        I::Error: std::fmt::Debug,
    {
        self._platform(platform);
        self
    }

    /// Add a site to the list of sites that we wish to
    /// create versionpins for in the database.
    ///
    /// # Arguments
    /// * `site` - a site we wish to create as versionpin for, provided as any
    /// type which implements TryInto<Platform> (so &str, String, etc)
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn site<I>(mut self, site: I) -> Self
    where
        I: TryInto<Site>,
        I::Error: std::fmt::Debug,
    {
        self._site(site);
        self
    }

    /// Add a vector of roles to the list of roles for the versionpins that we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `roles` - a mutable vector of roles, which we will "consume",
    /// extending our internal list of roles to create.
    ///
    /// # Returns
    /// * mut Self
    pub fn roles<I>(mut self, roles: &mut Vec<I>) -> Self
    where
        I: TryInto<Role>,
        I::Error: std::fmt::Debug,
    {
        for irole in roles.drain(..) {
            self._role(irole);
        }
        self
    }

    /// Add a vector of levels to the list of levels for the versionpins that we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `levels` - a reference to a mutable vector of levels, which we will "consume",
    /// extending our internal list of levels to create.
    ///
    /// # Returns
    /// * mut Self
    pub fn levels<I>(mut self, levels: &mut Vec<I>) -> Self
    where
        I: TryInto<Level>,
        I::Error: std::fmt::Debug,
    {
        for level in levels.drain(..) {
            self._level(level);
        }
        self
    }

    /// Add a vector of platforms to the list of platforms we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `platforms` - a reference to a mutable vector of strings, which we will "consume",
    /// extending our internal list of names to create.
    ///
    /// # Returns
    /// * a mutable reference to Self
    pub fn platforms<I>(mut self, platforms: &mut Vec<I>) -> Self
    where
        I: TryInto<Platform>,
        I::Error: std::fmt::Debug,
    {
        for platform in platforms.drain(..) {
            self._platform(platform);
        }
        self
    }

    /// Add a vector of sites to the list of sites for the versionpins that we intend
    /// on creating.
    ///
    /// # Arguments
    /// * `sites` - a reference to a mutable vector of sites, which we will "consume",
    /// extending our internal list of sites to create.
    ///
    /// # Returns
    /// * mut Self
    pub fn sites<I>(mut self, sites: &mut Vec<I>) -> Self
    where
        I: TryInto<Site>,
        I::Error: std::fmt::Debug,
    {
        for site in sites.drain(..) {
            self._site(site);
        }
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
    /// * Ok(u64) | Err(AddVersionpinsError)
    pub fn create(mut self) -> Result<Self, AddVersionpinsError> {
        // make sure the various coords start with any (or facility for level)
        let platforms = self
            .platforms
            .iter()
            .unique()
            .map(|x| format!("any.{}", x.as_ref()))
            .collect::<Vec<_>>();

        let levels = self
            .levels
            .iter()
            .unique()
            .map(|x| {
                let xs = x.to_string();
                if xs.starts_with("facility") {
                    xs
                } else {
                    format!("facility.{}", xs)
                }
            })
            .collect::<Vec<_>>();

        let roles = self
            .roles
            .iter()
            .unique()
            .map(|x| {
                if x.as_ref().starts_with("any") {
                    x.as_ref().to_string()
                } else {
                    format!("any.{}", x.as_ref())
                }
            })
            .collect::<Vec<_>>();

        let sites = self
            .sites
            .iter()
            .unique()
            .map(|x| {
                if x.as_ref().starts_with("any") {
                    x.as_ref().to_string()
                } else {
                    format!("any.{}", x.as_ref())
                }
            })
            .collect::<Vec<_>>();

        // If the user has not thought to actually add components before calling
        // create, that is bad. Lets return an Error.
        if platforms.len() == 0 {
            return Err(AddVersionpinsError::NoPlatformsError);
        }
        if roles.len() == 0 {
            return Err(AddVersionpinsError::NoRolesError);
        }
        if levels.len() == 0 {
            return Err(AddVersionpinsError::NoLevelsError);
        }
        if sites.len() == 0 {
            return Err(AddVersionpinsError::NoSitesError);
        }

        // Generate vectors that hold references
        let mut roles_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &roles {
            roles_ref.push(p);
        }

        let mut levels_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &levels {
            levels_ref.push(p);
        }
        let mut platforms_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &platforms {
            platforms_ref.push(p);
        }

        let mut sites_ref: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for p in &sites {
            sites_ref.push(p);
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
