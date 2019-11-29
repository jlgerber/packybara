/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::{Level, Platform, Role, Site};

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

    pub fn level<I: Into<Level>>(&mut self, level: I) -> &mut Self {
        self.level = Some(level.into());
        self
    }

    pub fn role<I: Into<Role>>(&mut self, role: I) -> &mut Self {
        self.role = Some(role.into());
        self
    }

    pub fn platform<I: Into<Platform>>(&mut self, platform: I) -> &mut Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn site<I: Into<Site>>(&mut self, site: I) -> &mut Self {
        self.site = Some(site.into());
        self
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
    /// let pin = Pin::new().level("dev01").role("model").build();
    /// ```
    pub fn new() -> PinBuilder {
        PinBuilder::new()
    }

    /// Construct a new Pin instance
    pub fn from_parts<L, R, P, S>(level: L, role: R, platform: P, site: S) -> Self
    where
        L: Into<Level>,
        R: Into<Role>,
        P: Into<Platform>,
        S: Into<Site>,
    {
        Pin {
            level: level.into(),
            role: role.into(),
            platform: platform.into(),
            site: site.into(),
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

    #[test]
    fn can_construct_from_strs() {
        let pin = Pin::from_parts("dev01", "model", "cent7_64", "portland");
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model"),
                platform: Platform::from_str("cent7_64"),
                site: Site::from_str("portland"),
            }
        );
    }

    #[test]
    fn can_construct_from_builder() {
        let pin = Pin::new().level("dev01").role("model").build();
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model"),
                platform: Platform::from_str("any"),
                site: Site::from_str("any"),
            }
        );
    }
    #[test]
    fn can_construct_all_from_builder() {
        let pin = Pin::new()
            .level("dev01")
            .role("model")
            .platform("cent7_64")
            .site("portland")
            .build();
        assert_eq!(
            pin,
            Pin {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model"),
                platform: Platform::from_str("cent7_64"),
                site: Site::from_str("portland"),
            }
        );
    }
}
