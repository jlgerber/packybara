/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::ctx_error::*;
use crate::{Level, Platform, Role, Site};
use std::convert::{From, TryInto};

pub struct CtxBuilder {
    level: Option<Level>,
    role: Option<Role>,
    platform: Option<Platform>,
    site: Option<Site>,
}

impl CtxBuilder {
    fn new() -> CtxBuilder {
        CtxBuilder {
            level: None,
            role: None,
            platform: None,
            site: None,
        }
    }

    pub fn level<I>(&mut self, level: I) -> CtxResult<&mut Self>
    where
        I: TryInto<Level>,
        CtxError: From<<I as TryInto<Level>>::Error>,
    {
        self.level = Some(level.try_into()?);
        Ok(self)
    }

    pub fn role<I>(&mut self, role: I) -> CtxResult<&mut Self>
    where
        I: TryInto<Role>,
        CtxError: From<<I as TryInto<Role>>::Error>,
    {
        self.role = Some(role.try_into()?);
        Ok(self)
    }

    pub fn platform<I>(&mut self, platform: I) -> CtxResult<&mut Self>
    where
        I: TryInto<Platform>,
        CtxError: From<<I as TryInto<Platform>>::Error>,
    {
        self.platform = Some(platform.try_into()?);
        Ok(self)
    }

    pub fn site<I: TryInto<Site>>(&mut self, site: I) -> CtxResult<&mut Self>
    where
        CtxError: From<<I as TryInto<Site>>::Error>,
    {
        self.site = Some(site.try_into()?);
        Ok(self)
    }

    pub fn build(&mut self) -> Ctx {
        // slight optimization so we dont have to clone all of
        // those strings. We create a single CtxBuilder
        // and swap references
        use std::mem;
        let mut tmp = CtxBuilder::new();
        mem::swap(&mut tmp, self);
        let CtxBuilder {
            level,
            role,
            platform,
            site,
        } = tmp;
        let level = level.unwrap_or(Level::Facility);
        let role = role.unwrap_or(Role::Any);
        let platform = platform.unwrap_or(Platform::Any);
        let site = site.unwrap_or(Site::Any);
        Ctx {
            level,
            role,
            platform,
            site,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ctx {
    level: Level,
    role: Role,
    platform: Platform,
    site: Site,
}

impl Ctx {
    /// Construct a CtxBuilder that you may set properties on
    /// with individual setters. Once done, you must call `build()`
    /// to construct the final Ctx.
    ///
    /// # Example
    ///
    /// ```
    /// use packybara::Ctx;
    ///
    /// let pin = Ctx::new()
    ///                 .level("dev01").unwrap()
    ///                 .role("model").unwrap()
    ///                 .build();
    /// ```
    pub fn new() -> CtxBuilder {
        CtxBuilder::new()
    }

    /// Construct a new Ctx instance
    pub fn try_from_parts<L, R, P, S>(level: L, role: R, platform: P, site: S) -> CtxResult<Self>
    where
        L: TryInto<Level>,
        R: TryInto<Role>,
        P: TryInto<Platform>,
        S: TryInto<Site>,
        CtxError: From<<L as TryInto<Level>>::Error>
            + From<<S as TryInto<Site>>::Error>
            + From<<P as TryInto<Platform>>::Error>
            + From<<R as TryInto<Role>>::Error>,
    {
        Ok(Ctx {
            level: level.try_into()?,
            role: role.try_into()?,
            platform: platform.try_into()?,
            site: site.try_into()?,
        })
    }
    /// Construct a new Ctx instance
    pub fn from_parts(level: Level, role: Role, platform: Platform, site: Site) -> Self {
        Ctx {
            level,
            role,
            platform,
            site,
        }
    }
    /// Get the level from teh Ctx
    pub fn level(&self) -> &Level {
        &self.level
    }

    /// Get the role from th Ctx
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// Get the Platform from the Ctx
    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Get the site from the Ctx
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
        let pin = Ctx::try_from_parts("dev01", "model", "cent7_64", "portland").unwrap();
        assert_eq!(
            pin,
            Ctx {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64").unwrap(),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }

    #[test]
    fn can_construct_from_builder() {
        let pin = Ctx::new()
            .level("dev01")
            .unwrap()
            .role("model")
            .unwrap()
            .build();
        assert_eq!(
            pin,
            Ctx {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("any").unwrap(),
                site: Site::from_str("any").unwrap(),
            }
        );
    }
    #[test]
    fn can_construct_all_from_builder() {
        let pin = Ctx::new()
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
            Ctx {
                level: Level::from_str("dev01").unwrap(),
                role: Role::from_str("model").unwrap(),
                platform: Platform::from_str("cent7_64").unwrap(),
                site: Site::from_str("portland").unwrap(),
            }
        );
    }
}
