/*******************************************************
 * Copyright (C) 2019 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use std::convert::From;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Site models our facility locations recognized by package management
// # Converting from &str
/// Site implements the std::str::FromStr trait
///
/// ## Example
///
/// ```rust
/// use packybara::Site;
/// use std::str::FromStr;
///
/// let site = Site::from_str("playa");
/// assert_eq!(site, Site::Playa);
/// ```
///
/// # Converting to str
///
/// Site implements AsRef<str>
///
/// ## Example
///
/// ```rust
/// use packybara::Site;
///
/// let site = Site::Hyderabad;
/// assert_eq!(site.as_ref(), "hyderabad");
/// ```
/// # Displaying
///
/// Site implements Display
///
/// ## Example
///
/// ```
/// use packybara::Site;
///
/// let site = Site::Vancouver;
/// let site_string = format!("{}", site);
/// assert_eq!(site_string.as_str(), "vancouver");
/// ```
#[derive(Debug, Display, EnumString, AsRefStr, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord)]
pub enum Site {
    Unknown(String),
    #[strum(serialize = "any", to_string = "any")]
    Any,
    #[strum(
        serialize = "playa",
        serialize = "Playa",
        serialize = "venice",
        serialize = "Venice",
        serialize = "pv",
        to_string = "playa"
    )]
    Playa,
    #[strum(
        serialize = "vancouver",
        serialize = "Vancouver",
        serialize = "bc",
        to_string = "vancouver"
    )]
    Vancouver,
    #[strum(
        serialize = "portland",
        serialize = "Portland",
        serialize = "pd",
        to_string = "portland"
    )]
    Portland,
    #[strum(
        serialize = "hyderabad",
        serialize = "Hyderabad",
        serialize = "hb",
        to_string = "hyderabad"
    )]
    Hyderabad,
    #[strum(
        serialize = "montreal",
        serialize = "Montreal",
        serialize = "mt",
        to_string = "montreal"
    )]
    Montreal,
}

impl From<&str> for Site {
    fn from(item: &str) -> Self {
        Site::from_str(item)
    }
}

impl From<String> for Site {
    fn from(item: String) -> Self {
        Site::from_str(item.as_ref())
    }
}

impl Site {
    pub fn from_str(input: &str) -> Self {
        match <Site as std::str::FromStr>::from_str(input) {
            Ok(site) => site,
            Err(_) => Site::Unknown(input.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_construct_from_site_str_playa() {
        let site = Site::from_str("playa");
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_title_playa() {
        let site = Site::from_str("Playa");
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_venice() {
        let site = Site::from_str("venice");
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_title_venice() {
        let site = Site::from_str("Venice");
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_prefix_str_playa() {
        let site = Site::from_str("pv");
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_vancouver() {
        let site = Site::from_str("vancouver");
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_site_str_title_vancouver() {
        let site = Site::from_str("Vancouver");
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_prefix_str_vancouver() {
        let site = Site::from_str("bc");
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_site_str_portland() {
        let site = Site::from_str("portland");
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_site_str_title_portland() {
        let site = Site::from_str("Portland");
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_prefix_str_portland() {
        let site = Site::from_str("pd");
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_site_str_hyderabad() {
        let site = Site::from_str("hyderabad");
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_site_str_title_hyderabad() {
        let site = Site::from_str("Hyderabad");
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_prefix_str_hyderabad() {
        let site = Site::from_str("hb");
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_site_str_montreal() {
        let site = Site::from_str("montreal");
        assert_eq!(site, Site::Montreal);
    }

    #[test]
    fn can_construct_from_site_str_title_montreal() {
        let site = Site::from_str("Montreal");
        assert_eq!(site, Site::Montreal);
    }

    #[test]
    fn can_construct_from_prefix_str_montreal() {
        let site = Site::from_str("mt");
        assert_eq!(site, Site::Montreal);
    }
    #[test]
    fn can_convert_into_static_str() {
        let site: Site = Site::from_str("montreal");
        let pstr: &'static str = site.into();
        assert_eq!(pstr, "montreal");
    }

    #[test]
    fn can_convert_to_str() {
        let site = Site::Playa;
        assert_eq!(site.as_ref(), "playa");
    }

    #[test]
    fn can_display() {
        let site = Site::Vancouver;
        let pstr = format!("{}", site);
        assert_eq!(pstr.as_str(), "vancouver");
    }

    #[test]
    fn can_sort() {
        assert!(Site::Playa < Site::Vancouver);
        assert!(Site::Vancouver < Site::Portland);
        assert!(Site::Portland < Site::Hyderabad);
        assert!(Site::Hyderabad < Site::Montreal);
        assert!(Site::Montreal > Site::Playa);
    }
}
