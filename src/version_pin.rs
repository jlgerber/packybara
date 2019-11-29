/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//! The VersionPin combines a Pin and a Distribution,
//! locating the Distribution in the higher order
//! pin space.

use crate::distribution::Distribution;
use crate::pin::Pin;
use crate::pin_error::*;
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
    pub fn level<I>(&mut self, level: I) -> PinResult<&mut Self>
    where
        I: TryInto<Level>,
        PinError: From<<I as TryInto<Level>>::Error>,
    {
        self.level = Some(level.try_into()?);
        Ok(self)
    }

    pub fn role<I>(&mut self, role: I) -> PinResult<&mut Self>
    where
        I: TryInto<Role>,
        PinError: From<<I as TryInto<Role>>::Error>,
    {
        self.role = Some(role.try_into()?);
        Ok(self)
    }

    pub fn platform<I>(&mut self, platform: I) -> PinResult<&mut Self>
    where
        I: TryInto<Platform>,
        PinError: From<<I as TryInto<Platform>>::Error>,
    {
        self.platform = Some(platform.try_into()?);
        Ok(self)
    }

    pub fn site<I>(&mut self, site: I) -> PinResult<&mut Self>
    where
        I: TryInto<Site>,
        PinError: From<<I as TryInto<Site>>::Error>,
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
        let pin = Pin::from_parts(level, role, platform, site);
        VersionPin { distribution, pin }
    }
}
/// Struct that pairs a Distribution with a Pin
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionPin {
    pub distribution: Distribution,
    pub pin: Pin,
}

impl VersionPin {
    /// Construct a VersionPin from a Distribution and a Pin
    pub fn from_parts(distribution: Distribution, pin: Pin) -> Self {
        VersionPin { distribution, pin }
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
    // fn can_construct_versionpin_from_builder() {
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
    //         pin: Pin::try_from_parts("dev01", "model", "cent7_64", "portland").unwrap(),
    //     };
    //     assert_eq!(vp, expect);
    // }

    #[test]
    fn can_construct_versionpin_from_builder() {
        let (vp, expect) = || -> PinResult<(VersionPin, VersionPin)> {
            let vp = VersionPin::new(Distribution::new("maya-2018.sp3"))
                .role("model")?
                .level("dev01")?
                .site("portland")?
                .platform("cent7_64")?
                .build();
            let expect = VersionPin {
                distribution: Distribution::new("maya-2018.sp3"),
                pin: Pin::try_from_parts("dev01", "model", "cent7_64", "portland")?,
            };
            Ok((vp, expect))
        }()
        .unwrap();
        assert_eq!(vp, expect);
    }
}
