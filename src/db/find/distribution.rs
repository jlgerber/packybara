use crate::coords_error::CoordsError;
pub use crate::Distribution;
use postgres::Client;
use snafu::{ResultExt, Snafu};

/// Error type returned from FindDistributionsError
#[derive(Debug, Snafu)]
pub enum FindDistributionError {
    /// No Query results obtained
    #[snafu(display("No Results were obtained by search"))]
    NoQueryResults,
    ///
    #[snafu(display("Error calling Distribution::new({}): {}", input, source))]
    CreateDistributionError { input: String, source: CoordsError },
    /// When constructing a query, postgres has thrown an error
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
}

/// Responsible for finding a distribution
pub struct FindDistribution<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
}

impl<'a> FindDistribution<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindDistribution {
            client,
            package,
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }

    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    pub fn platform(&mut self, platform_n: &'a str) -> &mut Self {
        self.platform = Some(platform_n);
        self
    }

    pub fn site(&mut self, site_n: &'a str) -> &mut Self {
        self.site = Some(site_n);
        self
    }

    pub fn query(&mut self) -> Result<Distribution, FindDistributionError> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        for row in self
            .client
            .query(
                "SELECT distribution
            FROM find_distribution(
                $1, 
                role => $2, 
                platform => $3, 
                level=>$4, 
                site => $5)",
                &[&self.package, &role, &platform, &level, &site],
            )
            .context(TokioPostgresError {
                msg: "problem with select from find_distribution",
            })?
        {
            let distribution: &str = row.get(0);
            result.push(
                Distribution::new(distribution).context(CreateDistributionError {
                    input: distribution,
                })?,
            );
        }
        result.pop().ok_or(FindDistributionError::NoQueryResults)
    }
}
