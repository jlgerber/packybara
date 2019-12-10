/// if predicate is true, return trueval. Otherwise, return falseval.
///
/// # Arguments
/// *`predicate` - bool.
/// * `trueval` - The value returned if `predicate` is true.
/// * `falseval` - The value returned if `predicate` is false.
///
/// # Returns
/// T
pub fn pred_true_false<T>(predicate: bool, trueval: T, falseval: T) -> T {
    if predicate {
        trueval
    } else {
        falseval
    }
}
