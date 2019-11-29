/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//! The VersionPin combines a Coords and a Distribution,
//! locating the Distribution in the higher order
//! pin space.

use crate::coords::Coords;
use crate::coords_error::*;
use crate::distribution::Distribution;
use crate::{Level, Platform, Role, Site};
use std::convert::{From, TryInto};

pub struct VersionPinBuilder {
    distribution: Distribution,
    level: Option<Level>,
    role: Option<Role>,
    platform: Option<Platform>,
    site: Option<Site>,
}

impl VersionPinBuilder {
    /// New up a VersionPinBuilder
    pub fn new(distribution: Distribution) -> Self {
        VersionPinBuilder {
            distribution,
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    /// set the level
    pub fn level<I>(&mut self, level: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Level>,
        CoordsError: From<<I as TryInto<Level>>::Error>,
    {
        self.level = Some(level.try_into()?);
        Ok(self)
    }

    pub fn role<I>(&mut self, role: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Role>,
        CoordsError: From<<I as TryInto<Role>>::Error>,
    {
        self.role = Some(role.try_into()?);
        Ok(self)
    }

    pub fn platform<I>(&mut self, platform: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Platform>,
        CoordsError: From<<I as TryInto<Platform>>::Error>,
    {
        self.platform = Some(platform.try_into()?);
        Ok(self)
    }

    pub fn site<I>(&mut self, site: I) -> CoordsResult<&mut Self>
    where
        I: TryInto<Site>,
        CoordsError: From<<I as TryInto<Site>>::Error>,
    {
        self.site = Some(site.try_into()?);
        Ok(self)
    }

    pub fn build(&mut self) -> VersionPin {
        // slight optimization so we dont have to clone all of
        // those strings. We create a single PinBuilder
        // and swap references
        use std::mem;
        let mut tmp = VersionPinBuilder::new(Distribution::new(""));
        mem::swap(&mut tmp, self);
        let VersionPinBuilder {
            distribution,
            level,
            role,
            platform,
            site,
        } = tmp;
        let level = level.unwrap_or(Level::Facility);
        let role = role.unwrap_or(Role::Any);
        let platform = platform.unwrap_or(Platform::Any);
        let site = site.unwrap_or(Site::Any);
        let coords = Coords::from_parts(level, role, platform, site);
        VersionPin {
            distribution,
            coords,
        }
    }
}
/// Struct that pairs a Distribution with a  Coords
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPin {
    pub distribution: Distribution,
    pub coords: Coords,
}

impl VersionPin {
    /// Construct a VersionPin from a Distribution and a  Coords
    pub fn from_parts(distribution: Distribution, coords: Coords) -> Self {
        VersionPin {
            distribution,
            coords,
        }
    }

    /// Construct a new VersionPinBuilder, which has
    /// various setter methods as well as a build method
    /// which must be used to construct the final VersionPin
    pub fn new(distribution: Distribution) -> VersionPinBuilder {
        VersionPinBuilder::new(distribution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn can_construct_versioncoords_from_builder() {
    //     let vp = VersionPin::new(Distribution::new("maya-2018.sp3"))
    //         .role("model")
    //         .unwrap()
    //         .level("dev01")
    //         .unwrap()
    //         .site("portland")
    //         .unwrap()
    //         .platform("cent7_64")
    //         .unwrap()
    //         .build();
    //     let expect = VersionPin {
    //         distribution: Distribution::new("maya-2018.sp3"),
    //         coords:  Coords::try_from_parts("dev01", "model", "cent7_64", "portland").unwrap(),
    //     };
    //     assert_eq!(vp, expect);
    // }

    #[test]
    fn can_construct_versioncoords_from_builder() {
        let (vp, expect) = || -> CoordsResult<(VersionPin, VersionPin)> {
            let vp = VersionPin::new(Distribution::new("maya-2018.sp3"))
                .role("model")?
                .level("dev01")?
                .site("portland")?
                .platform("cent7_64")?
                .build();
            let expect = VersionPin {
                distribution: Distribution::new("maya-2018.sp3"),
                coords: Coords::try_from_parts("dev01", "model", "cent7_64", "portland")?,
            };
            Ok((vp, expect))
        }()
        .unwrap();
        assert_eq!(vp, expect);
    }
}
