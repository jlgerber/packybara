pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{OrderDirection, SearchAttribute, SearchMode};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;

pub type FindRolesResult<T, E = FindRolesError> = std::result::Result<T, E>;

// Helper function to convert a SearchAttribute to a column nme
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

/// Error type returned from  FindRolesError
#[derive(Debug, Snafu)]
pub enum FindRolesError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindRoles.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindRolesRow {
    /// the id of result in the VersionPin table
    pub role: String,
    pub level: String,
    pub platform: String,
    pub site: String,
}

impl fmt::Display for FindRolesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @ (level:{} platform:{} site:{})",
            self.role, self.level, self.platform, self.site
        )
    }
}

impl FindRolesRow {
    /// New up a  FindRolesRow instance
    ///
    /// # Arguments
    /// * `role`  - the role name
    /// * `level` - the level name
    /// * `platform` - The platform name
    /// * `site` - The site name
    ///
    /// # Returns
    /// - FindRolesRow instance
    pub fn new(role: String, level: String, platform: String, site: String) -> Self {
        FindRolesRow {
            role,
            level,
            platform,
            site,
        }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `role`
    /// * `level`
    /// * `platform`
    /// * `site`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindRolesRow instance
    /// - Err - FindRolesError
    pub fn try_from_parts(
        role: &str,
        level: &str,
        platform: &str,
        site: &str,
    ) -> FindRolesResult<FindRolesRow> {
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

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `role`
    /// * `level`
    /// * `platform`
    /// * `site`
    ///
    /// # Returns
    /// - FindRolesRow instance
    pub fn from_parts(role: &str, level: &str, platform: &str, site: &str) -> FindRolesRow {
        Self::try_from_parts(role, level, platform, site).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindRoles<'a> {
    client: &'a mut Client,
    role: Option<&'a str>,
    level: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
    limit: Option<i32>,
    search_mode: SearchMode,
    simple: bool,
}

impl fmt::Debug for FindRoles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FindRoles({:?} {:?} {:?} {:?})",
            self.role, self.level, self.platform, self.site
        )
    }
}
// helper function to simplify handing different types of queries
fn prep_query(extension: &str, op: &str, params_cnt: i32, is_like: bool) -> String {
    if is_like {
        format!(" LIKE ${}", params_cnt)
    } else {
        format!("{} {} text2ltree(${})", extension, op, params_cnt)
    }
}

impl<'a> FindRoles<'a> {
    /// new up a FIndAllRoles instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindRoles {
            client,
            level: None,
            role: None,
            platform: None,
            site: None,
            order_by: None,
            order_direction: None,
            limit: None,
            search_mode: SearchMode::Ancestor,
            simple: false,
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

    pub fn simple(&mut self, simple: bool) -> &mut Self {
        self.simple = simple;
        self
    }
    /// Set an optional level.
    ///
    /// This is generally accomplished by calling
    /// ```ignore
    /// .as_ref().map(Deref::deref)
    /// ```
    /// on an `Option<String>` to convert it into an `Option<&str>`
    pub fn level_opt(&mut self, level: Option<&'a str>) -> &mut Self {
        self.level = level;
        self
    }

    /// Set an optional role
    pub fn role_opt(&mut self, role: Option<&'a str>) -> &mut Self {
        self.role = role;
        self
    }

    /// Set an optional platform
    pub fn platform_opt(&mut self, platform: Option<&'a str>) -> &mut Self {
        self.platform = platform;
        self
    }
    /// Set an optional Site
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

    fn simple_query(&mut self) -> Result<Vec<FindRolesRow>, Box<dyn std::error::Error>> {
        let query_str = "SELECT DISTINCT 
                name
            FROM 
                role_view ORDER BY name";
        let mut result = Vec::new();
        log::info!("SQL {}", query_str);
        for row in self.client.query(query_str, &[])? {
            let role_name = row.get(0);
            result.push(FindRolesRow::try_from_parts(
                role_name, "facility", "any", "any",
            )?);
        }
        Ok(result)
    }

    /// Initiate the query based on the current state of self and return a
    /// vector of results
    pub fn query(&mut self) -> Result<Vec<FindRolesRow>, Box<dyn std::error::Error>> {
        fn process_map(root: &str, value: &str) -> String {
            if value != root {
                if !value.contains("%") {
                    format!("{}.{}", root, value)
                } else {
                    value.to_string()
                }
            } else {
                root.to_string()
            }
        }
        if self.simple {
            return self.simple_query();
        }
        let level = self
            .level
            .map_or("facility".to_string(), |x| process_map("facility", x));
        let role = self
            .role
            .map_or("any".to_string(), |x| process_map("any", x));
        let platform = self
            .platform
            .map_or("any".to_string(), |x| process_map("any", x));
        let site = self
            .site
            .map_or("any".to_string(), |x| process_map("any", x));
        let mut result = Vec::new();
        // build up a vector of parameters for the prepared search
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        // Parameter Count used to supply prepared statement
        // with correct index. It is 1's based.
        let mut params_cnt = 1;

        // used to join optional queries. will be set to " AND "
        // after first optional parameter is used
        let mut join_b = false;
        // The op is the operator symbol used in the search
        let op = self.search_mode.to_symbol();
        let mut query_str = "SELECT DISTINCT 
                role,
                level,
                platform,
                site 
            FROM 
                versionpin_view"
            .to_string();

        // build up query to handle optional parameters
        if self.level.is_some()
            || self.role.is_some()
            || self.platform.is_some()
            || self.site.is_some()
        {
            fn prep_coord(
                query_str: &mut String,
                coord: &str,
                coord_name: &str,
                join_b: &mut bool,
                op: &str,
                params_cnt: &mut i32,
            ) {
                let is_like = coord.contains("%");
                let join = if *join_b { " AND " } else { "" };
                *query_str = format!(
                    "{}{} {}{}",
                    query_str,
                    join,
                    coord_name,
                    prep_query("_path", op, *params_cnt, is_like)
                );
                *join_b = true;
                *params_cnt = *params_cnt + 1;
            }
            query_str.push_str(" WHERE ");
            if self.level.is_some() {
                // just including join here in case i reorder or add an additional
                // item above
                //
                prep_coord(
                    &mut query_str,
                    &level,
                    "level",
                    &mut join_b,
                    &op,
                    &mut params_cnt,
                );
                params.push(&level);
            }
            if self.role.is_some() {
                prep_coord(
                    &mut query_str,
                    &role,
                    "role",
                    &mut join_b,
                    &op,
                    &mut params_cnt,
                );
                params.push(&role);
            }
            if self.platform.is_some() {
                prep_coord(
                    &mut query_str,
                    &platform,
                    "platform",
                    &mut join_b,
                    &op,
                    &mut params_cnt,
                );
                params.push(&platform);
            }
            if self.site.is_some() {
                prep_coord(
                    &mut query_str,
                    &site,
                    "site",
                    &mut join_b,
                    &op,
                    &mut params_cnt,
                );
                params.push(&site);
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
        log::info!("SQL {}", qstr);
        log::info!("Prepared Arguments: {:?}", &params);
        for row in self.client.query(qstr, &params[..])? {
            let role_name: &str = row.get(0);
            let level_name: &str = row.get(1);
            let platform_name: &str = row.get(2);
            let site_name: &str = row.get(3);
            result.push(FindRolesRow::try_from_parts(
                role_name,
                level_name,
                platform_name,
                site_name,
            )?);
        }
        Ok(result)
    }
}
