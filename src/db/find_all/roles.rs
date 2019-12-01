pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{OrderDirection, SearchAttribute, SearchMode};
pub use crate::Coords;
pub use crate::Distribution;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindAllRolesResult<T, E = FindAllRolesError> = std::result::Result<T, E>;

fn match_attrib(search_by: &SearchAttribute) -> &'static str {
    match *search_by {
        SearchAttribute::Level => "level",
        SearchAttribute::Platform => "platform",
        SearchAttribute::Role => "role",
        SearchAttribute::Site => "site",
        SearchAttribute::Package => "distribution",
        _ => "unknown",
    }
}
/// Error type returned from  FindAllRolesError
#[derive(Debug, Snafu)]
pub enum FindAllRolesError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllRoles.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllRolesRow {
    /// the id of result in the VersionPin table
    pub role: String,
    pub level: String,
    pub platform: String,
    pub site: String,
}

impl fmt::Display for FindAllRolesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@(level:{} platform:{} site:{})",
            self.role, self.level, self.platform, self.site
        )
    }
}

impl FindAllRolesRow {
    /// New up a  FindAllRolesRow instance
    ///
    /// # Arguments
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(role: String, level: String, platform: String, site: String) -> Self {
        FindAllRolesRow {
            role,
            level,
            platform,
            site,
        }
    }
    /// Try to attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// Args
    pub fn try_from_parts(
        role: &str,
        level: &str,
        platform: &str,
        site: &str,
    ) -> FindAllRolesResult<FindAllRolesRow> {
        let coords = Coords::try_from_parts(level, role, platform, site).context(
            CoordsTryFromPartsError {
                coords: format!(
                    "(level:'{}' role:'{}' platform:'{}' site:'{}')",
                    level, role, platform, site
                ),
            },
        )?;
        let Coords {
            role,
            level,
            platform,
            site,
        } = coords;
        Ok(Self::new(
            role.to_string(),
            level.to_string(),
            platform.to_string(),
            site.to_string(),
        ))
    }

    pub fn from_parts(role: &str, level: &str, platform: &str, site: &str) -> FindAllRolesRow {
        Self::try_from_parts(role, level, platform, site).unwrap()
    }
}
/// Responsible for finding a distribution

pub struct FindAllRoles<'a> {
    client: &'a mut Client,
    role: Option<&'a str>,
    level: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
    limit: Option<i32>,
    search_mode: SearchMode,
}

impl fmt::Debug for FindAllRoles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FindAllRoles({:?} {:?} {:?} {:?})",
            self.role, self.level, self.platform, self.site
        )
    }
}

impl<'a> FindAllRoles<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        FindAllRoles {
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

    pub fn level_opt(&mut self, level: Option<&'a str>) -> &mut Self {
        self.level = level;
        self
    }

    pub fn role_opt(&mut self, role: Option<&'a str>) -> &mut Self {
        self.role = role;
        self
    }

    pub fn platform_opt(&mut self, platform: Option<&'a str>) -> &mut Self {
        self.platform = platform;
        self
    }

    pub fn site_opt(&mut self, site: Option<&'a str>) -> &mut Self {
        self.site = site;
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
    pub fn query(&mut self) -> Result<Vec<FindAllRolesRow>, Box<dyn std::error::Error>> {
        println!("{:#?}", self);
        let level = self
            .level
            .map_or("facility".to_string(), |x| format!("facility.{}", x));
        let role = self
            .role
            .map_or("any".to_string(), |x| format!("any.{}", x));
        let platform = self
            .platform
            .map_or("any".to_string(), |x| format!("any.{}", x));
        let site = self
            .site
            .map_or("any".to_string(), |x| format!("any.{}", x));
        let mut result = Vec::new();
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut params_cnt = 1;
        let mut join = "";
        let op = self.search_mode.to_symbol();
        let mut query_str =
            "SELECT DISTINCT role,level,platform,site from versionpin_view".to_string();
        // build up query to handle
        if self.level.is_some()
            || self.role.is_some()
            || self.platform.is_some()
            || self.site.is_some()
        {
            query_str.push_str(" WHERE ");
            if self.level.is_some() {
                // just including join here in case i reorder or add an additional
                // item above
                println!("HERE {:?}", self.level);

                query_str = format!(
                    "{}{} level_path {} text2ltree(${})",
                    query_str, join, op, params_cnt
                );
                params.push(&level);
                join = " AND ";
                params_cnt += 1;
            }
            if self.role.is_some() {
                query_str = format!(
                    "{}{} role_path {} text2ltree(${})",
                    query_str, join, op, params_cnt
                );
                params.push(&role);
                join = " AND ";
                params_cnt += 1;
            }
            if self.platform.is_some() {
                query_str = format!(
                    "{}{} platform_path {} text2ltree(${})",
                    query_str, join, op, params_cnt
                );
                params.push(&platform);
                join = " AND ";
                params_cnt += 1;
            }
            if self.site.is_some() {
                query_str = format!(
                    "{}{} site_path {} text2ltree(${})",
                    query_str, join, op, params_cnt
                );
                params.push(&site);
                join = " AND ";
                params_cnt += 1;
            }
        }
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
        println!("QUERY {}", qstr);
        for row in self.client.query(qstr, &params[..])? {
            let role_name: &str = row.get(0);
            let level_name: &str = row.get(1);
            let platform_name: &str = row.get(2);
            let site_name: &str = row.get(3);
            result.push(FindAllRolesRow::try_from_parts(
                role_name,
                level_name,
                platform_name,
                site_name,
            )?);
        }
        Ok(result)
    }
}
