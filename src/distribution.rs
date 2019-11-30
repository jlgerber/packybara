/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::coords_error::{CoordsError, CoordsResult};
use std::fmt;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distribution {
    name: String,
}

impl fmt::Display for Distribution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn validate(name: String) -> CoordsResult<String> {
    if name.len() == 0 {
        return Err(CoordsError::DistributionConstructionError {
            problem: "name is blank".to_string(),
        });
    }
    let cnt = name.matches("-").count();
    if cnt != 1 {
        return Err(CoordsError::DistributionConstructionError {
            problem: format!(
                "distribution name must have a single dash in it: '{}'",
                name
            ),
        });
    }
    if name.matches(" ").count() > 0 {
        return Err(CoordsError::DistributionConstructionError {
            problem: format!("Contains space in name: '{}'", name),
        });
    }
    if name.matches("__").count() > 0 {
        return Err(CoordsError::DistributionConstructionError {
            problem: format!("double underscore in name not permitted: '{}'", name),
        });
    }
    let first_char = name.chars().next();
    if first_char == Some('_') {
        return Err(CoordsError::DistributionConstructionError {
            problem: format!("name not allowed to start with underscore: '{}'", name),
        });
    }
    Ok(name)
}

impl Distribution {
    pub fn new<T: Into<String>>(name: T) -> CoordsResult<Self> {
        let name = name.into();
        let name = validate(name)?;
        Ok(Distribution { name })
    }
    pub(crate) fn new_unchecked<T: Into<String>>(name: T) -> Self {
        let name = name.into();
        Distribution { name }
    }
    /// Retrieve the name of the package
    pub fn package(&self) -> &str {
        self.name.split("-").next().unwrap()
    }
    /// retrieve the version of the distribution
    pub fn version(&self) -> &str {
        self.name.split("-").skip(1).next().unwrap()
    }
    /// Retrieve the name of the distribution
    pub fn distribution(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_return_dist_name() {
        let distribution = Distribution::new("maya-2018.sp3").unwrap();
        let name = distribution.package();
        assert_eq!(name, "maya");
    }

    #[test]
    fn can_return_dist_namec() {
        fn inner() -> CoordsResult<String> {
            let distribution = Distribution::new("maya-2018.sp3")?;
            let name = distribution.package();
            Ok(name.to_string())
        }
        let name = inner().unwrap();
        assert_eq!(name, "maya");
    }
    #[test]
    fn validation_should_catch_spaces() {
        let d = Distribution::new("foo bar-1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"Contains space in name: 'foo bar-1.0.0'");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_double_underscores() {
        let d = Distribution::new("foobar-1.0__0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(
                problem,
                &"double underscore in name not permitted: 'foobar-1.0__0'"
            );
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_starting_with_underscores() {
        let d = Distribution::new("_foobar-1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(
                problem,
                &"name not allowed to start with underscore: '_foobar-1.0.0'"
            );
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_with_more_than_one_dash() {
        let d = Distribution::new("foobar--1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(
                problem,
                &"distribution name must have a single dash in it: 'foobar--1.0.0'"
            );
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_without_one_dash() {
        let d = Distribution::new("foobar.1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(
                problem,
                &"distribution name must have a single dash in it: 'foobar.1.0.0'"
            );
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }
}
