use std::time;
use std::ops;

use chrono;
use chrono::{Timelike};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatDuration {
    secs: f64,
}

impl FloatDuration {
    pub fn new(secs: f64) -> FloatDuration {
        FloatDuration {secs}
    }

    pub fn days(days: f64) -> FloatDuration {
        FloatDuration {secs: days * (3600.0 * 24.0)}
    }
    pub fn hours(hours: f64) -> FloatDuration {
        FloatDuration {secs: hours * 3600.0}
    }
    pub fn minutes(mins: f64) -> FloatDuration {
        FloatDuration {secs: mins * 60.0}
    }
    pub fn seconds(secs: f64) -> FloatDuration {
        FloatDuration {secs}
    }
    pub fn milliseconds(millis: f64) -> FloatDuration {
        FloatDuration {secs: millis / 1000.0}
    }
    pub fn microseconds(micros: f64) -> FloatDuration {
        FloatDuration {secs: micros / 1.0e6}
    }
    pub fn nanoseconds(nanos: f64) -> FloatDuration {
        FloatDuration {secs: nanos / 1.0e9}
    }

    pub fn as_days(&self) -> f64 {
        self.secs / (3600.0*24.0)
    }
    pub fn as_hours(&self) -> f64 {
        self.secs / 3600.0
    }
    pub fn as_minutes(&self) -> f64 {
        self.secs / 60.0
    }
    pub fn as_seconds(&self) -> f64 {
        self.secs
    }
    pub fn as_milliseconds(&self) -> f64 {
        self.secs * 1.0e3
    }
    pub fn as_microseconds(&self) -> f64 {
        self.secs * 1.0e6
    }
    pub fn as_nanoseconds(&self) -> f64 {
        self.secs * 1.0e9
    }

    pub fn as_std(&self) -> Option<time::Duration> {
        if self.secs.is_sign_negative() {
            None
        } else {
            let seconds = self.secs.trunc();
            let nanos = self.secs.fract() * 1e9;

            Some(time::Duration::new(seconds as u64, nanos as u32))
        }
    }

    pub fn from_std(duration: time::Duration) -> FloatDuration {
        return FloatDuration::seconds(
            (duration.as_secs() as f64) * (duration.subsec_nanos() as f64)/1.0e-9);
    }
}

impl ops::Neg for FloatDuration {
    type Output = FloatDuration;

    fn neg(self) -> FloatDuration {
        FloatDuration {secs: -self.secs}
    }
}

impl ops::Add<FloatDuration> for FloatDuration {
    type Output = FloatDuration;

    fn add(self, rhs: FloatDuration) -> FloatDuration {
        FloatDuration {secs: self.secs + rhs.secs}
    }
}
impl ops::Sub<FloatDuration> for FloatDuration {
    type Output = FloatDuration;

    fn sub(self, rhs: FloatDuration) -> FloatDuration {
        FloatDuration {secs: self.secs - rhs.secs}
    }
}

impl ops::Mul<f64> for FloatDuration {
    type Output = FloatDuration;

    fn mul(self, rhs: f64) -> FloatDuration {
        FloatDuration {secs: self.secs * rhs}
    }
}
impl ops::Div<f64> for FloatDuration {
    type Output = FloatDuration;

    fn div(self, rhs: f64) -> FloatDuration {
        FloatDuration {secs: self.secs / rhs}
    }
}

impl ops::AddAssign<FloatDuration> for FloatDuration {
    fn add_assign(&mut self, rhs: FloatDuration) {
        self.secs += rhs.secs;
    }
}
impl ops::SubAssign<FloatDuration> for FloatDuration {
    fn sub_assign(&mut self, rhs: FloatDuration) {
        self.secs -= rhs.secs;
    }
}

impl ops::MulAssign<f64> for FloatDuration {
    fn mul_assign(&mut self, rhs: f64) {
        self.secs *= rhs;
    }
}
impl ops::DivAssign<f64> for FloatDuration {
    fn div_assign(&mut self, rhs: f64) {
        self.secs /= rhs;
    }
}
