pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::{IdType, LongIdType};
pub use crate::Coords;
pub use crate::Distribution;
use crate::{Level, Platform, Role, Site};
use log;
use snafu::ResultExt;
use snafu::Snafu;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use strum::ParseError;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

/// Logged activity in audit log may be one of
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr, Clone,
)]
pub enum ChangeAction {
    #[strum(
        serialize = "insert",
        serialize = "Insert",
        serialize = "INSERT",
        to_string = "INSERT"
    )]
    Insert,
    #[strum(
        serialize = "update",
        serialize = "Update",
        serialize = "UPDATE",
        to_string = "UPDATE"
    )]
    Update,
    #[strum(
        serialize = "truncate",
        serialize = "TRUNCATE",
        serialize = "TRUNCATE",
        to_string = "TRUNCATE"
    )]
    Truncate,
}
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
    #[snafu(display("Error creating ChangeAction from '{}': {}", input, source))]
    ChangeActionError { input: String, source: ParseError },
    #[snafu(display("transaction_id not set"))]
    TransactionIdMissingError,
    /// Error from postgres
    #[snafu(display("Postgres Error: {} {}", msg, source))]
    TokioPostgresError {
        msg: &'static str,
        source: tokio_postgres::error::Error,
    },
}

/// A row returned from the  FindAllChanges.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllChangesRow {
    pub id: LongIdType,
    pub transaction_id: LongIdType,
    pub action: ChangeAction,
    pub level: Level,
    pub role: Role,
    pub platform: Platform,
    pub site: Site,
    pub package: String,
    // when I start to track updates to with's this will have to
    // be an enum ChangeType {Distribution(Distribution), Withs(Vec<String>)}
    pub old: Option<Distribution>, // should this be a distribution?
    pub new: Distribution,
}

impl fmt::Display for FindAllChangesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} ({} {} {} {}) {} {}",
            self.id,
            self.transaction_id,
            self.action,
            self.level,
            self.role,
            self.platform,
            self.site,
            self.old
                .as_ref()
                .map(|x| format!("{}", x))
                .unwrap_or(String::new()),
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
        action: ChangeAction,
        level: Level,
        role: Role,
        platform: Platform,
        site: Site,
        package: S,
        old: Option<Distribution>,
        new: Distribution,
    ) -> Self {
        FindAllChangesRow {
            id,
            transaction_id,
            action,
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
        action: &'b str,
        level: &'b str,
        role: &'b str,
        platform: &'b str,
        site: &'b str,
        package: &'b str,
        old: &'b str,
        new: &'b str,
    ) -> FindAllChangesResult<FindAllChangesRow> {
        // TODO: police category
        let action = ChangeAction::from_str(action).context(ChangeActionError {
            input: action.to_string(),
        })?;
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
        let old = if old == "" {
            None
        } else {
            Some(
                Distribution::try_from(old).context(CoordsTryFromPartsError {
                    coords: "unable to create from supplied str",
                })?,
            )
        };

        let new = Distribution::try_from(new).context(CoordsTryFromPartsError {
            coords: "unable to create from supplied str",
        })?;
        Ok(Self::new(
            id,
            transaction_id,
            action,
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
        action: &'b str,
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
            action,
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

    /// Set the transaction id and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `txid` - The transaction id for the revision which we update the internal one to.
    ///
    /// # Returns
    ///
    /// * Mutable reference to Self
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

    /// Invoke the database query and return a result
    ///
    /// # Arguments
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// * A future wrapping a Result returning a Vector of FindAllChangesRow if ok, or
    /// a FindAllCHangeError if in error
    pub async fn query(&mut self) -> FindAllChangesResult<Vec<FindAllChangesRow>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let query_str = "SELECT
                id,
                transaction_id,
                action,
                level_name,
                role_name,
                platform_name,
                site_name,
                package,
                old,
                new
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
        for row in self
            .client
            .query(query_str.as_str(), &params[..])
            .await
            .context(TokioPostgresError {
                msg: "problem with select from find_vpin_audit  function",
            })?
        {
            let id: IdType = row.get(0);
            let txid: LongIdType = row.get(1);
            let action: &str = row.get(2);
            let level: &str = row.get(3);
            let role: &str = row.get(4);
            let platform: &str = row.get(5);
            let site: &str = row.get(6);
            let package: &str = row.get(7);
            let old: &str = row.get(8);
            let new: &str = row.get(9);

            result.push(FindAllChangesRow::try_from_parts(
                id as LongIdType,
                txid,
                action,
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
