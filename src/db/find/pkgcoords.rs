pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{
    JoinMode, LtreeSearchMode, OrderDirection, SearchAttribute, SearchMode,
};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::Snafu;
use std::fmt;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderPkgCoordsBy {
    #[strum(
        serialize = "package",
        serialize = "Package",
        serialize = "PACKAGE",
        to_string = "package"
    )]
    Package,
    #[strum(
        serialize = "role",
        serialize = "Role",
        serialize = "ROLE",
        to_string = "role"
    )]
    Role,
    #[strum(
        serialize = "level",
        serialize = "Level",
        serialize = "LEVEL",
        to_string = "level"
    )]
    Level,
    #[strum(
        serialize = "platform",
        serialize = "Platform",
        serialize = "PLATFORM",
        to_string = "platform"
    )]
    Platform,
    #[strum(
        serialize = "site",
        serialize = "Site",
        serialize = "SITE",
        to_string = "site"
    )]
    Site,
}

pub type FindPkgCoordsResult<T, E = FindPkgCoordsError> = std::result::Result<T, E>;

/// Error type returned from FindPkgCoordsError
#[derive(Debug, Snafu)]
pub enum FindPkgCoordsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the FindPkgCoords.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindPkgCoordsRow {
    /// the id of result in the PkgCoord table
    pub id: i32,
    pub package: String,
    pub level: String,
    pub role: String,
    pub platform: String,
    pub site: String,
}

impl fmt::Display for FindPkgCoordsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} {} {} {})",
            self.package, self.level, self.role, self.platform, self.site
        )
    }
}

impl FindPkgCoordsRow {
    /// New up a FindPkgCoordsRow instance
    ///
    /// # Arguments
    /// * `pkgcoord_id`: The id of the relevant row in the pkgcoord table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new<I: Into<String>>(
        id: i32,
        package: I,
        level: I,
        role: I,
        platform: I,
        site: I,
    ) -> Self {
        FindPkgCoordsRow {
            id,
            package: package.into(),
            level: level.into(),
            role: role.into(),
            platform: platform.into(),
            site: site.into(),
        }
    }
    /// Try to attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// Args
    pub fn try_from_parts(
        id: i32,
        package: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindPkgCoordsResult<FindPkgCoordsRow> {
        Ok(Self::new(id, package, level, role, platform, site))
    }

    pub fn from_parts(
        id: i32,
        package: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindPkgCoordsRow {
        Self::new(id, package, level, role, platform, site)
    }
}
/// Responsible for finding a distribution
pub struct FindPkgCoords<'a> {
    client: &'a mut Client,
    pub package: Option<&'a str>,
    pub level: Option<&'a str>,
    pub role: Option<&'a str>,
    pub platform: Option<&'a str>,
    pub site: Option<&'a str>,
    pub search_mode: SearchMode,
    pub order_by: Option<Vec<OrderPkgCoordsBy>>,
}

