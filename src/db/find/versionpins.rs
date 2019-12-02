pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindVersionPinsResult<T, E = FindVersionPinsError> = std::result::Result<T, E>;

/// Error type returned from FindVersionPinsError
#[derive(Debug, Snafu)]
pub enum FindVersionPinsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the FindVersionPins.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindVersionPinsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: i32,
    pub distribution: Distribution,
    pub coords: Coords,
    pub withs: Option<Vec<String>>,
}

impl fmt::Display for FindVersionPinsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = write!(f, "{} {}", self.distribution, self.coords);
        // just to get the compiler to stop complaining about an unused var
        if result.is_err() {
            return result;
        }
        match self.withs {
            Some(ref w) => result = write!(f, " [{}]", w.join(", ")),
            None => result = write!(f, " []"),
        }
        result
    }
}

impl FindVersionPinsRow {
    /// New up a FindVersionPinsRow instance
    ///
    /// # Arguments
    /// * `versionpin_id`: The id of the relevant row in the versionpin table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(
        versionpin_id: i32,
        distribution: Distribution,
        coords: Coords,
        withs: Option<Vec<String>>,
    ) -> Self {
        FindVersionPinsRow {
            versionpin_id,
            distribution,
            coords,
            withs,
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
        withs: Option<Vec<String>>,
    ) -> FindVersionPinsResult<FindVersionPinsRow> {
        let new_distribution = Distribution::new(distribution).context(DistributionNewError {
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

        Ok(Self::new(id, new_distribution, coords, withs))
    }

    pub fn from_parts(
        id: i32,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
        withs: Option<Vec<String>>,
    ) -> FindVersionPinsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(id, distribution, coords, withs)
    }
}
/// Responsible for finding a distribution
pub struct FindVersionPins<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
}

impl<'a> FindVersionPins<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindVersionPins {
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
    pub fn query(&mut self) -> Result<Vec<FindVersionPinsRow>, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        let prepared_args: &[&(dyn ToSql + std::marker::Sync)] =
            &[&self.package, &role, &platform, &level, &site];
        let query_str = "SELECT versionpin_id, 
                        distribution, 
                        level_name, 
                        role_name, 
                        site_name, 
                        platform_name,
                        withs
                    FROM search_distributions(
                        $1, 
                        role => $2, 
                        platform => $3, 
                        level=>$4, 
                        site => $5)";
        log::info!("SQL {}", query_str);
        log::info!("Prepared Arguments: {:?}", prepared_args);
        for row in self.client.query(query_str, prepared_args)? {
            let id: i32 = row.get(0);
            let distribution: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let site_name: &str = row.get(4);
            let platform_name: &str = row.get(5);
            let withs: Option<Vec<String>> = row.get(6);
            result.push(FindVersionPinsRow::try_from_parts(
                id,
                distribution,
                level_name,
                role_name,
                platform_name,
                site_name,
                withs,
            )?);
        }
        Ok(result)
    }
}
