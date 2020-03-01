use itertools::Itertools;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
//use std::fmt;
use crate::traits::TransactionHandler;
use crate::{Level, Platform, Role, Site};
use log;
use std::convert::TryInto;
use tokio_postgres::Transaction;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InvalidPlatformKind {
    InvalidName,
    InvalidCharacter,
}
/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum AddVersionPinsError {
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
    /// reference to the tokio_postgres::Client, which is responsible for holding
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

    // private method that does the heavy lifting for adding a role
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

    // private method tht does the heavy lifting for adding a platform
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

    // private method that does the heavy lifting for adding a site
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

    /// Create the platforms we have previously identified with the
    /// `platform` and/or `platforms` methods, returning the number of
    /// novel platforms created, if successful, or an error if unsuccessful.
    ///
    /// # Returns
    /// * Ok(u64) | Err(AddVersionPinsError)
    pub async fn create(mut self) -> Result<Self, AddVersionPinsError> {
        // make sure the various coords start with any (or facility for level)
        let platforms = self
            .platforms
            .iter()
            .unique()
            .map(|x| {
                x.to_string()
                // let xs = x.to_string();
                // if xs.starts_with("any") {
                //     xs
                // } else {
                //     format!("any.{}", x.as_ref())
                // }
            })
            .collect::<Vec<_>>();

        let levels = self
            .levels
            .iter()
            .unique()
            .map(|x| {
                x.to_string()
                //let x = x.to_string()
                // if xs.starts_with("facility") {
                //     xs
                // } else {
                //     format!("facility.{}", xs)
                // }
            })
            .collect::<Vec<_>>();

        let roles = self
            .roles
            .iter()
            .unique()
            .map(|x| {
                x.to_string()
                // if x.as_ref().starts_with("any") {
                //     x.as_ref().to_string()
                // } else {
                //     format!("any.{}", x.as_ref())
                // }
            })
            .collect::<Vec<_>>();

        let sites = self
            .sites
            .iter()
            .unique()
            .map(|x| {
                x.to_string()
                // if x.as_ref().starts_with("any") {
                //     x.as_ref().to_string()
                // } else {
                //     format!("any.{}", x.as_ref())
                // }
            })
            .collect::<Vec<_>>();

        // If the user has not thought to actually add components before calling
        // create, that is bad. Lets return an Error.
        if platforms.len() == 0 {
            return Err(AddVersionPinsError::NoPlatformsError);
        }
        if roles.len() == 0 {
            return Err(AddVersionPinsError::NoRolesError);
        }
        if levels.len() == 0 {
            return Err(AddVersionPinsError::NoLevelsError);
        }
        if sites.len() == 0 {
            return Err(AddVersionPinsError::NoSitesError);
        }
        let mut result_cnt: u64 = 0;
        // let package = self.package.clone();
        // let version = self.version.clone();
        let dist = format!("{}-{}", self.package, self.version);
        let tx = self.tx().await.expect("unable to create a transaction");
        for role in roles {
            for level in &levels {
                for platform in &platforms {
                    for site in &sites {
                        //                         let insert_str = "INSERT INTO pkgcoord(package,role,level,site,platform) VALUES($1, text2ltree($2), text2ltree($3), text2ltree($4), text2ltree($5)) ON CONFLICT DO NOTHING";
                        //                         let args: Vec<&(dyn ToSql + Sync)> =
                        //                             vec![&package, &role, &level, &site, &platform];
                        //                         log::info!("Sql: {}", insert_str);
                        //                         log::info!("Args:{:?}", &args);
                        //                         tx.execute(insert_str, &args[..])
                        //                             .context(TokioPostgresError {
                        //                                 msg: "failed to insert pkgcoord",
                        //                             })?;
                        //                         let insert_str = "INSERT INTO versionpin(distribution, coord)
                        //  WITH
                        //    t1 AS
                        //      (SELECT id FROM distribution WHERE package=$1 AND version=$2
                        //    ),
                        //    t2 AS
                        //     (SELECT id FROM pkgcoord WHERE package=$1 AND role=$3 AND level=$4 AND platform=$5 AND site=$6)
                        //    SELECT t1.id, t2.id
                        //    FROM t1,t2 ON CONFLICT DO NOTHING";
                        //                         let args: Vec<&(dyn ToSql + Sync)> =
                        //                             vec![&package, &version, &role, &level, &platform, &site];
                        let insert_str = "SELECT * from INSERT_VERSIONPIN($1, level_n => $2, site_n => $3, role_n => $4, platform_n => $5)";
                        let args: Vec<&(dyn ToSql + Sync)> =
                            vec![&dist, &level, &site, &role, &platform];

                        log::info!("Sql: {}", insert_str);
                        log::info!("Args:{:?}", &args);
                        let results = tx.execute(insert_str, &args[..]).await.context(
                            TokioPostgresError {
                                msg: "failed to insert versionpin",
                            },
                        )?;
                        result_cnt += results;
                    }
                }
            }
        }
        self.result_cnt = result_cnt;
        Ok(self)
    }
}
