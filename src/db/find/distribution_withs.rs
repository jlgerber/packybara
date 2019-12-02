pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{OrderDirection, SearchAttribute, SearchMode};
pub use crate::Coords;
pub use crate::Distribution;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindDistributionWithsResult<T, E = FindDistributionWithsError> = std::result::Result<T, E>;

/// Error type returned from FindDistributionsError
#[derive(Debug, Snafu)]
pub enum FindDistributionWithsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the FindDistributions.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindDistributionWithsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: i32,
    pub distribution: Distribution,
    pub coords: Coords,
}

impl fmt::Display for FindDistributionWithsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.versionpin_id, self.distribution, self.coords
        )
    }
}

impl FindDistributionWithsRow {
    /// New up a FindDistributionsRow instance
    ///
    /// # Arguments
    /// * `versionpin_id`: The id of the relevant row in the versionpin table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(versionpin_id: i32, distribution: Distribution, coords: Coords) -> Self {
        FindDistributionWithsRow {
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
    ) -> FindDistributionWithsResult<FindDistributionWithsRow> {
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

        Ok(Self::new(id, new_distribution, coords))
    }

    pub fn from_parts(
        id: i32,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindDistributionWithsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(id, distribution, coords)
    }
}
/// Responsible for finding a distribution
pub struct FindDistributionWiths<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
}

impl<'a> FindDistributionWiths<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindDistributionWiths {
            client,
            package,
            level: None,
            role: None,
            platform: None,
            site: None,
            order_by: None,
            order_direction: None,
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

    pub fn order_by(&mut self, attributes: Vec<SearchAttribute>) -> &mut Self {
        self.order_by = Some(attributes);
        self
    }

    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }

    pub fn query(&mut self) -> Result<Vec<FindDistributionWithsRow>, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        let mut query_str = "SELECT 
                versionpin_id,
                distribution, 
                level_name, 
                role_name, 
                site_name, 
                platform_name
            FROM find_distribution_withs(
                $1,
                role => $2, 
                platform => $3, 
                level=> $4, 
                site => $5)"
            .to_string();
        fn from_attr_to_str(attr: &SearchAttribute) -> &'static str {
            match attr {
                SearchAttribute::Level => "level_name",
                SearchAttribute::Role => "role_name",
                SearchAttribute::Platform => "platform_name",
                SearchAttribute::Site => "site_name",
                SearchAttribute::Package => "package",
                _ => panic!("TODO add snafu Error here"),
            }
        }
        if let Some(ref orderby) = self.order_by {
            let orderby = orderby
                .iter()
                .map(|x| from_attr_to_str(&x))
                .collect::<Vec<_>>();
            query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
        }
        for row in self.client.query(
            query_str.as_str(),
            &[&self.package, &role, &platform, &level, &site],
        )? {
            let id: i32 = row.get(0);
            let distribution: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let site_name: &str = row.get(4);
            let platform_name: &str = row.get(5);
            result.push(FindDistributionWithsRow::try_from_parts(
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
