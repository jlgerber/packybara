use failure::Compat;
use levelspec::LSpecError;
use snafu::Snafu;
use strum;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum CoordsError {
    /// InvalidLevel
    #[snafu(display("Could construct Level from {}", input))]
    InvalidLevel { input: String },
    /// NewLevelspecError
    #[snafu(display("Error constructing LevelSpec {}: {}", level, source))]
    NewLevelspecError {
        level: String,
        source: Compat<LSpecError>,
    },
    /// FromStrToSiteError
    #[snafu(display("Error converting string to Site for {}. Error: {}", input, source))]
    FromStrToSiteError {
        input: String,
        source: strum::ParseError,
    },
    /// FromStrToRoleError
    #[snafu(display("Problem with input string for Role conversion: {}.", input,))]
    FromStrToRoleError { input: String },
    /// FromStrToPlatformError
    #[snafu(display("Error converting string to Platform for {}. Error: {}", input, source))]
    FromStrToPlatformError {
        input: String,
        source: strum::ParseError,
    },
    /// DistributionConstructionError
    #[snafu(display("Error distribution. Problem: {}", problem))]
    DistributionConstructionError { problem: &'static str },
}

pub type CoordsResult<T, E = CoordsError> = std::result::Result<T, E>;
