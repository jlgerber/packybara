pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use snafu::{ResultExt, Snafu};
use std::fmt;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;
//use std::str::FromStr;
//use crate::types::IdType;
use serde::Serialize;
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
#[derive(Debug, PartialEq, Eq, Serialize)]
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
    ///
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
    name: Option<&'a str>,
    //order_by: Vec<OrderPackageBy>,
    // order_direction: Option<OrderDirection>,
    // limit: Option<IdType>,
}

impl<'a> fmt::Debug for FindAllPackages<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllPackages()")
    }
}

impl<'a> FindAllPackages<'a> {
    /// new up a FIndAllPackages instance.
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    /// * An instance of FndAllPackages
    pub fn new() -> Self {
        FindAllPackages {
            name: None
            //order_by: Vec::new(),
            // order_direction: None,
            // limit: None,
        }
    }

    /// Set teh name fo the platform and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the platform
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Set an optional name for the platform.
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
    ///
    /// * `client` - A mutable reference to a Client instance
    ///
    /// # Returns
    /// * an Ok wrapped Vector of FindAllPackagesRow or an Error wrapped Box dyn Error
    pub async fn query(
        &mut self,
        client: &Client,
    ) -> FindAllPackagesResult<Vec<FindAllPackagesRow>> {
        let mut op = "=";
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let name = self.name.unwrap_or("any");

        let mut query_str = "SELECT 
                name
            FROM 
                package"
            .to_string();
        if self.name.is_some() {
            if name.contains("%") {
                op = "LIKE";
            }
            params.push(&name);
            query_str = format!("{} WHERE name {} $1", query_str, op);
        }
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
        log::info!("Arguments\n{:?}", &params);
        for row in client
            .query(query_str.as_str(), &params[..])
            .await
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
