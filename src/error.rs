use std::result;
use std::error::Error;
use std::fmt;

use time;

#[derive(Debug, Clone)]
pub enum DurationError {
    StdOutOfRange,
}

impl Error for DurationError {
    fn description(&self) -> &str {
        match *self {
            DurationError::StdOutOfRange => 
                "Conversion between FloatDuration and std::time::Duration \
                 out of range"
        }
    }
}

impl From<time::OutOfRangeError> for DurationError {
    fn from(err: time::OutOfRangeError) -> DurationError {
        DurationError::StdOutOfRange
    }
}

impl fmt::Display for DurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub type Result<T> = result::Result<T, DurationError>;
