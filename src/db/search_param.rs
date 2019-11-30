/// Provide a measure of extensibility
#[derive(Debug, PartialOrd,Ord, PartialEq, Eq)]
pub enum<'a> SearchParam<'a> {
    Show(&'a str),
    OrderBy(&'str),
    OrderAsc,
    OrderDesc,
    Limit(u32),
}