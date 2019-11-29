use failure::Compat;
use levelspec::LSpecError;
use snafu::Snafu;
use strum;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum CtxError {
    #[snafu(display("Could construct Level from {}", input))]
    InvalidLevel { input: String },
    #[snafu(display("Error constructing LevelSpec {}: {}", level, source))]
    NewLevelspecError {
        level: String,
        source: Compat<LSpecError>,
    },
    #[snafu(display("Error converting string to Site for {}. Error: {}", input, source))]
    FromStrToSiteError {
        input: String,
        source: strum::ParseError,
    },
    #[snafu(display("Problem with input string for Role conversion: {}.", input,))]
    FromStrToRoleError { input: String },
    #[snafu(display("Error converting string to Platform for {}. Error: {}", input, source))]
    FromStrToPlatformError {
        input: String,
        source: strum::ParseError,
    },
}

pub type CtxResult<T, E = CtxError> = std::result::Result<T, E>;
