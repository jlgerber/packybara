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

pub type FindAllVersionPinsResult<T, E = FindAllVersionPinsError> = std::result::Result<T, E>;

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
/// Error type returned from  FindAllVersionPinsError
#[derive(Debug, Snafu)]
pub enum FindAllVersionPinsError {
    ///  VersionPinNewError - failure to new up a versionpin.
    #[snafu(display("Error constructing VersionPin from {}: {}", msg, source))]
    VersionPinNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllVersionPins.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllVersionPinsRow {
    /// the id of result in the VersionPin table
    pub versionpin_id: IdType,
    pub distribution_id: IdType,
    pub pkgcoord_id: IdType,
    pub distribution: Distribution,
    pub coords: Coords,
    pub withs: Option<Vec<String>>,
}

impl fmt::Display for FindAllVersionPinsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = write!(
            f,
            "{} (vp id:{} dist id:{} pkgcoord id:{}) {}",
            self.distribution,
            self.versionpin_id,
            self.distribution_id,
            self.pkgcoord_id,
            self.coords
        );
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

impl FindAllVersionPinsRow {
    /// New up a  FindAllVersionPinsRow instance
    ///
    /// # Arguments
    /// * `versionpin_id`: The id of the relevant row in the versionpin table
    /// * `distribution`: The distribution found
    /// * `coords`: The location in package space that the distribution resides at
    pub fn new(
        versionpin_id: IdType,
        distribution_id: IdType,
        pkgcoord_id: IdType,
        distribution: Distribution,
        coords: Coords,
        withs: Option<Vec<String>>,
    ) -> Self {
        FindAllVersionPinsRow {
            versionpin_id,
            distribution_id,
            pkgcoord_id,
            distribution,
            coords,
            withs,
        }
    }
    /// Try to attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    pub fn try_from_parts(
        id: IdType,
        distribution_id: IdType,
        pkgcoord_id: IdType,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
        withs: Option<Vec<String>>,
    ) -> FindAllVersionPinsResult<FindAllVersionPinsRow> {
        let new_distribution = Distribution::new(distribution).context(VersionPinNewError {
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

        Ok(Self::new(
            id,
            distribution_id,
            pkgcoord_id,
            new_distribution,
            coords,
            withs,
        ))
    }

    pub fn from_parts(
        id: IdType,
        distribution_id: IdType,
        pkgcoord_id: IdType,
        distribution: &str,
        level: &str,
        role: &str,
        platform: &str,
        site: &str,
        withs: Option<Vec<String>>,
    ) -> FindAllVersionPinsRow {
        let distribution = Distribution::new_unchecked(distribution);
        let coords = Coords::try_from_parts(level, role, platform, site).unwrap();

        Self::new(
            id,
            distribution_id,
            pkgcoord_id,
            distribution,
            coords,
            withs,
        )
    }
}
/// Responsible for finding a distribution
pub struct FindAllVersionPins<'a> {
    client: &'a mut Client,
    package: Option<&'a str>,
    version: Option<&'a str>,
    level: Option<&'a str>,
    isolate_facility: bool,
    role: Option<&'a str>,
    platform: Option<&'a str>,
    site: Option<&'a str>,
    order_by: Option<Vec<SearchAttribute>>,
    order_direction: Option<OrderDirection>,
    limit: Option<IdType>,
    search_mode: LtreeSearchMode,
}

impl<'a> FindAllVersionPins<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        FindAllVersionPins {
            client,
            package: None,
            version: None,
            level: None,
            isolate_facility: false,
            role: None,
            platform: None,
            site: None,
            order_by: None,
            order_direction: None,
            limit: None,
            search_mode: LtreeSearchMode::Ancestor,
        }
    }

    pub fn package(&mut self, package_n: &'a str) -> &mut Self {
        self.package = Some(package_n);
        self
    }
    pub fn some_package(&mut self, package_n: Option<&'a str>) -> &mut Self {
        self.package = package_n;
        self
    }

    pub fn version(&mut self, version_n: &'a str) -> &mut Self {
        self.version = Some(version_n);
        self
    }
    pub fn some_version(&mut self, version_n: Option<&'a str>) -> &mut Self {
        self.version = version_n;
        self
    }

    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }
    /// If isolate_facility is True, then we scope our search to
    /// the current show when taking search_mode into account
    pub fn isolate_facility(&mut self, isolate: bool) -> &mut Self {
        self.isolate_facility = isolate;
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

    pub fn limit(&mut self, limit: IdType) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn search_mode(&mut self, mode: LtreeSearchMode) -> &mut Self {
        self.search_mode = mode;
        self
    }

    pub fn query(&mut self) -> Result<Vec<FindAllVersionPinsRow>, Box<dyn std::error::Error>> {
        let level = self.level.unwrap_or("facility").to_string();
        let role = self.role.unwrap_or("any").to_string();
        let platform = self.platform.unwrap_or("any").to_string();
        let site = self.site.unwrap_or("any").to_string();
        let mut result = Vec::new();
        let mut query_str = "SELECT id, 
        distribution_id,
        pkgcoord_id,
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
        search_mode => $5"
            .to_string();

        let smode = self.search_mode.to_string();
        let mut prepared_args: Vec<&(dyn ToSql + Sync)> =
            vec![&role, &platform, &level, &site, &smode];
        let mut params_cnt = 6;
        if let Some(ref package) = self.package {
            query_str = format!("{},\n\tpackage_name => ${}", query_str, params_cnt);
            params_cnt += 1;
            prepared_args.push(package);
        }
        if let Some(ref version) = self.version {
            query_str = format!("{},\n\tversion_name => ${}", query_str, params_cnt);
            // dont need the following, since we are the last param to get optionally added
            // but uncomment if adding more
            //params_cnt += 1;
            prepared_args.push(version);
        }
        query_str = format!("{})", query_str);
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
        // commented out for now
        //let facility_search_mode = String::from("exact");
        let qstr = query_str.as_str();
        // we do something special here if we are looking for facility.
        // if self.isolate_facility == true && &level == "facility" {
        //     prepared_args.push(&facility_search_mode);
        // } else {
        //     prepared_args.push(&smode);
        // };

        log::info!("SQL\n{}", qstr);
        log::info!("Arguents\n{:?}", prepared_args);
        for row in self.client.query(qstr, &prepared_args[..])? {
            let level_name: &str = row.get(4);
            // this should be done at the query level, but i have to switch this
            // from using a function to raw sql. Or alter the function
            if self.isolate_facility == true {
                if (level == "facility" && level_name != "facility")
                    || (level != "facility" && level_name == "facility")
                {
                    continue;
                }
            }
            let id: IdType = row.get(0);
            let dist_id: IdType = row.get(1);
            let pkgcoord_id: IdType = row.get(2);
            let distribution: &str = row.get(3);
            let role_name: &str = row.get(5);
            let site_name: &str = row.get(6);
            let platform_name: &str = row.get(7);
            let withs: Option<Vec<String>> = row.get(8);
            result.push(FindAllVersionPinsRow::try_from_parts(
                id,
                dist_id,
                pkgcoord_id,
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
