pub use crate::coords_error::{CoordsError, CoordsResult};
pub use crate::db::search_attribute::{LtreeSearchMode, OrderDirection, SearchAttribute};
use crate::types::IdType;
pub use crate::Coords;
pub use crate::Distribution;
use log;
use serde::Serialize;
use snafu::{ResultExt, Snafu};
use std::fmt;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderRoleBy {
    #[strum(
        serialize = "name",
        serialize = "Name",
        serialize = "NAME",
        to_string = "name"
    )]
    Name,
    #[strum(
        serialize = "category",
        serialize = "Category",
        serialize = "CATEGORY",
        to_string = "category"
    )]
    Category,
}
/// Valid categories that one may search
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum Categories {
    #[strum(
        serialize = "role",
        serialize = "Role",
        serialize = "ROLE",
        to_string = "role"
    )]
    Role,
    #[strum(
        serialize = "subrole",
        serialize = "Subrole",
        serialize = "SUBROLE",
        to_string = "subrole"
    )]
    Subrole,
    #[strum(
        serialize = "any",
        serialize = "Any",
        serialize = "ANY",
        to_string = "any"
    )]
    Any,
}

pub type FindAllRolesResult<T, E = FindAllRolesError> = std::result::Result<T, E>;

/// Error type returned from  FindAllRolesError
#[derive(Debug, Snafu)]
pub enum FindAllRolesError {
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

/// A row returned from the  FindAllRoles.query
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FindAllRolesRow {
    pub role: String,
    pub category: String,
}

impl fmt::Display for FindAllRolesRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  ({})", self.role, self.category)
    }
}

impl FindAllRolesRow {
    /// New up a  FindAllRolesRow instance
    ///
    /// # Arguments
    /// * `role`  - the role name
    /// * `category` - the category name
    ///
    /// # Returns
    /// - FindAllRolesRow instance
    pub fn new(role: String, category: String) -> Self {
        FindAllRolesRow { role, category }
    }
    /// Attempt to construct a distribution from &strs. This is a fallible operation
    /// returning a result.
    ///
    /// # Arguments
    ///
    /// * `role`
    /// * `category`
    ///
    /// # Returns
    /// Result
    /// - Ok - FindAllRolesRow instance
    /// - Err - FindAllRolesError
    pub fn try_from_parts(role: &str, category: &str) -> FindAllRolesResult<FindAllRolesRow> {
        // TODO: police category
        Ok(Self::new(role.to_string(), category.to_string()))
    }

    /// Infallible counterpart to try_from_parts. Will panic if there is a problem
    ///
    /// # Arguments
    /// * `role`
    /// * `category`
    ///
    /// # Returns
    /// - FindAllRolesRow instance
    pub fn from_parts(role: &str, category: &str) -> FindAllRolesRow {
        Self::try_from_parts(role, category).unwrap()
    }
}
/// Responsible for finding a distribution
pub struct FindAllRoles<'a> {
    role: Option<&'a str>,
    category: Option<&'a str>,
    order_by: Option<Vec<OrderRoleBy>>,
    order_direction: Option<OrderDirection>,
    limit: Option<IdType>,
}

impl fmt::Debug for FindAllRoles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FindAllRoles({:?} {:?})", self.role, self.category)
    }
}

impl<'a> FindAllRoles<'a> {
    /// new up a FIndAllRoles instance.
    ///
    /// # Arguments
    ///
    /// * `client` - A mutable reference to a Client instance
    ///
    /// # Returns
    ///
    /// * A FindAllRoles instance
    pub fn new() -> Self {
        FindAllRoles {
            category: None,
            role: None,
            order_by: None,
            order_direction: None,
            limit: None,
        }
    }

    /// Set the role name and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `role_n` - The name of the role
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn role(&mut self, role_n: &'a str) -> &mut Self {
        self.role = Some(role_n);
        self
    }

