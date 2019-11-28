use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Platform models the os variants available to us.
///
/// # Converting from &str
/// Platform implements the std::str::FromStr trait
///
/// ## Example
///
/// ```rust
/// use packybara::Platform;
/// use std::str::FromStr;
///
/// let platform = Platform::from_str("cent6_64").expect("cant convert");
/// assert_eq!(platform, Platform::Cent6);
/// ```
///
/// # Converting to str
///
/// Platform implements AsRef<str>
///
/// ## Example
///
/// ```rust
/// use packybara::Platform;
///
/// let platform = Platform::Cent7;
/// assert_eq!(platform.as_ref(), "cent7_64");
/// ```
/// # Displaying
///
/// Platform implements Display
///
/// ## Example
///
/// ```
/// use packybara::Platform;
///
/// let platform = Platform::Cent7;
/// let platform_string = format!("{}", platform);
/// assert_eq!(platform_string.as_str(), "cent7_64");
/// ```
#[derive(Debug, Display, EnumString, AsRefStr, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    #[strum(serialize = "win_xp", to_string = "win_xp")]
    WinXp,
    #[strum(serialize = "win_10", to_string = "win_10")]
    Win10,
    #[strum(serialize = "cent5_64", to_string = "cent5_64")]
    Cent5,
    #[strum(serialize = "cent6_64", to_string = "cent6_64")]
    Cent6,
    #[strum(serialize = "cent7_64", to_string = "cent7_64")]
    Cent7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_construct_from_str() {
        let platform = Platform::from_str("cent6_64").unwrap();
        assert_eq!(platform, Platform::Cent6);
    }

    #[test]
    fn can_convert_into_static_str() {
        let platform: Platform = Platform::from_str("cent6_64").unwrap();
        let pstr: &'static str = platform.into();
        assert_eq!(pstr, "cent6_64");
    }

    #[test]
    fn can_convert_to_str() {
        let platform = Platform::Cent7;
        assert_eq!(platform.as_ref(), "cent7_64");
    }

    #[test]
    fn can_display() {
        let platform = Platform::Cent7;
        let pstr = format!("{}", platform);
        assert_eq!(pstr.as_str(), "cent7_64");
    }

    #[test]
    fn can_sort() {
        assert!(Platform::WinXp < Platform::Win10);
        assert!(Platform::Win10 < Platform::Cent5);
        assert!(Platform::Cent5 < Platform::Cent6);
        assert!(Platform::Cent6 < Platform::Cent7);
    }
}
