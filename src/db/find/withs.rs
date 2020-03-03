pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::IdType;
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindWithsResult<T, E = FindWithsError> = std::result::Result<T, E>;

/// Error type returned from FindDistributionsError
#[derive(Debug, Snafu)]
pub enum FindWithsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
    /// Error from postgres
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
}

/// A row returned from the FindDistributions.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindWithsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: IdType,
    pub distribution: Distribution,
    pub coords: Coords,
}

impl fmt::Display for FindWithsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.versionpin_id, self.distribution, self.coords
        )
    }
}

impl FindWithsRow {
    /// New up a FindDistributionsRow instance
    ///
    /// # Arguments
    /// * `versionpin_id`: The id of the relevant row in the versionpin table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(versionpin_id: IdType, distribution: Distribution, coords: Coords) -> Self {
        FindWithsRow {
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
        id: IdType,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindWithsResult<FindWithsRow> {
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
        id: IdType,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindWithsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(id, distribution, coords)
    }
}
/// Responsible for finding a distribution
pub struct FindWiths<'a> {
    client: &'a mut Client,
    package: &'a str,
    level: Option<&'a str>,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
}

impl<'a> FindWiths<'a> {
    pub fn new(client: &'a mut Client, package: &'a str) -> Self {
        FindWiths {
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

    pub fn level_opt(&mut self, level_n: Option<&'a str>) -> &mut Self {
        self.level = level_n;
        self
    }

    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    pub fn role_opt(&mut self, role_n: Option<&'a str>) -> &mut Self {
        self.role = role_n;
        self
    }
    pub fn platform(&mut self, platform_n: &'a str) -> &mut Self {
        self.platform = Some(platform_n);
        self
    }
    pub fn platform_opt(&mut self, platform_n: Option<&'a str>) -> &mut Self {
        self.platform = platform_n;
        self
    }
    pub fn site(&mut self, site_n: &'a str) -> &mut Self {
        self.site = Some(site_n);
        self
    }
    pub fn site_opt(&mut self, site_n: Option<&'a str>) -> &mut Self {
        self.site = site_n;
        self
    }
    pub fn order_by_opt(&mut self, attributes: Option<Vec<SearchAttribute>>) -> &mut Self {
        self.order_by = attributes;
        self
    }

    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }

    pub fn order_direction_opt(&mut self, direction: Option<OrderDirection>) -> &mut Self {
        self.order_direction = direction;
        self
    }
    pub fn query(&mut self) -> Result<Vec<FindWithsRow>, FindWithsError> {
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

        let prep_vals: &[&(dyn ToSql + std::marker::Sync)] =
            &[&self.package, &role, &platform, &level, &site];
        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Arguments\n{:?}", prep_vals);

        for row in self
            .client
            .query(query_str.as_str(), prep_vals)
            .context(TokioPostgresError {
                msg: "problem querying withs row",
            })?
        {
            let id: IdType = row.get(0);
            let distribution: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let site_name: &str = row.get(4);
            let platform_name: &str = row.get(5);
            result.push(FindWithsRow::try_from_parts(
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
