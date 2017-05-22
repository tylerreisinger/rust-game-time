use std::time;
use std::fmt;
use std::ops;

pub const SECS_PER_DAY: f64 = 60.0*60.0*24.0;
pub const SECS_PER_HOUR: f64 = 60.0*60.0;
pub const SECS_PER_MINUTE: f64 = 60.0;

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
        secs_left -= hours*SECS_PER_MINUTE;
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
            (duration.as_secs() as f64) + (duration.subsec_nanos() as f64)*1.0e-9);
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

impl DecomposedTime {
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

impl fmt::Display for FloatDuration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let decompose_time = self.decompose();

        if decompose_time.days > 0 {
            write!(fmt, "{}d ", decompose_time.days as u64)?;
        }

        if decompose_time.sign.is_negative() {
            write!(fmt, "-")?;
        }
        write!(fmt, "{:02}:{:02}:{:02}.{}", 
            decompose_time.hours,
            decompose_time.minutes,
            decompose_time.seconds,
            decompose_time.fractional_seconds)
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
        assert_eq!(FloatDuration::from_std(std_duration1), duration1);
        
        let duration2 = FloatDuration::hours(-2.0);
        assert!(duration2.is_negative());
        assert_eq!(duration2.as_std(), None);
        let std_duration2 = (-duration2).as_std().unwrap();
        assert_eq!(std_duration2, time::Duration::new(3600*2, 0));
        assert_eq!(FloatDuration::from_std(std_duration2), -duration2);
    }
}
