pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::IdType;
pub use crate::Coords;
pub use crate::Distribution;
use log;
use snafu::Snafu;
use std::fmt;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

pub type FindAllWithsResult<T, E = FindAllWithsError> = std::result::Result<T, E>;

/// Error type returned from  FindAllWithsError
#[derive(Debug, Snafu)]
pub enum FindAllWithsError {
    ///  WithNewError - failure to new up a With.
    #[snafu(display("Error constructing With from {}: {}", msg, source))]
    WithNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllWiths.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllWithsRow {
    /// the id of result in the With table
    pub id: IdType,
    pub vpin_id: IdType,
    pub with: String,
    pub order: IdType,
}

impl fmt::Display for FindAllWithsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} order: {} (id:{} vpin id:{} )",
            self.with, self.order, self.id, self.vpin_id,
        )
    }
}

impl FindAllWithsRow {
    /// New up a  FindAllWithsRow instance
    ///
    /// # Arguments
    /// * `id`: The id of the relevant row in the with table
    /// * `vpin_id`: The versionpin id that the with belongs to
    /// * `with`: The actual with package
    /// * `order`: The order of the with package in the list of with packages
    pub fn new(id: IdType, vpin_id: IdType, with: String, order: IdType) -> Self {
        FindAllWithsRow {
            id,
            vpin_id,
            with,
            order,
        }
    }
    /// Try to attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    /// * `id`: The id of the relevant row in the with table
    /// * `vpin_id`: The versionpin id that the with belongs to
    /// * `with`: The actual with package
    /// * `order`: The order of the with package in the list of with packages
    pub fn try_from_parts(
        id: IdType,
        vpin_id: IdType,
        with: String,
        order: IdType,
    ) -> FindAllWithsResult<FindAllWithsRow> {
        Ok(Self::new(id, vpin_id, with, order))
    }

    /// Try to attempt to construct a distribution from &strs. This is an infallible operation
    /// and will panic if it fails
    ///
    /// # Arguments
    /// * `id`: The id of the relevant row in the with table
    /// * `vpin_id`: The versionpin id that the with belongs to
    /// * `with`: The actual with package
    /// * `order`: The order of the with package in the list of with packages
    pub fn from_parts(id: IdType, vpin_id: IdType, with: String, order: IdType) -> FindAllWithsRow {
        Self::new(id, vpin_id, with, order)
    }
}
/// Responsible for finding a distribution
pub struct FindAllWiths<'a> {
    client: &'a mut Client,
    vpin_id: IdType,
}

impl<'a> FindAllWiths<'a> {
    pub fn new(client: &'a mut Client, vpin_id: IdType) -> Self {
        FindAllWiths { client, vpin_id }
    }

    pub async fn query(&mut self) -> Result<Vec<FindAllWithsRow>, Box<dyn std::error::Error>> {
        let query_str = "SELECT id, versionpin, package, pinorder
        FROM withpackage WHERE versionpin = $1 ORDER BY pinorder"
            .to_string();
        let mut result = Vec::new();
        let qstr = query_str.as_str();
        let prepared_args: &[&(dyn ToSql + std::marker::Sync)] = &[&self.vpin_id];
        log::info!("SQL\n{}", qstr);
        log::info!("Arguents\n{:?}", prepared_args);
        for row in self.client.query(qstr, prepared_args).await? {
            let id: IdType = row.get(0);
            let vpin_id: IdType = row.get(1);
            let with: String = row.get(2);
            let order: IdType = row.get(3);
            result.push(FindAllWithsRow::try_from_parts(id, vpin_id, with, order)?);
        }
        Ok(result)
    }
}
