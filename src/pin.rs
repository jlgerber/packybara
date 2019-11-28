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
