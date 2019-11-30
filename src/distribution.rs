/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::coords_error::{CoordsError, CoordsResult};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distribution {
    name: String,
}

impl Distribution {
    pub fn new<T: Into<String>>(name: T) -> CoordsResult<Self> {
        let name = name.into();
        Distribution::validate(name.as_ref())?;
        Ok(Distribution { name })
    }

    fn validate(name: &str) -> CoordsResult<()> {
        if name.matches(" ").count() > 0 {
            return Err(CoordsError::DistributionConstructionError {
                problem: "Contains space in name",
            });
        }
        if name.matches("__").count() > 0 {
            return Err(CoordsError::DistributionConstructionError {
                problem: "double underscore in name not permitted",
            });
        }
        let first_char = name.chars().next();
        if first_char == Some('_') {
            return Err(CoordsError::DistributionConstructionError {
                problem: "name not allowed to start with underscore",
            });
        }
        if name.matches("-").count() != 1 {
            return Err(CoordsError::DistributionConstructionError {
                problem: "distribution name must have a single dash in it",
            });
        }
        Ok(())
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
    fn validation_should_catch_spaces() {
        let d = Distribution::new("foo bar-1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"Contains space in name");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_double_underscores() {
        let d = Distribution::new("foobar-1.0__0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"double underscore in name not permitted");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_starting_with_underscores() {
        let d = Distribution::new("_foobar-1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"name not allowed to start with underscore");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_with_more_than_one_dash() {
        let d = Distribution::new("foobar--1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"distribution name must have a single dash in it");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }

    #[test]
    fn validation_should_catch_names_without_one_dash() {
        let d = Distribution::new("foobar.1.0.0");
        assert_eq!(d.is_err(), true);
        if let CoordsError::DistributionConstructionError { ref problem } = d.unwrap_err() {
            assert_eq!(problem, &"distribution name must have a single dash in it");
        } else {
            panic!("error not of type CoordsError::DistributionConstructionError");
        }
    }
}
