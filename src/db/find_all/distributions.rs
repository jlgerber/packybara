pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::IdType;
pub use crate::Coords;
pub use crate::Distribution;
use log;
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use std::fmt;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

//use std::str::FromStr;
//use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
// pub enum OrderDistributionBy {
//     #[strum(
//         serialize = "name",
//         serialize = "Name",
//         serialize = "NAME",
//         to_string = "name"
//     )]
//     Name,
// }

pub type FindAllDistributionsResult<T, E = FindAllDistributionsError> = std::result::Result<T, E>;

/// Error type returned from  FindAllDistributionsError
#[derive(Debug, Snafu)]
pub enum FindAllDistributionsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
    #[snafu(display(
        "provided distribution not parseable. try_from called with {}",
        distribution,
    ))]
    DistributionError { distribution: String },
    /// Error from postgres
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
}

/// A row returned from the  FindAllDistributions.query
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FindAllDistributionsRow {
    pub id: IdType,
    pub package: String,
    pub version: String,
}

impl fmt::Display for FindAllDistributionsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.package, self.version)
    }
}

impl FindAllDistributionsRow {
    /// New up a  FindAllDistributionsRow instance
    ///
    /// # Arguments
    ///
    /// * `name`  - the name name
    ///
    /// # Returns
    ///
    /// - FindAllDistributionsRow instance
    pub fn new(id: IdType, package: String, version: String) -> Self {
        FindAllDistributionsRow {
            id,
            package,
            version,
        }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `name`
    ///
    /// # Returns
    ///
    /// * Result
    /// - Ok - FindAllDistributionsRow instance
    /// - Err - FindAllDistributionsError
    pub fn try_from_parts(
        id: IdType,
        package: &str,
        version: &str,
    ) -> FindAllDistributionsResult<FindAllDistributionsRow> {
        // TODO: police category
        Ok(Self::new(id, package.to_string(), version.to_string()))
    }
    pub fn try_from(
        id: IdType,
        distribution: &str,
    ) -> FindAllDistributionsResult<FindAllDistributionsRow> {
        // TODO: police category
        let pieces = distribution.split("-").collect::<Vec<_>>(); //FIX
        if pieces.len() != 2 {
            return Err(FindAllDistributionsError::DistributionError {
                distribution: distribution.to_string(),
            })?;
        }
        Ok(Self::new(id, pieces[0].to_string(), pieces[1].to_string()))
    }
    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `name`
    ///
    /// # Returns
    /// - FindAllDistributionsRow instance
    pub fn from_parts(id: IdType, package: &str, version: &str) -> FindAllDistributionsRow {
        Self::try_from_parts(id, package, version).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllDistributions<'a> {
    client: &'a mut Client,
    package: Option<&'a str>,
    version: Option<&'a str>,
    //order_by: Vec<OrderDistributionBy>,
    order_direction: Option<OrderDirection>,
    // limit: Option<IdType>,
}

impl fmt::Debug for FindAllDistributions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FindAllDistributions({:?}-{:?})",
            self.package, self.version
        )
    }
}

impl<'a> FindAllDistributions<'a> {
    /// new up a FIndAllDistributions instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindAllDistributions {
            client,
            package: None,
            version: None,
            //order_by: Vec::new(),
            order_direction: None,
            // limit: None,
        }
    }

    pub fn package(&mut self, package: &'a str) -> &mut Self {
        self.package = Some(package);
        self
    }

    pub fn version(&mut self, version: &'a str) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn package_opt(&mut self, package: Option<&'a str>) -> &mut Self {
        self.package = package;
        self
    }

    pub fn version_opt(&mut self, version: Option<&'a str>) -> &mut Self {
        self.version = version;
        self
    }
    // pub fn order_by(&mut self, attributes: Vec<OrderDistributionBy>) -> &mut Self {
    //     self.order_by = attributes;
    //     self
    // }
    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }
    pub fn order_direction_opt(&mut self, direction: Option<OrderDirection>) -> &mut Self {
        self.order_direction = direction;
        self
    }

    // pub fn limit(&mut self, limit: IdType) -> &mut Self {
    //     self.limit = Some(limit);
    //     self
    // }

    pub async fn query(&mut self) -> FindAllDistributionsResult<Vec<FindAllDistributionsRow>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT 
                distribution_id,
                package,
                version_name
            FROM 
                distribution_view"
            .to_string();
        let mut cnt: i32 = 1;
        let package = self.package.unwrap_or("none");
        let version = self.version.unwrap_or("0");
        let direction = self
            .order_direction
            .as_ref()
            .unwrap_or(&OrderDirection::Desc);
        if self.package.is_some() {
            let q = if package.contains("%") { "like" } else { "=" };
            query_str = format!("{} WHERE package {} ${}", query_str, q, cnt);
            cnt += 1;
            params.push(&package);
        }
        if self.version.is_some() {
            let q = if version.contains("%") { "like" } else { "=" };

            if cnt == 1 {
                query_str = format!("{} WHERE version_name {} $1", query_str, q);
            } else {
                query_str = format!("{} AND version_name {} $2", query_str, q);
            }
            params.push(&version);
        }

        query_str = format!(
            "{}  ORDER BY package {}, version {}",
            query_str, direction, direction
        );
        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Arguments\n{:?}", &params);
        for row in self
            .client
            .query(query_str.as_str(), &params[..])
            .await
            .context(TokioPostgresError {
                msg: "problem with select from distribution_view",
            })?
        {
            let id = row.get(0);
            let package = row.get(1);
            let version = row.get(2);
            result.push(FindAllDistributionsRow::try_from_parts(
                id, package, version,
            )?);
        }
        Ok(result)
    }
}
