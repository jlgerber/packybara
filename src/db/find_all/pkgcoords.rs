pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{
    JoinMode, LtreeSearchMode, OrderDirection, SearchAttribute, SearchMode,
};
pub use crate::utils::pred_true_false;
pub use crate::Coords;
pub use crate::Distribution;
use serde::Serialize;

use crate::types::IdType;
use log;
use snafu::{ResultExt, Snafu};
use std::fmt;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

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

pub type FindAllPkgCoordsResult<T, E = FindAllPkgCoordsError> = std::result::Result<T, E>;

/// Error type returned from FindAllPkgCoordsError
#[derive(Debug, Snafu)]
pub enum FindAllPkgCoordsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
    #[snafu(display("No CLient Error"))]
    NoClientError,
    /// Error from postgres
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
}

/// A row returned from the FindAllPkgCoords.query
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FindAllPkgCoordsRow {
    /// the id of result in the PkgCoord table
    pub id: IdType,
    pub package: String,
    pub level: String,
    pub role: String,
    pub platform: String,
    pub site: String,
}

impl fmt::Display for FindAllPkgCoordsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} {} {} {})",
            self.package, self.level, self.role, self.platform, self.site
        )
    }
}

impl FindAllPkgCoordsRow {
    /// New up a FindAllPkgCoordsRow instance
    ///
    /// # Arguments
    /// * `pkgcoord_id`: The id of the relevant row in the pkgcoord table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new<I: Into<String>>(
        id: IdType,
        package: I,
        level: I,
        role: I,
        platform: I,
        site: I,
    ) -> Self {
        FindAllPkgCoordsRow {
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
        id: IdType,
        package: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindAllPkgCoordsResult<FindAllPkgCoordsRow> {
        Ok(Self::new(id, package, level, role, platform, site))
    }

    pub fn from_parts(
        id: IdType,
        package: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
    ) -> FindAllPkgCoordsRow {
        Self::new(id, package, level, role, platform, site)
    }
}
/// Responsible for finding a distribution
pub struct FindAllPkgCoords<'a> {
    client: Option<&'a mut Client>,
    pub package: Option<&'a str>,
    pub level: Option<&'a str>,
    pub role: Option<&'a str>,
    pub platform: Option<&'a str>,
    pub site: Option<&'a str>,
    pub search_mode: SearchMode,
    pub order_by: Option<Vec<OrderPkgCoordsBy>>,
}

