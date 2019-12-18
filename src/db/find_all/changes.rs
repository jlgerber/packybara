pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::{IdType, LongIdType};
pub use crate::Coords;
pub use crate::Distribution;
use crate::{Level, Platform, Role, Site};
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::ResultExt;
use snafu::Snafu;
use std::convert::TryFrom;
use std::fmt;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderChangeBy {
    #[strum(
        serialize = "transaction_id",
        serialize = "TransactionId",
        serialize = "TRANSACTION_ID",
        to_string = "transaction_id"
    )]
    TransactionId,
}

pub type FindAllChangesResult<T, E = FindAllChangesError> = std::result::Result<T, E>;

/// Error type returned from  FindAllChangesError
#[derive(Debug, Snafu)]
pub enum FindAllChangesError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
    #[snafu(display("transaction_id not set"))]
    TransactionIdMissingError,
}

/// A row returned from the  FindAllChanges.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllChangesRow {
    pub id: LongIdType,
    pub transaction_id: LongIdType,
    pub level: Level,
    pub role: Role,
    pub platform: Platform,
    pub site: Site,
    pub package: String,
    // when I start to track updates to with's this will have to
    // be an enum ChangeType {Distribution(Distribution), Withs(Vec<String>)}
    pub old: Distribution, // should this be a distribution?
    pub new: Distribution,
}

impl fmt::Display for FindAllChangesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({} {} {} {}) {} {}",
            self.id,
            self.transaction_id,
            self.level,
            self.role,
            self.platform,
            self.site,
            self.old,
            self.new
        )
    }
}

impl FindAllChangesRow {
    /// New up a  FindAllChangesRow instance
    ///
    /// # Arguments
    /// * `id`  - the revision id
    /// * `transaction_id` - the transaction id
    /// * `author` - The author of the revision
    /// * `comment` - The comment associated with the revision
    ///
    /// # Returns
    /// - FindAllChangesRow instance
    pub fn new<S: Into<String>>(
        id: LongIdType,
        transaction_id: LongIdType,
        level: Level,
        role: Role,
        platform: Platform,
        site: Site,
        package: S,
        old: Distribution,
        new: Distribution,
    ) -> Self {
        FindAllChangesRow {
            id,
            transaction_id,
            level,
            role,
            platform,
            site,
            package: package.into(),
            old,
            new,
        }
    }
    /// Attempt to construct a revision from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `id`
    /// * `transaction_id`
    /// * `author`
    /// * `datetime`
    /// * `comment`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllChangesRow instance
    /// - Err - FindAllChangesError
    pub fn try_from_parts<'b>(
        id: LongIdType,
        transaction_id: LongIdType,
        level: &'b str,
        role: &'b str,
        platform: &'b str,
        site: &'b str,
        package: &'b str,
        old: &'b str,
        new: &'b str,
    ) -> FindAllChangesResult<FindAllChangesRow> {
        // TODO: police category
        let level = Level::try_from(level).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        let role = Role::try_from(role).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        let platform = Platform::try_from(platform).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        let site = Site::try_from(site).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        let old = Distribution::try_from(old).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        let new = Distribution::try_from(new).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        Ok(Self::new(
            id,
            transaction_id,
            level,
            role,
            platform,
            site,
            package,
            old,
            new,
        ))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `id`
    /// * `transaction_id`
    /// * `author`
    /// * `datetime`
    /// * `comment`
    ///
    /// # Returns
    /// - FindAllChangesRow instance
    pub fn from_parts<'b>(
        id: LongIdType,
        transaction_id: LongIdType,
        level: &'b str,
        role: &'b str,
        platform: &'b str,
        site: &'b str,
        package: &'b str,
        old: &'b str,
        new: &'b str,
    ) -> FindAllChangesRow {
        Self::try_from_parts(
            id,
            transaction_id,
            level,
            role,
            platform,
            site,
            package,
            old,
            new,
        )
        .expect("unable to create changes row")
    }
}
/// Responsible for finding a distribution
pub struct FindAllChanges<'a> {
    client: &'a mut Client,
    transaction_id: Option<LongIdType>,
}

impl fmt::Debug for FindAllChanges<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllChanges( txid:{:?})", self.transaction_id)
    }
}

impl<'a> FindAllChanges<'a> {
    /// new up a FIndAllChanges instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindAllChanges {
            client,
            transaction_id: None,
        }
    }

    pub fn transaction_id(&mut self, txid: LongIdType) -> &mut Self {
        self.transaction_id = Some(txid);
        self
    }
    /// Set an optional id.
    ///
    /// # Arguments
    /// * `txid` - optional id
    ///
    /// # Returns
    /// Mutable reference to Self
    pub fn transaction_id_opt(&mut self, txid: Option<LongIdType>) -> &mut Self {
        self.transaction_id = txid;
        self
    }

    pub fn query(&mut self) -> Result<Vec<FindAllChangesRow>, Box<dyn std::error::Error>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let query_str = "SELECT 
                id, transaction_id, level_name, role_name, platform_name, site_name, package, old, new
            FROM 
            find_vpin_audit($1)"
            .to_string();
        if self.transaction_id.is_none() {
            return Err(FindAllChangesError::TransactionIdMissingError)?;
        }

        let transaction_id = self.transaction_id.unwrap();
        params.push(&transaction_id);
        let mut result = Vec::new();

        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Prepared: {:?}", &params);
        for row in self.client.query(query_str.as_str(), &params[..])? {
            let id: IdType = row.get(0);
            let txid: LongIdType = row.get(1);
            let level: &str = row.get(2);
            let role: &str = row.get(3);
            let platform: &str = row.get(4);
            let site: &str = row.get(5);
            let package: &str = row.get(6);
            let old: &str = row.get(7);
            let new: &str = row.get(8);

            result.push(FindAllChangesRow::try_from_parts(
                id as LongIdType,
                txid,
                level,
                role,
                platform,
                site,
                package,
                old,
                new,
            )?);
        }
        Ok(result)
    }
}
