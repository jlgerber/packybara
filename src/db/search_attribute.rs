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
pub enum SearchMode {
    #[strum(
        serialize = "ancestor",
        serialize = "Ancestor",
        serialize = "ANCESTOR",
        to_string = "ancestor"
    )]
    Ancestor,
    #[strum(
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

impl SearchMode {
    pub fn to_symbol(&self) -> &'static str {
        match *self {
            Self::Ancestor => "<@",
            Self::Descendant => "@>",
            Self::Exact => "=",
        }
    }
}
