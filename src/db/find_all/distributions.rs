pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{OrderDirection, SearchAttribute, SearchMode};
pub use crate::Coords;
pub use crate::Distribution;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindAllDistributionsResult<T, E = FindAllDistributionsError> = std::result::Result<T, E>;

fn match_attrib(search_by: &SearchAttribute) -> &'static str {
    match *search_by {
        SearchAttribute::Level => "level_name",
        SearchAttribute::Platform => "platform_name",
        SearchAttribute::Role => "role_name",
        SearchAttribute::Site => "site_name",
        SearchAttribute::Package => "distribution",
        _ => "unknown",
    }
}
/// Error type returned from  FindAllDistributionsError
#[derive(Debug, Snafu)]
pub enum FindAllDistributionsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllDistributions.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllDistributionsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: i32,
    pub distribution: Distribution,
    pub coords: Coords,
    pub withs: Option<Vec<String>>,
}

impl fmt::Display for FindAllDistributionsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = write!(f, "{} {}", self.distribution, self.coords);
        match self.withs {
            Some(ref w) => result = write!(f, " [{}]", w.join(", ")),
            None => result = write!(f, " []"),
        }
        result
    }
}

impl FindAllDistributionsRow {
    /// New up a  FindAllDistributionsRow instance
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
        FindAllDistributionsRow {
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
    ) -> FindAllDistributionsResult<FindAllDistributionsRow> {
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
    ) -> FindAllDistributionsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(id, distribution, coords, withs)
    }
}
/// Responsible for finding a distribution
pub struct FindAllDistributions<'a> {
    client: &'a mut Client,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
    limit: Option<i32>,
    search_mode: SearchMode,
}

impl<'a> FindAllDistributions<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        FindAllDistributions {
            client,
            level: None,
            role: None,
            platform: None,
            site: None,
            order_by: None,
            order_direction: None,
            limit: None,
            search_mode: SearchMode::Ancestor,
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

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn search_mode(&mut self, mode: SearchMode) -> &mut Self {
        self.search_mode = mode;
        self
    }

    pub fn query(&mut self) -> Result<Vec<FindAllDistributionsRow>, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility");
        let role = self.role.unwrap_or("any");
        let platform = self.platform.unwrap_or("any");
        let site = self.site.unwrap_or("any");
        let mut result = Vec::new();
        let mut query_str = "SELECT id, 
        distribution, 
        level_name, 
        role_name, 
        site_name, 
        platform_name,
        withs
    FROM findall_versionpins(
        role => $1, 
        platform => $2, 
        level=>$3, 
        site => $4,
        search_mode => $5)"
            .to_string();

        if let Some(ref orderby) = self.order_by {
            let orderby = orderby.iter().map(|x| match_attrib(x)).collect::<Vec<_>>();
            query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
        }
        if let Some(ref orderdir) = self.order_direction {
            query_str.push_str(&[" ", orderdir.as_ref(), " "].concat());
        }

        if let Some(limit) = self.limit {
            query_str.push_str(format!(" LIMIT {}", limit).as_str());
        }

        let qstr = query_str.as_str();
        for row in self.client.query(
            qstr,
            &[&role, &platform, &level, &site, &self.search_mode.as_ref()],
        )? {
            let id: i32 = row.get(0);
            let distribution: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let site_name: &str = row.get(4);
            let platform_name: &str = row.get(5);
            let withs: Option<Vec<String>> = row.get(6);
            result.push(FindAllDistributionsRow::try_from_parts(
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
