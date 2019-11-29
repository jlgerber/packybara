use failure::Compat;
use levelspec::LSpecError;
use snafu::{ResultExt, Snafu};
use strum;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum PinError {
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
}

pub type PinResult<T, E = PinError> = std::result::Result<T, E>;
