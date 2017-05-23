use std::time;
use std::fmt;
use std::ops;
use std::f64;

use chrono;

use super::error;
use super::error::DurationError;

pub const SECS_PER_DAY: f64 = 60.0*60.0*24.0;
pub const SECS_PER_HOUR: f64 = 60.0*60.0;
pub const SECS_PER_MINUTE: f64 = 60.0;

pub trait TimePoint<Rhs=Self> {
    fn float_duration_since(self, rhs: Rhs) -> error::Result<FloatDuration>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatDuration {
    secs: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DecomposedTime {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub fractional_seconds: f64,
    pub sign: i8,
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

    pub fn decompose(&self) -> DecomposedTime {
        let mut secs_left = self.secs;

        let days = (secs_left / SECS_PER_DAY).trunc();
        secs_left -= days*SECS_PER_DAY;
        let hours = (secs_left / SECS_PER_HOUR).trunc();
        secs_left -= hours*SECS_PER_HOUR;
        let minutes = (secs_left / SECS_PER_MINUTE).trunc();
        secs_left -= minutes*SECS_PER_MINUTE;
        let seconds = secs_left.trunc();
        let fract = secs_left.fract();

        DecomposedTime {
            days: days as u32,
            hours: hours as u32,
            minutes: minutes as u32, 
            seconds: seconds as u32, 
            fractional_seconds: fract,
            sign: self.secs.signum() as i8,
        }
    }

    pub fn as_days(&self) -> f64 {
        self.secs / SECS_PER_DAY
    }
    pub fn as_hours(&self) -> f64 {
        self.secs / SECS_PER_HOUR
    }
    pub fn as_minutes(&self) -> f64 {
        self.secs / SECS_PER_MINUTE
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

    pub fn abs(self) -> FloatDuration {
        FloatDuration { secs: self.secs.abs() }
    }
    pub fn zero() -> FloatDuration {
        FloatDuration {secs: 0.0}
    }
    pub fn is_zero(&self) -> bool {
        self.secs == 0.0 
    }
    pub fn is_positive(&self) -> bool {
        self.secs.is_sign_positive()
    }
    pub fn is_negative(&self) -> bool {
        self.secs.is_sign_negative()
    }

    pub fn min_value() -> FloatDuration {
        FloatDuration { secs: f64::MIN }
    }
    pub fn max_value() -> FloatDuration {
        FloatDuration { secs: f64::MAX }
    }

    pub fn as_std(&self) -> error::Result<time::Duration> {
        if self.secs.is_sign_negative() {
            Err(DurationError::StdOutOfRange)
        } else {
            let seconds = self.secs.trunc();
            let nanos = self.secs.fract() * 1e9;

            Ok(time::Duration::new(seconds as u64, nanos as u32))
        }
    }

    pub fn from_std(duration: time::Duration) -> error::Result<FloatDuration> {
        Ok(FloatDuration::seconds(
            (duration.as_secs() as f64) + (duration.subsec_nanos() as f64)*1.0e-9))
    }

    pub fn from_decomposed(decomposed: &DecomposedTime) -> FloatDuration {
        let seconds = (decomposed.days as f64)*SECS_PER_DAY
            + (decomposed.hours as f64)*SECS_PER_HOUR
            + (decomposed.minutes as f64)*SECS_PER_MINUTE
            + (decomposed.seconds as f64)
            + decomposed.fractional_seconds;

        FloatDuration {
            secs: seconds
        }
    }
}

impl FloatDuration {
    pub fn as_chrono_duration(&self) -> error::Result<chrono::Duration> {
        let is_negative = self.is_negative();
        let std_duration = self.abs().as_std()?;
        let chrono_duration = chrono::Duration::from_std(std_duration)?;
        if is_negative {
            Ok(-chrono_duration)
        } else {
            Ok(chrono_duration)
        }
    }
    
    pub fn from_chrono_duration(duration: &chrono::Duration) 
            -> error::Result<FloatDuration> 
    {
        let is_negative = duration.num_seconds() < 0;        

        let std_duration =
            if is_negative {
                (-*duration).to_std()?
            } else {
                duration.to_std()?
            };

        let float_duration = FloatDuration::from_std(std_duration)?;
        if is_negative {
            Ok(-float_duration)
        } else {
            Ok(float_duration)
        }
    }
}

impl DecomposedTime {
    pub fn new() -> DecomposedTime {
        DecomposedTime { days: 0, hours: 0, minutes: 0, seconds: 0,
            fractional_seconds: 0.0, sign: 1 }
    }

    pub fn from_components(days: u32, hours: u32, minutes: u32, 
               seconds: u32, fractional_seconds: f64) -> DecomposedTime 
    {
        DecomposedTime {
            days, hours, minutes, seconds, fractional_seconds, sign: 1}
    }

    pub fn negate(mut self) -> DecomposedTime {
        self.sign *= -1;
        self
    }
}

impl<Tz: chrono::TimeZone> TimePoint for chrono::DateTime<Tz> {
    fn float_duration_since(self, since: chrono::DateTime<Tz>) 
            -> error::Result<FloatDuration> 
    {
        let chrono_duration = self.signed_duration_since(since);
        FloatDuration::from_chrono_duration(&chrono_duration)
    }
}
impl TimePoint for time::Instant {
    fn float_duration_since(self, since: time::Instant) -> error::Result<FloatDuration> {
        let std_duration = self.duration_since(since);
        FloatDuration::from_std(std_duration)
    }
}
impl TimePoint for time::SystemTime {
    fn float_duration_since(self, since: time::SystemTime) -> error::Result<FloatDuration> {
        let std_duration = self.duration_since(since)?;
        FloatDuration::from_std(std_duration)
    }
}

impl fmt::Display for DecomposedTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.days > 0 {
            write!(fmt, "{}d ", self.days as u64)?;
        }

        if self.sign.is_negative() {
            write!(fmt, "-")?;
        }
        write!(fmt, "{:02}:{:02}:{:02}.{}", 
            self.hours,
            self.minutes,
            self.seconds,
            self.fractional_seconds)
    }
}

impl fmt::Display for FloatDuration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.secs > SECS_PER_DAY {
            write!(fmt, "{} days", self.as_days())
        } else if self.secs > SECS_PER_HOUR {
            write!(fmt, "{} hours", self.as_hours())
        } else if self.secs > SECS_PER_MINUTE {
            write!(fmt, "{} minutes", self.as_minutes())
        } else if self.secs > 1.0 {
            write!(fmt, "{} seconds", self.as_seconds())
        } else if self.secs > 1.0e-3 {
            write!(fmt, "{} milliseconds", self.as_milliseconds())
        } else if self.secs > 1.0e-6 {
            write!(fmt, "{} microseconds", self.as_microseconds())
        } else {
            write!(fmt, "{} nanoseconds", self.as_nanoseconds())
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time;

    #[test]
    fn test_construct() {
        let duration1 = FloatDuration::hours(3.0);
        assert_eq!(duration1.as_hours(), 3.0);
        assert_eq!(duration1.as_minutes(), 180.0);
        assert_eq!(duration1.as_seconds(), 180.0*60.0);
        assert_eq!(duration1.as_days(), 3.0/24.0);
        assert_eq!(duration1.as_milliseconds(), 180.0*60.0*1000.0);

        let duration2 = FloatDuration::milliseconds(55.0);
        assert_eq!(duration2.as_seconds(), 0.055);
        assert_eq!(duration2.as_milliseconds(), 55.0);
        assert_eq!(duration2.as_microseconds(), 55000.0);
        assert_eq!(duration2.as_nanoseconds(), 55000000.0);
        assert!(!duration2.is_zero());

        let duration3 = FloatDuration::zero();
        assert!(duration3.is_zero());

        assert_eq!(FloatDuration::days(1.5), FloatDuration::hours(36.0));
        assert_eq!(FloatDuration::minutes(30.0), FloatDuration::hours(0.5));
        assert_eq!(FloatDuration::seconds(180.0), FloatDuration::minutes(3.0));
        assert_eq!(FloatDuration::seconds(3.5), FloatDuration::milliseconds(3500.0));
        assert_eq!(FloatDuration::microseconds(300.0), FloatDuration::milliseconds(0.30));
        assert_eq!(FloatDuration::nanoseconds(1000.0), FloatDuration::microseconds(1.0));
    }

    #[test]
    fn test_arithmetic() {
        let duration1 = FloatDuration::minutes(5.0) + FloatDuration::seconds(30.0);
        assert_eq!(duration1, FloatDuration::seconds(330.0));

        let duration2 = FloatDuration::hours(3.0) * 2.5;
        assert_eq!(duration2, FloatDuration::hours(7.5));

        let duration3 = FloatDuration::days(3.0) / 3.0 - FloatDuration::hours(2.0);
        assert_eq!(duration3, FloatDuration::hours(22.0));

        let duration4 = FloatDuration::zero() + FloatDuration::milliseconds(500.0) 
            + FloatDuration::microseconds(500.0);
        assert_eq!(duration4, FloatDuration::microseconds(500500.0));

    }

    #[test]
    fn test_std_conversion() {
        let duration1 = FloatDuration::minutes(5.0);
        let std_duration1 = duration1.as_std().unwrap();
        assert!(duration1.is_positive());
        assert_eq!(std_duration1, time::Duration::new(300, 0));
        assert_eq!(FloatDuration::from_std(std_duration1).unwrap(), duration1);
        
        let duration2 = FloatDuration::hours(-2.0);
        assert!(duration2.is_negative());
        assert!(!duration2.as_std().is_ok());
        let std_duration2 = (-duration2).as_std().unwrap();
        assert_eq!(std_duration2, time::Duration::new(3600*2, 0));
        assert_eq!(FloatDuration::from_std(std_duration2).unwrap(), -duration2);
    }

    #[test]
    fn test_display() {
        use std::fmt::Write;

        let mut buffer1 = "".to_string();
        let mut buffer1_2 = "".to_string();
        let duration1 = FloatDuration::minutes(3.5);
        
        write!(buffer1, "{}", duration1).unwrap();
        write!(buffer1_2, "{}", duration1.decompose()).unwrap();
        assert_eq!(buffer1, "3.5 minutes");
        assert_eq!(buffer1_2, "00:03:30.0");

        let mut buffer2 = "".to_string();
        let mut buffer2_2 = "".to_string();
        let duration2 = FloatDuration::days(3.0) + FloatDuration::hours(12.0);

        write!(buffer2, "{}", duration2).unwrap();
        write!(buffer2_2, "{}", duration2.decompose()).unwrap();
        assert_eq!(buffer2, "3.5 days");
        assert_eq!(buffer2_2, "3d 12:00:00.0");
    }
}
