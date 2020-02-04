/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
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
