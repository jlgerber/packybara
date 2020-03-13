use super::versionpins::{FindVersionPinsError, FindVersionPinsRow};
use crate::coords_error::CoordsError;
use crate::types::IdType;
pub use crate::Distribution;
use log;
use snafu::{ResultExt, Snafu};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum FindVersionPinError {
    /// No Query results obtained
    #[snafu(display("No Results were obtained by search"))]
    NoQueryResults,
    /// A problem has occured while trying to create the distribution
    #[snafu(display("Error calling Distribution::new({}): {}", input, source))]
    CreateDistributionError { input: String, source: CoordsError },
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
    /// An error has occured while trying to instantiate a FindVersionPinsRow
    #[snafu(display("Error Constructing FindVersionPinsRow {}", source))]
    FindVersionPinsRowError { source: FindVersionPinsError },
}

/// Contains the parameters used to search for the distribution and its
/// with distributions.
pub struct FindVersionPin<'a> {
    /// The name of the package we are interested in searching for
    package: &'a str,
    /// The optional level we wish to start our search at
    level: Option<&'a str>,
    /// The optional role we wish to start search search at
    role: Option<&'a str>,
    /// The optional platform (eg cent7_64) we wish to start our search at
    platform: Option<&'a str>,
    /// The optional site (eg portland) we wish to start our search at
    site: Option<&'a str>,
}

impl<'a> FindVersionPin<'a> {
    /// New up a FindVersionPin instance given a client and package name
    ///
    /// # Arguments
    ///
    /// * `client` - A mutable reference to a Client instance
    /// * `package` - The name of the package whose distribution we are interested in
    ///
    /// # Returns
    ///
    /// * FindVersionPin instance
    pub fn new(package: &'a str) -> Self {
        FindVersionPin {
            package,
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    /// Set the level to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `level_n` - The level name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }

    /// Set the role to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `role`_n - The role name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    /// Set the platform to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `platform_n` - The platform name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn platform(&mut self, platform_n: &'a str) -> &mut Self {
        self.platform = Some(platform_n);
        self
    }

    /// Set the site to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `site_n` - The site name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn site(&mut self, site_n: &'a str) -> &mut Self {
        self.site = Some(site_n);
        self
    }

    /// Set the optional level to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `level_n` - An Option wrapping a level name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn level_opt(&mut self, level_n: Option<&'a str>) -> &mut Self {
        self.level = level_n;
        self
    }

    /// Set the optional role to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `role_n` - An Option wrapping a role name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn role_opt(&mut self, role_n: Option<&'a str>) -> &mut Self {
        self.role = role_n;
        self
    }

    /// Set the optional platform to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `platform_n` - An Option wrapping a platform name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn platform_opt(&mut self, platform_n: Option<&'a str>) -> &mut Self {
        self.platform = platform_n;
        self
    }

    /// Set the optional site to search for the VersionPin at.
    ///
    /// # Arguments
    ///
    /// * `site_n` - An Option wrapping a site name
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self, per the builder pattern
    pub fn site_opt(&mut self, site_n: Option<&'a str>) -> &mut Self {
        self.site = site_n;
        self
    }

    /// Execute the db query searching for the closest distribution to the
    /// provided (or default) package coordinates (package name, level, role, platform, site)
    /// and returning a `FindVersionPinsRow` instance if successful, which also provides
    /// the closest matching distributions for all of the with packages, making the
    /// return value suitable for evaluating contexts
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * Result
    ///   * `Ok`  - `FindVersionPinsRow` instance
    ///   * `Err` - `FindVersionPinError` instance
    pub async fn query(&self, client: &Client) -> Result<FindVersionPinsRow, FindVersionPinError> {
        let query_str = "SELECT 
            versionpin_id, 
            distribution, 
            level_name, 
            role_name, 
            site_name, 
            platform_name,
            withs
        FROM find_distribution_and_withs(
            $1, 
            role => $2, 
            platform => $3, 
            level=>$4, 
            site => $5)";
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");

        let prepared_args: &[&(dyn ToSql + std::marker::Sync)] =
            &[&self.package, &role, &platform, &level, &site];
        log::info!("SQL\n{}", query_str);
        log::info!("Arguments\n{:?}", prepared_args);
        let row = client
            .query(query_str, prepared_args)
            .await
            .context(TokioPostgresError {
                msg: "problem with select from find_distribution_and_withs",
            })?
            .pop()
            .ok_or(FindVersionPinError::NoQueryResults)?;
        let id: IdType = row.get(0);
        let distribution: &str = row.get(1);
        let level_name: &str = row.get(2);
        let role_name: &str = row.get(3);
        let site_name: &str = row.get(4);
        let platform_name: &str = row.get(5);
        let withs: Option<Vec<String>> = row.get(6);
        FindVersionPinsRow::try_from_parts(
            id,
            distribution,
            level_name,
            role_name,
            platform_name,
            site_name,
            withs,
        )
        .context(FindVersionPinsRowError)
    }
}