    /// Set the category name and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `category_n` - The name of the category for the level
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn category(&mut self, category_n: &'a str) -> &mut Self {
        self.category = Some(category_n);
        self
    }

    /// Set an optional category.
    ///
    /// This is generally accomplished by calling
    /// ```ignore
    /// .as_ref().map(Deref::deref)
    /// ```
    /// on an `Option<String>` to convert it into an `Option<&str>`
    pub fn category_opt(&mut self, category: Option<&'a str>) -> &mut Self {
        self.category = category;
        self
    }

    /// Set an optional role
    ///
    /// # Arguments
    ///
    /// * `role` - An option wrapping the name of the role
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn role_opt(&mut self, role: Option<&'a str>) -> &mut Self {
        self.role = role;
        self
    }

    /// Set the column or columns to order the returned values by and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `attributes` - A vector of OrderRoleBy instances
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn order_by(&mut self, attributes: Vec<OrderRoleBy>) -> &mut Self {
        self.order_by = Some(attributes);
        self
    }

    /// Set the Optional column or columns to order the returned values by and return a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `attributes` - A vector of OrderRoleBy instances wrapped in Option
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn order_by_opt(&mut self, attributes: Option<Vec<OrderRoleBy>>) -> &mut Self {
        self.order_by = attributes;
        self
    }
    /// Set the direction of the query sort. Used in conjunction with `order_by`. Returns
    /// a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `direction` - A variant of teh `OrderDirection` enum
    ///
    /// # Returns
    ///
    /// * Mutable reference to Self
    pub fn order_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.order_direction = Some(direction);
        self
    }

    /// Set the optional direction of the query sort. Used in conjunction with `order_by`. Returns
    /// a mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `direction` - A variant of the `OrderDirection` enum wrapped in Option
    ///
    /// # Returns
    ///
    /// * Mutable reference to Self
    pub fn order_direction_opt(&mut self, direction: Option<OrderDirection>) -> &mut Self {
        self.order_direction = direction;
        self
    }

    /// Set the maximum number of results that the query will return and return a
    /// mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of returned roles
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn limit(&mut self, limit: IdType) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Set the optional maximum number of results that the query will return and return a
    /// mutable reference to Self
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of returned roles wrapped in an Option
    ///
    /// # Returns
    ///
    /// * A mutable reference to Self
    pub fn limit_opt(&mut self, limit: Option<IdType>) -> &mut Self {
        self.limit = limit;
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
    /// * A future wrapping a Result returning a Vector of FindAllRolesRow if ok, or
    /// a FindAllRolesError if in error
    pub async fn query(&mut self, client: &Client) -> FindAllRolesResult<Vec<FindAllRolesRow>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut query_str = "SELECT DISTINCT 
                name,
                category
            FROM 
                role_view WHERE name <> 'any'"
            .to_string();
        let category = self.category.unwrap_or("role");
        if self.category.is_some() {
            let categories = Categories::from_str(category);
            if categories.is_err() {
                //TODO: have function return custom error
                log::error!("category specificed is not valid: {}", category);
            } else if category != "any" {
                query_str = format!("{} AND category = $1", query_str);
                params.push(&category);
            }
        }

        //let order_by = self.order_by.as_ref().unwrap_or(&vec![OrderRoleBy::Name]);

        if let Some(ref orderby) = self.order_by {
            let orderby = orderby.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
            query_str = format!("{} ORDER BY {}", query_str, orderby.join(","));
        } else {
            query_str = format!("{}  ORDER BY name", query_str);
        }
        let mut result = Vec::new();
        log::info!("SQL\n{}", query_str.as_str());
        log::info!("Arguments\n{:?}", &params);
        for row in client
            .query(query_str.as_str(), &params[..])
            .await
            .context(TokioPostgresError {
                msg: "problem with select from role_view",
            })?
        {
            let role_name = row.get(0);
            let category = row.get(1);
            result.push(FindAllRolesRow::try_from_parts(role_name, category)?);
        }
        Ok(result)
    }
}
