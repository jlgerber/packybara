pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
pub use crate::Coords;
pub use crate::Distribution;
use log;
//use postgres::types::ToSql;
use postgres::Client;
use snafu::{ResultExt, Snafu};
use std::fmt;
//use std::str::FromStr;
//use crate::types::IdType;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderPackageBy {
    #[strum(
        serialize = "name",
        serialize = "Name",
        serialize = "NAME",
        to_string = "name"
    )]
    Name,
}

pub type FindAllPackagesResult<T, E = FindAllPackagesError> = std::result::Result<T, E>;

/// Error type returned from  FindAllPackagesError
#[derive(Debug, Snafu)]
pub enum FindAllPackagesError {
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

/// A row returned from the  FindAllPackages.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllPackagesRow {
    pub name: String,
}

impl fmt::Display for FindAllPackagesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FindAllPackagesRow {
    /// New up a  FindAllPackagesRow instance
    ///
    /// # Arguments
    /// * `name`  - the name name
    ///
    /// # Returns
    /// - FindAllPackagesRow instance
    pub fn new(name: String) -> Self {
        FindAllPackagesRow { name }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllPackagesRow instance
    /// - Err - FindAllPackagesError
    pub fn try_from_parts(name: &str) -> FindAllPackagesResult<FindAllPackagesRow> {
        // TODO: police category
        Ok(Self::new(name.to_string()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `name`
    ///
    /// # Returns
    /// - FindAllPackagesRow instance
    pub fn from_parts(name: &str) -> FindAllPackagesRow {
        Self::try_from_parts(name).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllPackages<'a> {
    client: &'a mut Client,
    //order_by: Vec<OrderPackageBy>,
    // order_direction: Option<OrderDirection>,
    // limit: Option<IdType>,
}

impl fmt::Debug for FindAllPackages<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllPackages()")
    }
}

impl<'a> FindAllPackages<'a> {
    /// new up a FIndAllPackages instance.
    ///
    /// # Arguments
    /// * `client` - a reference to a mutable Client instance
    ///
    /// # Returns
    /// * An instance of FndAllPackages
    pub fn new(client: &'a mut Client) -> Self {
        FindAllPackages {
            client,
            //order_by: Vec::new(),
            // order_direction: None,
            // limit: None,
        }
    }

    // pub fn order_by(&mut self, attributes: Vec<OrderPackageBy>) -> &mut Self {
    //     self.order_by = attributes;
    //     self
    // }
    // pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
    //     self.order_direction = Some(direction);
    //     self
    // }

    // pub fn limit(&mut self, limit: IdType) -> &mut Self {
    //     self.limit = Some(limit);
    //     self
    // }

    /// Execute the query using previously set parameters
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * an Ok wrapped Vector of FindAllPackagesRow or an Error wrapped Box dyn Error
    pub fn query(&mut self) -> FindAllPackagesResult<Vec<FindAllPackagesRow>> {
        //let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT 
                name
            FROM 
                package"
            .to_string();

        // if self.order_by.len() > 0 {
        //     query_str = format!(
        //         "{} ORDER BY {}",
        //         query_str,
        //         self.order_by
        //             .iter()
        //             .map(|x| x.as_ref())
        //             .collect::<Vec<_>>()
        //             .join(",")
        //     );
        // } else {
        query_str = format!("{}  ORDER BY name", query_str);
        //}
        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        //log::info!("Arguments\n{:?}", &params);
        for row in self
            .client
            .query(query_str.as_str(), &[])
            .context(TokioPostgresError {
                msg: "problem with select from package table",
            })?
        {
            let name = row.get(0);
            result.push(FindAllPackagesRow::try_from_parts(name)?);
        }
        Ok(result)
    }
}
