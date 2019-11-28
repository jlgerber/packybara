/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
pub struct PinBuilder {
    level: Option<String>,
    role: Option<String>,
    platform: Option<String>,
    site: Option<String>,
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

    pub fn level<I: Into<String>>(&mut self, level: I) -> &mut Self {
        self.level = Some(level.into());
        self
    }

    pub fn role<I: Into<String>>(&mut self, role: I) -> &mut Self {
        self.role = Some(role.into());
        self
    }

    pub fn platform<I: Into<String>>(&mut self, platform: I) -> &mut Self {
        self.platform = Some(platform.into());
        self
    }

    pub fn site<I: Into<String>>(&mut self, site: I) -> &mut Self {
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
        let level = level.unwrap_or("facility".to_string());
        let role = role.unwrap_or("any".to_string());
        let platform = platform.unwrap_or("any".to_string());
        let site = site.unwrap_or("any".to_string());
        Pin {
            level,
            role,
            platform,
            site,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pin {
    level: String,
    role: String,
    platform: String,
    site: String,
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
    pub fn from_parts<I>(level: I, role: I, platform: I, site: I) -> Self
    where
        I: Into<String>,
    {
        Pin {
            level: level.into(),
            role: role.into(),
            platform: platform.into(),
            site: site.into(),
        }
    }

    /// Get the level from teh Pin
    pub fn level(&self) -> &str {
        self.level.as_str()
    }

    /// Get the role from th Pin
    pub fn role(&self) -> &str {
        self.role.as_str()
    }

    /// Get the Platform from the Pin
    pub fn platform(&self) -> &str {
        self.platform.as_str()
    }

    /// Get the site from the Pin
    pub fn site(&self) -> &str {
        self.site.as_str()
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
                level: "dev01".to_string(),
                role: "model".to_string(),
                platform: "cent7_64".to_string(),
                site: "portland".to_string(),
            }
        );
    }

    #[test]
    fn can_construct_from_builder() {
        let pin = Pin::new().level("dev01").role("model").build();
        assert_eq!(
            pin,
            Pin {
                level: "dev01".to_string(),
                role: "model".to_string(),
                platform: "any".to_string(),
                site: "any".to_string(),
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
                level: "dev01".to_string(),
                role: "model".to_string(),
                platform: "cent7_64".to_string(),
                site: "portland".to_string(),
            }
        );
    }
}
