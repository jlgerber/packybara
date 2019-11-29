use failure::Compat;
use failure::Fail;
use levelspec::LSpecError;
use levelspec::LevelSpec;
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use std::convert::TryFrom;

#[derive(Debug, Snafu)]
pub enum LevelError {
    #[snafu(display("Could construct Level from {}", input))]
    InvalidLevel { input: String },
    #[snafu(display("Error constructing LevelSpec {}: {}", level, source))]
    NewLevelspecError {
        level: String,
        source: Compat<LSpecError>,
    },
}

pub type LevelResult<T, E = LevelError> = std::result::Result<T, E>;
