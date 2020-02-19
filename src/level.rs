/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::coords_error::*;
use failure::Fail;
use levelspec::LevelSpec;
use snafu::ResultExt;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Level {
    Facility,
    LevelSpec(LevelSpec),
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Facility => write!(f, "facility"),
            Self::LevelSpec(ref lspec) => write!(f, "{}", lspec),
        }
    }
}

impl TryFrom<&str> for Level {
    type Error = CoordsError;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        if item.to_lowercase() == "facility" {
            return Ok(Level::Facility);
        }
        match LevelSpec::new(item) {
            Ok(val) => Ok(Level::LevelSpec(val)),
            Err(_) => Err(CoordsError::InvalidLevel {
                input: item.to_string(),
            }),
        }
    }
}

impl TryFrom<String> for Level {
    type Error = CoordsError;

    fn try_from(item: String) -> Result<Self, Self::Error> {
        if item.to_lowercase() == "facility" {
            return Ok(Level::Facility);
        }
        let item_copy = item.clone();
        match LevelSpec::new(item) {
            Ok(val) => Ok(Level::LevelSpec(val)),
            Err(_) => Err(CoordsError::InvalidLevel { input: item_copy }),
        }
    }
}

impl Level {
    /// new up a Level from a &str
    ///
    /// # Arguments
    ///
    /// * `level` - The level in levelspec string form
    ///
    /// # Example
    /// ```rust
    /// use packybara::Level;
    ///
    /// let level = Level::from_str("facility");
    /// assert_eq!(level.expect("cant convert to facility"), Level::Facility);
    /// ```
    pub fn from_str(level: &str) -> CoordsResult<Level> {
        match level.to_lowercase().as_str() {
            "facility" => Ok(Level::Facility),
            _ => {
                let level_s = level.to_string();
                let ls = LevelSpec::new(level)
                    .map_err(|e| e.compat())
                    .context(NewLevelspecError { level: level_s })?;
                Ok(Level::LevelSpec(ls))
            }
        }
    }
    /// Convert to a string.
    /// Note: Level::Facility becomes "facility"
    ///
    /// # Example
    ///
    /// ```rust
    /// use packybara::Level;
    ///
    /// let level = Level::from_str("dev01.rd.9999").expect("cant convert");
    /// assert_eq!(level.to_string().as_str(), "dev01.rd.9999");
    /// ```
    pub fn to_string(&self) -> String {
        match *self {
            Self::Facility => "facility".to_string(),
            Self::LevelSpec(ref ls) => ls.to_vec_str().join("."),
        }
    }
    /// Retrieve the show from the Level. If the Level is
    /// facility, we return facility
    ///
    /// Note: we dont return an Option<&str> here because the underlying
    /// postgres schema/api uses facility to denote the facility level
    ///
    /// # Example
    /// ```rust
    /// use packybara::Level;
    ///
    /// let level = Level::Facility;
    /// assert_eq!(level.show(), "facility");
    ///
    /// let level = Level::from_str("dev01.rd.9999").expect("couldnt convert");
    /// assert_eq!(level.show(), "dev01");
    /// ```
    pub fn show(&self) -> &str {
        match *self {
            Self::Facility => "facility",
            Self::LevelSpec(ref ls) => ls.show().unwrap(),
        }
    }

    /// Test whether the instance of Level is Level::Facility
    ///
    /// # Example
    /// ```rust
    /// use packybara::Level;
    ///
    /// let level = Level::from_str("dev01.rd.9999").expect("unable to convert");
    /// assert!(!level.is_facility());
    ///
    /// assert!(Level::Facility.is_facility());
    /// ```
    pub fn is_facility(&self) -> bool {
        *self == Level::Facility
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_facility() {
        let level = Level::from_str("facility").unwrap();
        assert_eq!(level, Level::Facility);
    }

    #[test]
    fn can_construct_show() {
        let level = Level::from_str("dev01").unwrap();
        assert_eq!(level, Level::LevelSpec(LevelSpec::new("dev01").unwrap()));
    }

    #[test]
    fn can_construct_sequence() {
        let level = Level::from_str("dev01.rd").unwrap();
        assert_eq!(level, Level::LevelSpec(LevelSpec::new("dev01.rd").unwrap()));
    }

    #[test]
    fn can_construct_shot() {
        let level = Level::from_str("dev01.rd.0001").unwrap();
        assert_eq!(
            level,
            Level::LevelSpec(LevelSpec::new("dev01.rd.0001").unwrap())
        );
    }

    #[test]
    fn can_convert_facility_to_string() {
        let level = Level::from_str("facility").unwrap();
        assert_eq!(level.to_string(), "facility".to_string());
    }

    #[test]
    fn can_convert_shot_to_string() {
        let level = Level::from_str("dev01.rd.0001").unwrap();
        assert_eq!(level.to_string(), "dev01.rd.0001".to_string());
    }

    #[test]
    fn can_get_show_from_shot() {
        let level = Level::from_str("dev01.rd.0001").unwrap();
        assert_eq!(level.show(), "dev01");
    }

    #[test]
    fn can_get_show_from_facility() {
        let level = Level::from_str("facility").unwrap();
        assert_eq!(level.show(), "facility");
    }

    #[test]
    fn can_test_for_facility() {
        let level = Level::from_str("facility").unwrap();
        assert_eq!(level.is_facility(), true);
    }
}
