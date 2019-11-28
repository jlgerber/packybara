/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
#[derive(Debug, PartialEq, Eq)]
pub struct Pin {
    level: String,
    role: String,
    platform: String,
    site: String,
}

impl Pin {
    /// Construct a new Pin instance
    pub fn new<I>(level: I, role: I, platform: I, site: I) -> Self
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
        let pin = Pin::new("dev01", "model", "cent7_64", "portland");
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
