use std;
use std::result;
use std::error::Error;
use std::fmt;

use time;

#[derive(Debug, Clone)]
pub enum DurationError {
    StdOutOfRange,
    SystemTimeError(std::time::SystemTimeError),
}

impl Error for DurationError {
    fn description(&self) -> &str {
        match *self {
            DurationError::StdOutOfRange => {
                "Conversion between FloatDuration and std::time::Duration \
                 out of range"
            }
            DurationError::SystemTimeError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DurationError::StdOutOfRange => None,
            DurationError::SystemTimeError(ref e) => Some(e),
        }
    }
}

impl From<time::OutOfRangeError> for DurationError {
    fn from(_: time::OutOfRangeError) -> DurationError {
        DurationError::StdOutOfRange
    }
}
impl From<std::time::SystemTimeError> for DurationError {
    fn from(err: std::time::SystemTimeError) -> DurationError {
        DurationError::SystemTimeError(err)
    }
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub type Result<T> = result::Result<T, DurationError>;
