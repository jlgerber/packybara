//! The VersionPin combines a Pin and a Distribution,
//! locating the Distribution in the higher order
//! pin space.

use crate::pin::Pin;
use crate::distribution::Distribution;

/// Struct that pairs a Distribution with a Pin
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct VersionPin {
    pub distribution: Distribution,
    pub pin: Pin,
};

impl VersionPin {
    /// Construct a VersionPin from a Distribution and a Pin
    fn from_parts(distribution: Distribution, pin: Pin) -> Self {
        VersionPin {
            distribution,
            pin
        }
    }
}

