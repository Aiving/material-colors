use std::{error::Error as Err, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// An error returned when parsing a RGB color using [`from_str`] fails
    ///
    /// [`from_str`]: std::str::FromStr::from_str
    ParseRGB,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseRGB => "provided string was not RGB-like".fmt(f),
        }
    }
}

impl Err for Error {
    fn description(&self) -> &str {
        match self {
            Self::ParseRGB => "failed to parse RGB",
        }
    }
}
