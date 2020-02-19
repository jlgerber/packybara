/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use crate::coords_error::*;
use snafu::ResultExt;
use std::convert::TryFrom;
use std::str::FromStr;
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
/// let site = Site::from_str("playa").unwrap();
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
#[derive(
    Debug, Display, EnumString, AsRefStr, IntoStaticStr, PartialEq, Eq, PartialOrd, Ord, Clone, Hash,
)]
pub enum Site {
    #[strum(
        serialize = "any",
        serialize = "Any",
        serialize = "ANY",
        to_string = "any"
    )]
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

impl TryFrom<&str> for Site {
    type Error = CoordsError;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        //fn from(item: &str) -> Self {
        let site = Site::from_str(item).context(FromStrToSiteError {
            input: item.to_string(),
        })?;
        Ok(site)
    }
}

impl TryFrom<String> for Site {
    type Error = CoordsError;

    fn try_from(item: String) -> Result<Self, Self::Error> {
        let site = Site::from_str(item.as_ref()).context(FromStrToSiteError { input: item })?;
        Ok(site)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_construct_from_site_str_playa() {
        let site = Site::from_str("playa").unwrap();
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_title_playa() {
        let site = Site::from_str("Playa").unwrap();
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_venice() {
        let site = Site::from_str("venice").unwrap();
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_title_venice() {
        let site = Site::from_str("Venice").unwrap();
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_prefix_str_playa() {
        let site = Site::from_str("pv").unwrap();
        assert_eq!(site, Site::Playa);
    }

    #[test]
    fn can_construct_from_site_str_vancouver() {
        let site = Site::from_str("vancouver").unwrap();
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_site_str_title_vancouver() {
        let site = Site::from_str("Vancouver").unwrap();
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_prefix_str_vancouver() {
        let site = Site::from_str("bc").unwrap();
        assert_eq!(site, Site::Vancouver);
    }

    #[test]
    fn can_construct_from_site_str_portland() {
        let site = Site::from_str("portland").unwrap();
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_site_str_title_portland() {
        let site = Site::from_str("Portland").unwrap();
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_prefix_str_portland() {
        let site = Site::from_str("pd").unwrap();
        assert_eq!(site, Site::Portland);
    }

    #[test]
    fn can_construct_from_site_str_hyderabad() {
        let site = Site::from_str("hyderabad").unwrap();
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_site_str_title_hyderabad() {
        let site = Site::from_str("Hyderabad").unwrap();
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_prefix_str_hyderabad() {
        let site = Site::from_str("hb").unwrap();
        assert_eq!(site, Site::Hyderabad);
    }

    #[test]
    fn can_construct_from_site_str_montreal() {
        let site = Site::from_str("montreal").unwrap();
        assert_eq!(site, Site::Montreal);
    }

    #[test]
    fn can_construct_from_site_str_title_montreal() {
        let site = Site::from_str("Montreal").unwrap();
        assert_eq!(site, Site::Montreal);
    }

    #[test]
    fn can_construct_from_prefix_str_montreal() {
        let site = Site::from_str("mt").unwrap();
        assert_eq!(site, Site::Montreal);
    }
    #[test]
    fn can_convert_into_static_str() {
        let site: Site = Site::from_str("montreal").unwrap();
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