impl<'a> FindPkgCoords<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        FindPkgCoords {
            client,
            package: None,
            level: None,
            role: None,
            platform: None,
            site: None,
            search_mode: SearchMode::Ltree(LtreeSearchMode::Ancestor),
            order_by: None,
        }
    }

    pub fn package(&mut self, package_n: &'a str) -> &mut Self {
        self.package = Some(package_n);
        self
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

    pub fn search_mode(&mut self, mode: SearchMode) -> &mut Self {
        self.search_mode = mode;
        self
    }

    pub fn order_by(&mut self, order: &'a str) -> &mut Self {
        // Vec<OrderPkgCoordsBy>
        let mut orders = Vec::new();
        for o in order.split(",") {
            if let Ok(value) = OrderPkgCoordsBy::from_str(o) {
                orders.push(value);
            } else {
                log::error!("unable to order by {}", o);
            }
        }
        self.order_by = Some(orders);
        self
    }

    pub fn package_opt(&mut self, package_n: Option<&'a str>) -> &mut Self {
        self.package = package_n;
        self
    }

    pub fn level_opt(&mut self, level_n: Option<&'a str>) -> &mut Self {
        self.level = level_n;
        self
    }

    pub fn role_opt(&mut self, role_n: Option<&'a str>) -> &mut Self {
        self.role = role_n;
        self
    }

    pub fn platform_opt(&mut self, platform_n: Option<&'a str>) -> &mut Self {
        self.platform = platform_n;
        self
    }

    pub fn site_opt(&mut self, site_n: Option<&'a str>) -> &mut Self {
        self.site = site_n;
        self
    }

    pub fn order_by_opt(&mut self, order: Option<&'a str>) -> &mut Self {
        if let Some(order) = order {
            let mut orders = Vec::new();
            for o in order.split(",") {
                if let Ok(value) = OrderPkgCoordsBy::from_str(o) {
                    orders.push(value);
                } else {
                    log::error!("unable to order by {}", o);
                }
            }
            self.order_by = Some(orders);
        } else {
            self.order_by = None;
        }
        self
    }

    fn prep_query_str(default: &'static str, value: &str) -> String {
        match value {
            _ if value == default => default.to_string(),
            _ if value.contains("%") => value.to_string(),
            _ => format!("{}.{}", default, value),
        }
    }
    fn get_query_str(&mut self) -> (String, Vec<String>) {
        let package = self.package.unwrap_or("");
        let level = self.level.map_or("facility".to_string(), |x| {
            Self::prep_query_str("facility", x)
        });
        let role = self
            .role
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x));
        let platform = self
            .platform
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x));
        let site = self
            .site
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x));

        let mut prepared = Vec::new();
        let mut query_str = "SELECT 
                        pkgcoord_id, 
                        package, 
                        level_name, 
                        role_name, 
                        platform_name,
                        site_name
                    FROM pkgcoord_view"
            .to_string();
        let mut cnt = 1;

        if self.package.is_some() {
            let sm = if package.contains("%s") {
                SearchMode::Like
            } else {
                SearchMode::Equal
            };
            let search_var = if sm == SearchMode::Like {
                "package_name"
            } else {
                "package"
            };
            query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
            cnt += 1;
            prepared.push(package.to_string());
        }

        let sm = if level.contains("%") {
            &SearchMode::Like
        } else {
            &self.search_mode
        };
        let search_var = if sm == &SearchMode::Like {
            "level_name"
        } else {
            "level"
        };
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(level);
        let sm = if role.contains("%") {
            &SearchMode::Like
        } else {
            &self.search_mode
        };
        let search_var = if sm == &SearchMode::Like {
            "role_name"
        } else {
            "role"
        };
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(role);
        let sm = if platform.contains("%") {
            &SearchMode::Like
        } else {
            &self.search_mode
        };
        let search_var = if sm == &SearchMode::Like {
            "platform_name"
        } else {
            "platform"
        };
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(platform);
        let sm = if site.contains("%") {
            &SearchMode::Like
        } else {
            &self.search_mode
        };
        let search_var = if sm == &SearchMode::Like {
            "site_name"
        } else {
            "site"
        };

        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        prepared.push(site);
        if let Some(ref order) = self.order_by {
            let joined = order
                .iter()
                .map(|x| x.as_ref())
                .collect::<Vec<_>>()
                .join(",");
            query_str.push_str(format!(" ORDER BY {}", joined).as_str());
        }
        (query_str, prepared)
    }

    pub fn query(&mut self) -> Result<Vec<FindPkgCoordsRow>, Box<dyn std::error::Error>> {
        let (query_str, prep) = self.get_query_str();
        let mut result = Vec::new();
        let mut prepared_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for argval in &prep {
            prepared_args.push(argval);
        }
        log::info!("SQL\n{}", query_str);
        log::info!("Arguments\n{:?}", prepared_args);
        for row in self.client.query(query_str.as_str(), &prepared_args[..])? {
            let id: i32 = row.get(0);
            let package: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let platform_name: &str = row.get(4);
            let site_name: &str = row.get(5);
            result.push(FindPkgCoordsRow::try_from_parts(
                id,
                package,
                level_name,
                role_name,
                platform_name,
                site_name,
            )?);
        }
        Ok(result)
    }
}
