/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
/// alias for the type of a serial column in postgres. Note that
/// we could change to BIGINTEGER at some point, an this would be
/// changed to i64
pub type IdType = i32;
pub type LongIdType = i64;
