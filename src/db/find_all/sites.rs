pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use tokio_postgres::types::ToSql;

use serde::Serialize;
use snafu::{ResultExt, Snafu};
use std::fmt;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use tokio_postgres::Client;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderSiteBy {
    #[strum(
        serialize = "name",
        serialize = "Name",
        serialize = "NAME",
        to_string = "name"
    )]
    Name,
}

pub type FindAllSitesResult<T, E = FindAllSitesError> = std::result::Result<T, E>;

/// Error type returned from  FindAllSitesError
#[derive(Debug, Snafu)]
pub enum FindAllSitesError {
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

/// A row returned from the  FindAllSites.query
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FindAllSitesRow {
    pub name: String,
}

impl fmt::Display for FindAllSitesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FindAllSitesRow {
    /// New up a  FindAllSitesRow instance
    ///
    /// # Arguments
    ///
    /// * `name`  - the site name
    ///
    /// # Returns
    ///
    /// * FindAllSitesRow instance
    pub fn new(name: String) -> Self {
        FindAllSitesRow { name }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `site`
    ///
    /// # Returns
    ///
    /// * Result
    /// - Ok - FindAllSitesRow instance
    /// - Err - FindAllSitesError
    pub fn try_from_parts(site: &str) -> FindAllSitesResult<FindAllSitesRow> {
        // TODO: police category
        Ok(Self::new(site.to_string()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    ///
    /// * `role`
    /// * `category`
    ///
    /// # Returns
    ///
    /// * FindAllSitesRow instance
    pub fn from_parts(site: &str) -> FindAllSitesRow {
        Self::try_from_parts(site).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllSites<'a> {
    /// The client is used to interface with the database
    /// An optional name of the site to query for
    name: Option<&'a str>,
}

impl fmt::Debug for FindAllSites<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllSites({:?})", self.name)
    }
}

impl<'a> FindAllSites<'a> {
    /// new up a FindAllSites instance.
    ///
    /// # Arguments
    ///
    /// * None
    pub fn new() -> Self {
        FindAllSites { name: None }
    }

    /// Set teh name fo the site and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the site
    ///
    /// # Returns
    ///
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

    /// Invoke the database query and return a result
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to a Client instance
    ///
    /// # Returns
    ///
    /// * A future wrapping a Result returning a Vector of FindAllSitesRow if ok, or
    /// a FindAllSitesError if in error
    pub async fn query(&mut self, client: &Client) -> FindAllSitesResult<Vec<FindAllSitesRow>> {
        //let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut op = "=";
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT DISTINCT 
                name
            FROM 
                site_view WHERE name <> 'any'"
            .to_string();

        let name = self.name.unwrap_or("any");
        if self.name.is_some() {
            if name.contains("%") {
                op = "LIKE";
            }
            params.push(&name);
            query_str = format!("{} AND name {} $1", query_str, op);
        }
        query_str = format!("{}  ORDER BY name", query_str);
        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Arguments:\n{:?}", &params);
        let mut result = Vec::new();
        for row in client
            .query(query_str.as_str(), &params[..])
            .await
            .context(TokioPostgresError {
                msg: "problem with select from site_view",
            })?
        {
            let name = row.get(0);
            result.push(FindAllSitesRow::try_from_parts(name)?);
        }
        Ok(result)
    }
}
