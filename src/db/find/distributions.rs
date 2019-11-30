pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::Coords;
pub use crate::Distribution;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindDistributionsResult<T, E = FindDistributionsError> = std::result::Result<T, E>;

/// Error type returned from FindDistributionsError
#[derive(Debug, Snafu)]
//#[snafu(visibility = "pub(crate)")]
pub enum FindDistributionsError {
    /// NewLevelspecError
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionInitError { msg: String, source: CoordsError },

    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the FindDistributions.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindDistributionsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: i32,
    pub distribution: Distribution,
    pub coords: Coords,
}

impl fmt::Display for FindDistributionsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.distribution, self.coords)
    }
}

impl FindDistributionsRow {
    /// New up a FindDistributionsRow instance
    ///
    /// # Arguments
    /// * `versionpin_id`: The id of the relevant row in the versionpin table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(versionpin_id: i32, distribution: Distribution, coords: Coords) -> Self {
        FindDistributionsRow {
            versionpin_id,
            distribution,
            coords,
        }
    }
    /// Try to attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// Args
    pub fn try_from_parts(
        id: i32,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindDistributionsResult<FindDistributionsRow> {
        let new_distribution = Distribution::new(distribution).context(DistributionInitError {
            msg: distribution.to_string(),
        })?;

        let coords = Coords::try_from_parts(level, role, platform, site).context(
            CoordsTryFromPartsError {
                coords: format!(
                    "(level:'{}' role:'{}' platform:'{}' site:'{}')",
                    level, role, platform, site
                ),
            },
        )?;

        Ok(Self::new(id, new_distribution, coords))
    }

    pub fn from_parts(
        id: i32,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindDistributionsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(id, distribution, coords)
    }
}
/// Responsible for finding a distribution
pub struct FindDistributions<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
}

impl<'a> FindDistributions<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindDistributions {
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
    pub fn query(&mut self) -> Result<Vec<FindDistributionsRow>, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        for row in self.client.query(
            "SELECT versionpin_id, 
                    distribution, 
                    level_name, 
                    role_name, 
                    site_name, 
                    platform_name
            FROM search_distributions(
                $1, 
                role => $2, 
                platform => $3, 
                level=>$4, 
                site => $5)",
            &[&self.package, &role, &platform, &level, &site],
        )? {
            let id: i32 = row.get(0);
            let distribution: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let site_name: &str = row.get(4);
            let platform_name: &str = row.get(5);
            result.push(FindDistributionsRow::try_from_parts(
                id,
                distribution,
                level_name,
                role_name,
                platform_name,
                site_name,
            )?);
        }
        Ok(result)
    }
}
