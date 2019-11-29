/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::pin_error::*;
use crate::{Level, Platform, Role, Site};
use std::convert::{From, TryInto};

pub struct PinBuilder {
    level: Option<Level>,
    role: Option<Role>,
    platform: Option<Platform>,
    site: Option<Site>,
}

impl PinBuilder {
    fn new() -> PinBuilder {
        PinBuilder {
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

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

    pub fn platform<I: Into<Platform>>(&mut self, platform: I) -> &mut Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn site<I: TryInto<Site>>(&mut self, site: I) -> PinResult<&mut Self>
    where
        PinError: From<<I as TryInto<Site>>::Error>,
    {
        self.site = Some(site.try_into()?);
        Ok(self)
    }

    pub fn build(&mut self) -> Pin {
        // slight optimization so we dont have to clone all of
        // those strings. We create a single PinBuilder
        // and swap references
        use std::mem;
        let mut tmp = PinBuilder::new();
        mem::swap(&mut tmp, self);
        let PinBuilder {
            level,
            role,
            platform,
            site,
        } = tmp;
        let level = level.unwrap_or(Level::Facility);
        let role = role.unwrap_or(Role::Any);
        let platform = platform.unwrap_or(Platform::Any);
        let site = site.unwrap_or(Site::Any);
        Pin {
            level,
            role,
            platform,
            site,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pin {
    level: Level,
    role: Role,
    platform: Platform,
    site: Site,
}

impl Pin {
    /// Construct a PinBuilder that you may set properties on
    /// with individual setters. Once done, you must call `build()`
    /// to construct the final Pin.
    ///
    /// # Example
    ///
    /// ```
    /// use packybara::Pin;
    ///
    /// let pin = Pin::new()
    ///                 .level("dev01").unwrap()
    ///                 .role("model").unwrap()
    ///                 .build();
    /// ```
    pub fn new() -> PinBuilder {
        PinBuilder::new()
    }

    /// Construct a new Pin instance
    pub fn try_from_parts<L, R, P, S>(level: L, role: R, platform: P, site: S) -> PinResult<Self>
    where
        L: TryInto<Level>,
        R: TryInto<Role>,
        P: Into<Platform>,
        S: TryInto<Site>,
        PinError: From<<L as TryInto<Level>>::Error>
            + From<<S as TryInto<Site>>::Error>
            + From<<R as TryInto<Role>>::Error>,
    {
        Ok(Pin {
            level: level.try_into()?,
            role: role.try_into()?,
            platform: platform.into(),
            site: site.try_into()?,
        })
    }
    /// Construct a new Pin instance
    pub fn from_parts(level: Level, role: Role, platform: Platform, site: Site) -> Self {
        Pin {
            level,
            role,
            platform,
            site,
        }
    }
    /// Get the level from teh Pin
    pub fn level(&self) -> &Level {
        &self.level
    }

    /// Get the role from th Pin
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// Get the Platform from the Pin
    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Get the site from the Pin
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
        let pin = Pin::try_from_parts("dev01", "model", "cent7_64", "portland").unwrap();
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64"),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }

    #[test]
    fn can_construct_from_builder() {
        let pin = Pin::new()
            .level("dev01")
            .unwrap()
            .role("model")
            .unwrap()
            .build();
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("any"),
                site: Site::from_str("any").unwrap(),
            }
        );
    }
    #[test]
    fn can_construct_all_from_builder() {
        let pin = Pin::new()
            .level("dev01")
            .unwrap()
            .role("model")
            .unwrap()
            .platform("cent7_64")
            //.unwrap()
            .site("portland")
            .unwrap()
            .build();
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64"),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }
}
