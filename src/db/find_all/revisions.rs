pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::{IdType, LongIdType};
pub use crate::Coords;
pub use crate::Distribution;
use log;
use postgres::types::ToSql;
use postgres::Client;
use snafu::Snafu;
use std::fmt;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderRevisionBy {
    #[strum(serialize = "id", serialize = "Id", serialize = "ID", to_string = "id")]
    Id,
    #[strum(
        serialize = "author",
        serialize = "Author",
        serialize = "AUTHOR",
        to_string = "author"
    )]
    Author,
}

pub type FindAllRevisionsResult<T, E = FindAllRevisionsError> = std::result::Result<T, E>;

/// Error type returned from  FindAllRevisionsError
#[derive(Debug, Snafu)]
pub enum FindAllRevisionsError {
    ///  DistributionNewError - failure to new up a distribution.
    #[snafu(display("Error constructing Distribution from {}: {}", msg, source))]
    DistributionNewError { msg: String, source: CoordsError },
    /// CoordsTryFromPartsError - error when calling try_from_parts
    #[snafu(display("Error calling Coords::try_from_parts with {}: {}", coords, source))]
    CoordsTryFromPartsError { coords: String, source: CoordsError },
}

/// A row returned from the  FindAllRevisions.query
#[derive(Debug, PartialEq, Eq)]
pub struct FindAllRevisionsRow {
    pub id: IdType,
    pub transaction_id: LongIdType,
    pub author: String,
    pub comment: String,
}

impl fmt::Display for FindAllRevisionsRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.id, self.transaction_id, self.author, self.comment
        )
    }
}

impl FindAllRevisionsRow {
    /// New up a  FindAllRevisionsRow instance
    ///
    /// # Arguments
    /// * `id`  - the revision id
    /// * `transaction_id` - the transaction id
    /// * `author` - The author of the revision
    /// * `comment` - The comment associated with the revision
    ///
    /// # Returns
    /// - FindAllRevisionsRow instance
    pub fn new<S: Into<String>>(
        id: IdType,
        transaction_id: LongIdType,
        author: S,
        comment: S,
    ) -> Self {
        FindAllRevisionsRow {
            id,
            transaction_id,
            author: author.into(),
            comment: comment.into(),
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
    /// * `comment`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllRevisionsRow instance
    /// - Err - FindAllRevisionsError
    pub fn try_from_parts<S: Into<String>>(
        id: IdType,
        transaction_id: LongIdType,
        author: S,
        comment: S,
    ) -> FindAllRevisionsResult<FindAllRevisionsRow> {
        // TODO: police category
        Ok(Self::new(id, transaction_id, author.into(), comment.into()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `id`
    /// * `transaction_id`
    /// * `author`
    /// * `comment`
    ///
    /// # Returns
    /// - FindAllRevisionsRow instance
    pub fn from_parts<S: Into<String>>(
        id: IdType,
        transaction_id: LongIdType,
        author: S,
        comment: S,
    ) -> FindAllRevisionsRow {
        Self::try_from_parts(id, transaction_id, author.into(), comment.into()).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllRevisions<'a> {
    client: &'a mut Client,
    id: Option<IdType>,
    transaction_id: Option<LongIdType>,
    author: Option<&'a str>,
    order_by: Option<Vec<OrderRevisionBy>>,
    order_direction: Option<OrderDirection>,
    limit: Option<IdType>,
}

impl fmt::Debug for FindAllRevisions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllRevisions(id:{:?} txid:{:?} author:{:?} order by:{:?} direction:{:?} limit:{:?})", 
        self.id, self.transaction_id, self.author, self.order_by, self.order_direction, self.limit)
    }
}

impl<'a> FindAllRevisions<'a> {
    /// new up a FIndAllRevisions instance.
    pub fn new(client: &'a mut Client) -> Self {
        FindAllRevisions {
            client,
            id: None,
            transaction_id: None,
            author: None,
            order_by: None,
            order_direction: None,
            limit: None,
        }
    }

    pub fn id(&mut self, id: IdType) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn transaction_id(&mut self, txid: LongIdType) -> &mut Self {
        self.transaction_id = Some(txid);
        self
    }
    /// Set the author.
    ///
    /// # Arguments
    /// * `author`
    pub fn author(&mut self, author: &'a str) -> &mut Self {
        self.author = Some(author);
        self
    }

    /// Set an optional id.
    ///
    /// # Arguments
    /// * `id` - optional id
    ///
    /// # Returns
    /// Mutable reference to Self
    pub fn id_opt(&mut self, id: Option<IdType>) -> &mut Self {
        self.id = id;
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

    /// Set an optional author.
    ///
    /// # Arguments
    /// * `author` - optional author str
    ///
    /// # Returns
    /// Mutable reference to Self
    pub fn author_opt(&mut self, author: Option<&'a str>) -> &mut Self {
        self.author = author;
        self
    }

    /// Set ordering.
    ///
    /// # Arguments
    /// * `order_by` - vectro of OrderRevisionBy attributes
    ///
    /// # Returns
    /// Mutable reference to Self
    pub fn order_by(&mut self, attributes: Vec<OrderRevisionBy>) -> &mut Self {
        self.order_by = Some(attributes);
        self
    }
    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }

    pub fn order_direction_opt(&mut self, direction: Option<OrderDirection>) -> &mut Self {
        self.order_direction = direction;
        self
    }

    pub fn limit(&mut self, limit: IdType) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn limit_opt(&mut self, limit: Option<IdType>) -> &mut Self {
        self.limit = limit;
        self
    }
    pub fn query(&mut self) -> Result<Vec<FindAllRevisionsRow>, Box<dyn std::error::Error>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT 
                id, transaction_id, author, comment
            FROM 
                revision_view"
            .to_string();
        let mut where_and = "WHERE";
        let mut prep_id = 1;
        if let Some(ref id) = self.id {
            query_str = format!(" {} id = ${}", where_and, prep_id);
            where_and = "AND";
            prep_id += 1;
            params.push(id);
        }

        if let Some(ref transaction_id) = self.transaction_id {
            query_str = format!(" {} transaction_id = ${}", where_and, prep_id);
            where_and = "AND";
            prep_id += 1;
            params.push(transaction_id);
        }

        if let Some(ref author) = self.author {
            query_str = format!(" {} author = ${}", where_and, prep_id);
            //where_and = "AND";
            //prep_id += 1;
            params.push(author);
        }

        let direction = match self.order_direction {
            Some(ref dir) => dir.as_ref(),
            None => "ASC",
        };

        if let Some(ref orderby) = self.order_by {
            if orderby.len() > 0 {
                let orderby = orderby
                    .iter()
                    .map(|x| format!("{} {}", x.as_ref(), direction))
                    .collect::<Vec<_>>();
                query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
            }
        }

        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        //log::info!("Prepared: {:?}", &params);
        for row in self.client.query(query_str.as_str(), &params[..])? {
            let id: IdType = row.get(0);
            let txid: LongIdType = row.get(1);
            let author: &str = row.get(2);
            let comment: &str = row.get(3);
            result.push(FindAllRevisionsRow::try_from_parts(
                id, txid, author, comment,
            )?);
        }
        Ok(result)
    }
}
