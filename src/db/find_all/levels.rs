pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::IdType;
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::Snafu;
use std::fmt;

//use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderLevelBy {
    #[strum(
        serialize = "name",
        serialize = "Name",
        serialize = "NAME",
        to_string = "name"
    )]
    Name,
    #[strum(
        serialize = "show",
        serialize = "Show",
        serialize = "SHOW",
        to_string = "show"
    )]
    Category,
}

pub type FindAllLevelsResult<T, E = FindAllLevelsError> = std::result::Result<T, E>;

/// Error type returned from  FindAllLevelsError
#[derive(Debug, Snafu)]
pub enum FindAllLevelsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllLevels.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllLevelsRow {
    pub level: String,
    pub show: String,
}

impl fmt::Display for FindAllLevelsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  ({})", self.level, self.show)
    }
}

impl FindAllLevelsRow {
    /// New up a FindAllLevelsRow instance
    ///
    /// # Arguments
    /// * `level`  - the level name
    /// * `show` - the show name
    ///
    /// # Returns
    /// - FindAllLevelsRow instance
    pub fn new(level: String, show: String) -> Self {
        FindAllLevelsRow { level, show }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `level`
    /// * `show`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllLevelsRow instance
    /// - Err - FindAllLevelsError
    pub fn try_from_parts(level: &str, show: &str) -> FindAllLevelsResult<FindAllLevelsRow> {
        // TODO: police show
        Ok(Self::new(level.to_string(), show.to_string()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `level`
    /// * `show`
    ///
    /// # Returns
    /// - FindAllLevelsRow instance
    pub fn from_parts(level: &str, show: &str) -> FindAllLevelsRow {
        Self::try_from_parts(level, show).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllLevels<'a> {
    client: &'a mut Client,
    level: Option<&'a str>,
    show: Option<&'a str>,
    depth: Option<u8>,
    order_by: Option<Vec<OrderLevelBy>>,
    order_direction: Option<OrderDirection>,
    limit: Option<IdType>,
}

impl fmt::Debug for FindAllLevels<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllLevels({:?} {:?})", self.level, self.show)
    }
}

impl<'a> FindAllLevels<'a> {
    /// new up a FIndAllLevels instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindAllLevels {
            client,
            show: None,
            level: None,
            depth: None,
            order_by: None,
            order_direction: None,
            limit: None,
        }
    }

    pub fn level(&mut self, level_n: &'a str) -> &mut Self {
        self.level = Some(level_n);
        self
    }

    pub fn show(&mut self, show_n: &'a str) -> &mut Self {
        self.show = Some(show_n);
        self
    }

    pub fn depth(&mut self, depth_n: u8) -> &mut Self {
        self.depth = Some(depth_n);
        self
    }
    /// Set an optional show.
    ///
    /// This is generally accomplished by calling
    /// ```ignore
    /// .as_ref().map(Deref::deref)
    /// ```
    /// on an `Option<String>` to convert it into an `Option<&str>`
    pub fn show_opt(&mut self, show: Option<&'a str>) -> &mut Self {
        self.show = show;
        self
    }

    /// Set an optional level
    pub fn level_opt(&mut self, level: Option<&'a str>) -> &mut Self {
        self.level = level;
        self
    }

    /// Set an optional depht
    pub fn depth_opt(&mut self, depth: Option<u8>) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn order_by(&mut self, attributes: Vec<OrderLevelBy>) -> &mut Self {
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

    pub fn query(&mut self) -> Result<Vec<FindAllLevelsRow>, Box<dyn std::error::Error>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT DISTINCT 
                name,
                show
            FROM 
                level_view WHERE name <> 'any'"
            .to_string();
        let show = self.show.unwrap_or("any");
        let mut cnt = 1;
        if self.show.is_some() {
            if show != "any" {
                query_str = format!("{} AND show = ${}", query_str, cnt);
                params.push(&show);
                cnt += 1;
            }
        }
        let depth = (self.depth.unwrap_or(0) + 1) as i32;
        if self.depth.is_some() {
            query_str = format!("{} AND nlevel(path) = ${}", query_str, cnt);
            params.push(&depth);
            cnt += 1;
        }

        if let Some(ref orderby) = self.order_by {
            let orderby = orderby.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
            query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
        } else {
            query_str = format!("{}  ORDER BY name", query_str);
        }
        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Arguments\n{:?}", &params);
        for row in self.client.query(query_str.as_str(), &params[..])? {
            let level_name = row.get(0);
            let show = row.get(1);
            result.push(FindAllLevelsRow::try_from_parts(level_name, show)?);
        }
        Ok(result)
    }
}