impl<'a> FindAllPkgCoords<'a> {
    /// New up an instance of FindAllPkgCoords
    ///
    /// # Arguments
    /// * `client` - An Option wrapped mutable reference to a Client instacne
    ///
    /// # Returns
    /// * An instance of FindAllPkgCoords
    pub fn new(client: Option<&'a mut Client>) -> Self {
        FindAllPkgCoords {
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

    /// Set a package name
    ///
    /// # Arguments
    /// * `package_n` - The name of the package as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn package(&mut self, package_n: &'a str) -> &mut Self {
        self.package = Some(package_n);
        self
    }
    /// Set the level
    ///
    /// # Arguments
    /// * `level_n` - The level of the level as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }
    /// Set the role
    ///
    /// # Arguments
    /// * `role_n` - The name of the role as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    /// Set a platform
    ///
    /// # Arguments
    /// * `platform_n` - The name of the platform as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn platform(&mut self, platform_n: &'a str) -> &mut Self {
        self.platform = Some(platform_n);
        self
    }
    /// Set a site name
    ///
    /// # Arguments
    /// * `site_n` - The name of the site as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn site(&mut self, site_n: &'a str) -> &mut Self {
        self.site = Some(site_n);
        self
    }

    /// Set the search mode  
    ///
    /// # Arguments
    /// * `mode` - An instance of SearchMode
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn search_mode(&mut self, mode: SearchMode) -> &mut Self {
        self.search_mode = mode;
        self
    }

    /// Set the order by
    ///
    /// # Arguments
    /// * `order` - The name of column or columns to order the return by (comma separated)
    ///
    /// # Returns
    /// * A mutable reference to Self
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
    /// Set an option wrapped package name
    ///
    /// # Arguments
    /// * `package_n` - The name of the package as an option wrapped &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn package_opt(&mut self, package_n: Option<&'a str>) -> &mut Self {
        self.package = package_n;
        self
    }
    /// Set an option wrapped level
    ///
    /// # Arguments
    /// * `level_n` - The level as an option wrapped &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn level_opt(&mut self, level_n: Option<&'a str>) -> &mut Self {
        self.level = level_n;
        self
    }
    /// Set an option wrapped role
    ///
    /// # Arguments
    /// * `package_n` - The role as an option wrapped &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn role_opt(&mut self, role_n: Option<&'a str>) -> &mut Self {
        self.role = role_n;
        self
    }
    /// Set an option wrapped platform
    ///
    /// # Arguments
    /// * `platform_n` - The platform as an option wrapped &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn platform_opt(&mut self, platform_n: Option<&'a str>) -> &mut Self {
        self.platform = platform_n;
        self
    }
    /// Set an option wrapped site
    ///
    /// # Arguments
    /// * `site_n` - The site as an option wrapped &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn site_opt(&mut self, site_n: Option<&'a str>) -> &mut Self {
        self.site = site_n;
        self
    }
    /// Set an option wrapped order_by
    ///
    /// # Arguments
    /// * `order` - The columns that the query will be sorted by as an option wrapped,
    /// comma separated &str
    ///
    /// # Returns
    /// * A mutable reference to Self
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

    fn prep_query_str(default: &'static str, value: &str, substitute: bool) -> String {
        let mut result = match value {
            _ if value == default => default.to_string(),
            _ if value.contains("%") => value.to_string(),
            _ => format!("{}.{}", default, value),
        };
        if substitute {
            result = result.replace("_", ".");
        }
        result
    }
    fn get_query_str(&mut self) -> (String, Vec<String>) {
        let package = self.package.unwrap_or("");
        let level = self.level.map_or("facility".to_string(), |x| {
            Self::prep_query_str("facility", x, false)
        });
        let role = self
            .role
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x, true));
        let platform = self
            .platform
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x, false));
        let site = self
            .site
            .map_or("any".to_string(), |x| Self::prep_query_str("any", x, false));

        let mut prepared = Vec::new();
        let mut query_str = "SELECT \n\
                             pkgcoord_id, \n\
                             package, \n\
                             level_name, \n\
                             role_name, \n\
                             platform_name, \n\
                             site_name \n\
                             FROM pkgcoord_view"
            .to_string();
        let mut cnt = 1;
        // Package
        if self.package.is_some() {
            let sm = pred_true_false(package.contains("%s"), SearchMode::Like, SearchMode::Equal);
            let search_var = pred_true_false(sm == SearchMode::Like, "package_name", "package");
            query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
            cnt += 1;
            prepared.push(package.to_string());
        }
        //Level
        let sm = pred_true_false(level.contains("%"), &SearchMode::Like, &self.search_mode);
        let search_var = pred_true_false(sm == &SearchMode::Like, "level_name", "level");
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(level);
        // Role
        let sm = pred_true_false(role.contains("%"), &SearchMode::Like, &self.search_mode);
        let search_var = pred_true_false(sm == &SearchMode::Like, "role_name", "role");
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(role);
        // Platform
        let sm = pred_true_false(platform.contains("%"), &SearchMode::Like, &self.search_mode);
        let search_var = pred_true_false(sm == &SearchMode::Like, "platform_name", "platform");
        query_str.push_str(SearchMode::search_string(search_var, &sm, cnt).as_str());
        cnt += 1;
        prepared.push(platform);
        // Site
        let sm = pred_true_false(site.contains("%"), &SearchMode::Like, &self.search_mode);
        let search_var = pred_true_false(sm == &SearchMode::Like, "site_name", "site");
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
    /// execute the query
    pub async fn query(&mut self) -> FindAllPkgCoordsResult<Vec<FindAllPkgCoordsRow>> {
        let (query_str, prep) = self.get_query_str();
        let mut result = Vec::new();
        let mut prepared_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for argval in &prep {
            prepared_args.push(argval);
        }
        log::info!("SQL\n{}", query_str);
        log::info!("Arguments\n{:?}", prepared_args);
        let client = self.client.as_mut();
        if client.is_none() {
            return Err(FindAllPkgCoordsError::NoClientError)?;
        }
        let client = client.unwrap();
        for row in client
            .query(query_str.as_str(), &prepared_args[..])
            .await
            .context(TokioPostgresError {
                msg: "problem with select from pkgcoord_view",
            })?
        {
            let id: IdType = row.get(0);
            let package: &str = row.get(1);
            let level_name: &str = row.get(2);
            let role_name: &str = row.get(3);
            let platform_name: &str = row.get(4);
            let site_name: &str = row.get(5);
            result.push(FindAllPkgCoordsRow::try_from_parts(
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_query_string_default_works() {
        let mut fpc = FindAllPkgCoords::new(None);
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "any", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_level() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility.bayou", "any", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_like_level() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou%");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE level_name LIKE $1 AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["bayou%", "any", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_subrole() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.role("fx_beta");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "any.fx.beta", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_like_role() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.role("fx%");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND role_name LIKE $2 AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "fx%", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_platform() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.platform("cent7_64");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "any", "any.cent7_64", "any"]);
    }
    #[test]
    fn get_query_string_with_like_platform() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.platform("cent7%");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND platform_name LIKE $3 AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "any", "cent7%", "any"]);
    }
    #[test]
    fn get_query_string_with_site() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.site("montreal");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND text2ltree($4) <@ site"
        );
        assert_eq!(ps, &["facility", "any", "any", "any.montreal"]);
    }
    #[test]
    fn get_query_string_with_like_site() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.site("montreal%");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE text2ltree($1) <@ level AND text2ltree($2) <@ role AND text2ltree($3) <@ platform AND site_name LIKE $4"
        );
        assert_eq!(ps, &["facility", "any", "any", "montreal%"]);
    }
    #[test]
    fn get_query_string_with_level_like_and_package() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou%").package("maya");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE package = $1 AND level_name LIKE $2 AND text2ltree($3) <@ role AND text2ltree($4) <@ platform AND text2ltree($5) <@ site"
        );
        assert_eq!(ps, &["maya", "bayou%", "any", "any", "any"]);
    }

    // this one is important as it verifies that the subrole is being
    // appropriately transformed
    #[test]
    fn get_query_string_with_level_like_and_package_and_role() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou%").package("maya").role("fx_beta");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE package = $1 AND level_name LIKE $2 AND text2ltree($3) <@ role AND text2ltree($4) <@ platform AND text2ltree($5) <@ site"
        );
        assert_eq!(ps, &["maya", "bayou%", "any.fx.beta", "any", "any"]);
    }
    #[test]
    fn get_query_string_with_level_like_and_package_role_and_platform() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou%")
            .package("maya")
            .role("fx_beta")
            .platform("cent7_64");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE package = $1 AND level_name LIKE $2 AND text2ltree($3) <@ role AND text2ltree($4) <@ platform AND text2ltree($5) <@ site"
        );
        assert_eq!(
            ps,
            &["maya", "bayou%", "any.fx.beta", "any.cent7_64", "any"]
        );
    }
    #[test]
    fn get_query_string_with_level_like_and_package_role_platform_and_site() {
        let mut fpc = FindAllPkgCoords::new(None);
        fpc.level("bayou%")
            .package("maya")
            .role("fx_beta")
            .platform("cent7_64")
            .site("montreal");
        let (qs, ps) = fpc.get_query_str();
        assert_eq!(
            qs.as_str(),
            "SELECT \n\
        pkgcoord_id, \n\
        package, \n\
        level_name, \n\
        role_name, \n\
        platform_name, \n\
        site_name \n\
    FROM pkgcoord_view WHERE package = $1 AND level_name LIKE $2 AND text2ltree($3) <@ role AND text2ltree($4) <@ platform AND text2ltree($5) <@ site"
        );
        assert_eq!(
            ps,
            &[
                "maya",
                "bayou%",
                "any.fx.beta",
                "any.cent7_64",
                "any.montreal"
            ]
        );
    }
}
