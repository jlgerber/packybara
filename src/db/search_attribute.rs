//use snafu::ResultExt;
use snafu::Snafu;

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Provide a measure of extensibility
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum SearchCtrl {
    OrderBy(SearchAttribute),
    OrderByMany(Vec<SearchAttribute>),
    OrderAsc,
    OrderDesc,
    Limit(u32),
}

/// Direction in which to order by when using OrderBy
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum OrderDirection {
    #[strum(
        serialize = "asc",
        serialize = "Asc",
        serialize = "ASC",
        to_string = "ASC"
    )]
    Asc,
    #[strum(
        serialize = "desc",
        serialize = "Desc",
        serialize = "DESC",
        to_string = "DESC"
    )]
    Desc,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
/// Attributes that one may search and sort on.
pub enum SearchAttribute {
    #[strum(
        serialize = "package",
        serialize = "Package",
        serialize = "PACKAGE",
        to_string = "package"
    )]
    Package,
    #[strum(
        serialize = "level",
        serialize = "Level",
        serialize = "LEVEL",
        to_string = "level"
    )]
    Level,
    #[strum(
        serialize = "role",
        serialize = "Role",
        serialize = "ROLE",
        to_string = "role"
    )]
    Role,
    #[strum(
        serialize = "platform",
        serialize = "Platform",
        serialize = "PLATFORM",
        to_string = "platform"
    )]
    Platform,
    #[strum(
        serialize = "site",
        serialize = "Site",
        serialize = "SITE",
        to_string = "site"
    )]
    Site,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumString, AsRefStr, Display, IntoStaticStr)]
pub enum LtreeSearchMode {
    #[strum(
        serialize = "down",
        serialize = "ancestor",
        serialize = "Ancestor",
        serialize = "ANCESTOR",
        to_string = "ancestor"
    )]
    Ancestor,
    #[strum(
        serialize = "up",
        serialize = "descendant",
        serialize = "Descendant",
        serialize = "DESCENDANT",
        to_string = "descendant"
    )]
    Descendant,
    #[strum(
        serialize = "exact",
        serialize = "Exact",
        serialize = "EXACT",
        to_string = "exact"
    )]
    Exact,
}

impl LtreeSearchMode {
    pub fn to_symbol(&self) -> &'static str {
        match *self {
            Self::Ancestor => "<@",
            Self::Descendant => "@>",
            Self::Exact => "=",
        }
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum SearchModeError {
    #[snafu(display("Could construct LtreeSearchMode from {}", mode))]
    InvalidLtreeSearchMode { mode: String },
}

/// Where clause components, starting with Where. SHould have Or as well
/// to be complete
#[derive(Debug, PartialEq, Eq, EnumString, AsRefStr, Display, IntoStaticStr, Clone, Copy)]
pub enum JoinMode {
    #[strum(serialize = "where", to_string = "WHERE")]
    Where,
    #[strum(serialize = "and", to_string = "AND")]
    And,
}

/// Search mode
#[derive(Debug, PartialEq, Eq)]
pub enum SearchMode {
    Equal,
    // Ilike
    Like,
    // In(Vec<String>),
    Ltree(LtreeSearchMode),
}

impl SearchMode {
    /// Is the search mode Equal
    pub fn is_equal(&self) -> bool {
        *self == Self::Equal
    }
    /// Is the search mode Like
    pub fn is_like(&self) -> bool {
        *self == Self::Like
    }
    /// Convert to static str representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Like => "LIKE",
            Self::Equal => "=",
            Self::Ltree(op) => op.to_symbol(),
        }
    }
    /// Convert from a string to a SearchMode. This function is fallible.
    ///
    /// # Arguments
    /// * `input` - We accept the following inputs:
    ///
    /// Input | SearchMode
    /// --- | ---
    /// = | Equal
    /// equal | Equal
    /// eq | Equal
    /// like | Like
    /// ~ | Like
    /// < | Ancestor
    /// <@ | Ancestor
    /// ancestor | Ancestor
    /// down | ancestor
    /// d | ancestor
    /// . | Exact
    /// exact | Exact
    /// e | Exact
    /// > | Descendant
    /// @> | Descendant
    /// descendant | Descendant
    /// up | Descendant
    /// u | Descendant
    pub fn try_from_str<I: AsRef<str>>(input: I) -> Result<SearchMode, SearchModeError> {
        match input.as_ref().to_ascii_lowercase().as_str() {
            "=" | "equal" | "eq" => Ok(SearchMode::Equal),
            "like" | "~" => Ok(SearchMode::Like),
            "<" | "<@" | "ancestor" | "down" | "d" => Ok(Self::Ltree(LtreeSearchMode::Ancestor)),
            "." | "exact" | "e" => Ok(Self::Ltree(LtreeSearchMode::Exact)),
            ">" | "@>" | "descendant" | "up" | "u" => Ok(Self::Ltree(LtreeSearchMode::Descendant)),
            _ => Err(SearchModeError::InvalidLtreeSearchMode {
                mode: input.as_ref().to_string(),
            }),
        }
    }
    /// Given the object of the comparison, as well as an operation and the
    /// index of the prepared statement parameter, return a search string
    ///
    /// # Arguments
    /// * `joinval` - WHERE or AND
    /// * `this` - The name of the field that we are comparing against
    /// * `op` - The comparison operation we are performing
    /// * `params_cnt` - The index of the prepared statement parameter we are comparing against
    ///
    /// # Returns
    /// * String - Statement fragment `this ? that`.
    pub fn search_string(this: &str, op: &SearchMode, params_cnt: i32) -> String {
        let joinval = if params_cnt == 1 {
            JoinMode::Where
        } else {
            JoinMode::And
        };
        match op {
            Self::Like => format!(" {} {} LIKE ${}", joinval, this, params_cnt),
            //Self::Like => format!(" {} ${} LIKE {}", joinval, params_cnt, this),
            Self::Equal => format!(" {} {} = ${}", joinval, this, params_cnt),
            //Self::Equal => format!(" {} ${} = {}", joinval, params_cnt, this),
            // Self::Ltree(op) => format!(
            //     " {} {} {} text2ltree(${})",
            //     joinval,
            //     this,
            //     op.to_symbol(),
            //     params_cnt
            // ),
            Self::Ltree(op) => format!(
                " {} text2ltree(${}) {} {}",
                joinval,
                params_cnt,
                op.to_symbol(),
                this,
            ),
        }
    }
}
