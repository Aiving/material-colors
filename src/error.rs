use core::fmt;

#[cfg(feature = "std")]
use std::error::Error as Err;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error returned when RGB color parsing with the [`Argb::from_str`] fails
    ///
    /// [`Argb::from_str`]: std::str::FromStr
    ParseRGB,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseRGB => "provided string was not RGB-like".fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl Err for Error {
    fn description(&self) -> &str {
        match self {
            Self::ParseRGB => "failed to parse RGB",
        }
    }
}
