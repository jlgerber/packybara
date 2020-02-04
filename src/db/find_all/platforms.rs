pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
pub use crate::Coords;
pub use crate::Distribution;
use log;
//use postgres::types::ToSql;
use postgres::Client;
use snafu::Snafu;
use std::fmt;
//use std::str::FromStr;
use crate::types::IdType;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// A simple enum representing the possible columns to order the return by.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderPlatformBy {
    #[strum(
        serialize = "name",
        serialize = "Name",
        serialize = "NAME",
        to_string = "name"
    )]
    Name,
}

pub type FindAllPlatformsResult<T, E = FindAllPlatformsError> = std::result::Result<T, E>;

/// Error type returned from  FindAllPlatformsError
#[derive(Debug, Snafu)]
pub enum FindAllPlatformsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllPlatforms.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllPlatformsRow {
    pub name: String,
}

impl fmt::Display for FindAllPlatformsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FindAllPlatformsRow {
    /// New up a  FindAllPlatformsRow instance
    ///
    /// # Arguments
    /// * `name`  - the platform name
    ///
    /// # Returns
    /// - FindAllPlatformsRow instance
    pub fn new(name: String) -> Self {
        FindAllPlatformsRow { name }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `platform`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllPlatformsRow instance
    /// - Err - FindAllPlatformsError
    pub fn try_from_parts(platform: &str) -> FindAllPlatformsResult<FindAllPlatformsRow> {
        // TODO: police category
        Ok(Self::new(platform.to_string()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `role`
    /// * `category`
    ///
    /// # Returns
    /// - FindAllPlatformsRow instance
    pub fn from_parts(platform: &str) -> FindAllPlatformsRow {
        Self::try_from_parts(platform).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllPlatforms<'a> {
    client: &'a mut Client,
    name: Option<&'a str>,
    order_by: Option<Vec<OrderPlatformBy>>,
    order_direction: Option<OrderDirection>,
    limit: Option<IdType>,
}

impl fmt::Debug for FindAllPlatforms<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllPlatforms({:?})", self.name)
    }
}

impl<'a> FindAllPlatforms<'a> {
    /// new up a FIndAllPlatforms instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindAllPlatforms {
            client,
            name: None,
            order_by: None,
            order_direction: None,
            limit: None,
        }
    }
    /// Set a the platform name
    ///
    /// # Arguments
    /// * `name` - The name of the platform as a &str
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Set an optional category.
    ///
    /// This is generally accomplished by calling
    /// ```ignore
    /// .as_ref().map(Deref::deref)
    /// ```
    /// on an `Option<String>` to convert it into an `Option<&str>`
    pub fn name_opt(&mut self, name: Option<&'a str>) -> &mut Self {
        self.name = name;
        self
    }
    /// Set the columsn to sort on
    ///
    /// # Arguments
    /// * `attributes` - The name of column or columns to order the return by
    /// as a Vector of OrderPlatformBy instances
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn order_by(&mut self, attributes: Vec<OrderPlatformBy>) -> &mut Self {
        self.order_by = Some(attributes);
        self
    }
    /// Set the sort direction
    ///
    /// # Arguments
    /// * `direction` - The direction to sort in, represented as an instance of OrderDirection
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }
    /// Set the max number of elements returned by the query
    ///
    /// # Arguments
    /// * `limit` - The max number of elements to return, as an IdType
    ///
    /// # Returns
    /// * A mutable reference to Self
    pub fn limit(&mut self, limit: IdType) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn query(&mut self) -> Result<Vec<FindAllPlatformsRow>, Box<dyn std::error::Error>> {
        //let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT DISTINCT 
                name
            FROM 
                platform_view"
            .to_string();
        if let Some(ref orderby) = self.order_by {
            let orderby = orderby.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
            query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
        } else {
            query_str = format!("{}  ORDER BY name", query_str);
        }
        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        //log::info!("Prepared: {:?}", &params);
        for row in self.client.query(query_str.as_str(), &[])? {
            //&params[..])? {
            let name = row.get(0);
            result.push(FindAllPlatformsRow::try_from_parts(name)?);
        }
        Ok(result)
    }
}
