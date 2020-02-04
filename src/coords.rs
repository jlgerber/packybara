/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//! Coords are the coordinates in package space. Package space
//! is a bit weird. It has four dimensions, spanned by Level,
//! Platform, Role, and Site basis vectors.
//!
//! VersionPins are located in package space. They "pin" a
//! distribution to a location in Package Space. That location
//! is the version pin's coordinates. Hence the Coords type.
use crate::coords_error::*;
use crate::{Level, Platform, Role, Site};
use std::convert::{From, TryInto};
use std::fmt;
/// CoordsBuilder follows the builder pattern to allow
/// a fluent style api for setting the coordinates on a
/// per component basis.
pub struct CoordsBuilder {
    level: Option<Level>,
    role: Option<Role>,
    platform: Option<Platform>,
    site: Option<Site>,
}

impl CoordsBuilder {
    // Private function to new up a CoordsBuilder
    fn new() -> CoordsBuilder {
        CoordsBuilder {
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    /// Set the Level using an input that can be converted into
    /// a Level instance. That would generally be either a &str,
    /// or a String.
    pub fn level<I>(&mut self, level: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Level>,
        CoordsError: From<<I as TryInto<Level>>::Error>,
    {
        self.level = Some(level.try_into()?);
        Ok(self)
    }

    /// Set the Role by providing a value that may be converted
    /// into a Role. The attempt may fail, as not all strings
    /// are valid roles.
    ///
    /// # Arguments
    /// * `role` - The name of the role, or subrole. Genreally one or
    ///            more terms separated by underscores. For example
    ///            `model_beta`.
    ///
    /// # Returns Result
    /// * Ok - &mut self as requried by the builder pattern
    /// * Err - CoordsError instance
    pub fn role<I>(&mut self, role: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Role>,
        CoordsError: From<<I as TryInto<Role>>::Error>,
    {
        self.role = Some(role.try_into()?);
        Ok(self)
    }

    /// Sets the Platform by providing a variable whose type (generally
    /// a &str or String ) may be converted into a Platform .
    /// This operation may fail, as not every string is a valid platform.
    pub fn platform<I>(&mut self, platform: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Platform>,
        CoordsError: From<<I as TryInto<Platform>>::Error>,
    {
        self.platform = Some(platform.try_into()?);
        Ok(self)
    }

    /// Sets the Site by providing a variable whose type (genrally a &str
    /// or String) may be converted into a Site.
    /// This operation may fail, as obviously, not every string is a valid
    /// site.
    pub fn site<I: TryInto<Site>>(&mut self, site: I) -> CoordsResult<&mut Self>
    where
        CoordsError: From<<I as TryInto<Site>>::Error>,
    {
        self.site = Some(site.try_into()?);
        Ok(self)
    }

    /// Terminal builder method which constructs a Coords
    /// instance. While the individual setters in the builder
    /// are not guaranteed to succeed, this method is.
    pub fn build(&mut self) -> Coords {
        // slight optimization so we dont have to clone all of
        // those strings. We create a single CoordsBuilder
        // and swap references
        use std::mem;
        let mut tmp = CoordsBuilder::new();
        mem::swap(&mut tmp, self);
        let CoordsBuilder {
            level,
            role,
            platform,
            site,
        } = tmp;
        let level = level.unwrap_or(Level::Facility);
        let role = role.unwrap_or(Role::Any);
        let platform = platform.unwrap_or(Platform::Any);
        let site = site.unwrap_or(Site::Any);
        Coords {
            level,
            role,
            platform,
            site,
        }
    }
}

/// Struct representing the coordinates of a location
/// in "Package Space". The space is spanned by the
/// Level, Role, Platform, and Site axes.
/// Distributions are "pinned" to a location in Package Space,
/// and these "pins" may be located using a Coords instance.
///
/// We are commonly interested in finding the closest
/// distribution to some location in Package Space that corresponds
/// with where we are working at the time. In fact, finding
/// Distributions in Package Space is kind of the "Whole Enchelada"
/// when it comes to package management.
///
/// Enchaladas, by the way, live in Enchalada Space, which
/// is a whole lot more interesting than Package Space, or
/// at least more delicious. But Packybara isn't concerned with
/// Enchiladas.
///
/// Too bad.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Coords {
    /// The Facility, or Show, Sequence, or Shot
    pub level: Level,
    /// Either parent or subrole
    pub role: Role,
    /// The OS variant
    pub platform: Platform,
    /// The physical location
    pub site: Site,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(level:'{}' role:'{}' platform:'{}' site:'{}')",
            self.level, self.role, self.platform, self.site
        )
    }
}
impl Coords {
    /// Construct a CoordsBuilder that you may set properties on
    /// with individual setters. Once done, you call `build()`
    /// to construct a Coords instance.
    ///
    /// # Example
    ///
    /// ```
    /// use packybara::Coords;
    ///
    /// let coordinates = Coords::new()
    ///                 .level("dev01").unwrap()
    ///                 .role("model").unwrap()
    ///                 .build();
    /// ```
    pub fn new() -> CoordsBuilder {
        CoordsBuilder::new()
    }

    /// Construct a new Coords instance
    pub fn try_from_parts<L, R, P, S>(level: L, role: R, platform: P, site: S) -> CoordsResult<Self>
    where
        L: TryInto<Level>,
        R: TryInto<Role>,
        P: TryInto<Platform>,
        S: TryInto<Site>,
        CoordsError: From<<L as TryInto<Level>>::Error>
            + From<<S as TryInto<Site>>::Error>
            + From<<P as TryInto<Platform>>::Error>
            + From<<R as TryInto<Role>>::Error>,
    {
        Ok(Coords {
            level: level.try_into()?,
            role: role.try_into()?,
            platform: platform.try_into()?,
            site: site.try_into()?,
        })
    }
    /// Construct a new Coords instance from constituent parts all at once.
    pub fn from_parts(level: Level, role: Role, platform: Platform, site: Site) -> Self {
        Coords {
            level,
            role,
            platform,
            site,
        }
    }
    /// Get the level from the Coords
    pub fn level(&self) -> &Level {
        &self.level
    }

    /// Get the role from the Coords
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// Get the Platform from the Coords
    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Get the Site from the Coords
    pub fn site(&self) -> &Site {
        &self.site
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn can_construct_from_strs() {
        let pin = Coords::try_from_parts("dev01", "model", "cent7_64", "portland").unwrap();
        assert_eq!(
            pin,
            Coords {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64").unwrap(),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }

    #[test]
    fn can_construct_from_builder() {
        let pin = Coords::new()
            .level("dev01")
            .unwrap()
            .role("model")
            .unwrap()
            .build();
        assert_eq!(
            pin,
            Coords {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("any").unwrap(),
                site: Site::from_str("any").unwrap(),
            }
        );
    }
    #[test]
    fn can_construct_all_from_builder() {
        let pin = Coords::new()
            .level("dev01")
            .unwrap()
            .role("model")
            .unwrap()
            .platform("cent7_64")
            .unwrap()
            .site("portland")
            .unwrap()
            .build();
        assert_eq!(
            pin,
            Coords {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64").unwrap(),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }
}
