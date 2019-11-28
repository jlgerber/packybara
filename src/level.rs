use levelspec::{LSpecError, LevelSpec};

/// Level encapsulates alternative locations for packages.
#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    Facility,
    LevelSpec(LevelSpec),
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
    pub fn from_str(level: &str) -> Result<Level, LSpecError> {
        match level.to_lowercase().as_str() {
            "facility" => Ok(Level::Facility),
            _ => Ok(Level::LevelSpec(LevelSpec::new(level)?)),
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
