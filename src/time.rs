pub(crate) mod bufs;
mod into;
mod new;

use crate::{Error, Result};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

const THOUSAND: u16 = 1000;
const SIXTY: u8 = 60;

/// A subtitle timestamp.
///
/// Each [`Time`] must ensures:
/// - minutes < 60
/// - seconds < 60
/// - milliseconds < 1000
///
/// Use [`Time::new`] for correct construct.
#[derive(Copy, Clone, Debug, Default, Ord, Eq, PartialOrd, PartialEq)]
pub struct Time {
    pub hours: u16,
    pub mins: u8,
    pub secs: u8,
    pub millis: u16,
}

impl Time {
    /// The zero time.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::ZERO;
    /// assert!(time.is_zero());
    /// assert_eq!(time.as_millis(), 0);
    /// ```
    pub const ZERO: Time = Time::new_unchecked(0, 0, 0, 0);

    /// The minimum time. This is [`ZERO`](Time::ZERO) equivalent.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// assert_eq!(Time::MIN, Time::ZERO);
    /// ```
    pub const MIN: Time = Time::ZERO;

    /// The maximum time.
    ///
    /// In current implementation is above 7 years.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// assert_eq!(Time::MAX, Time::new_unchecked(u16::MAX, 59, 59, 999));
    /// ```
    pub const MAX: Time = Time::new_unchecked(u16::MAX, 59, 59, 999);

    const MAX_SECONDS: u32 = (59 + 59 * 60 + u16::MAX as u32 * 60 * 60);

    /// Returns true if this `Time` spans no time.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// assert!(Time::ZERO.is_zero());
    /// assert!(Time::new_unchecked(0, 0, 0, 0).is_zero());
    /// assert!(Time::from_millis(0).is_zero());
    /// assert!(Time::from_secs(0).is_zero());
    ///
    /// assert!(!Time::new_unchecked(1, 1, 1, 1).is_zero());
    /// assert!(!Time::from_millis(1).is_zero());
    /// assert!(!Time::from_secs(1).is_zero());
    /// ```
    pub const fn is_zero(&self) -> bool {
        matches!(self.hours, 0)
            && matches!(self.mins, 0)
            && matches!(self.secs, 0)
            && matches!(self.millis, 0)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (millis, add) = match self.millis + other.millis {
            x if x < THOUSAND => (x, 0),
            x => (x - THOUSAND, 1),
        };
        let (secs, add) = match self.secs + other.secs + add {
            x if x < SIXTY => (x, 0),
            x => (x - SIXTY, 1),
        };
        let (mins, add) = match self.mins + other.mins + add {
            x if x < SIXTY => (x, 0),
            x => (x - SIXTY, 1),
        };

        Self {
            hours: self.hours + other.hours + add,
            mins,
            secs,
            millis,
        }
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (millis, borrow) = if self.millis >= other.millis {
            (self.millis - other.millis, 0)
        } else {
            (self.millis + THOUSAND - other.millis, 1)
        };

        let osecs = other.secs + borrow;
        let (secs, borrow) = if self.secs >= osecs {
            (self.secs - osecs, 0)
        } else {
            (self.secs + SIXTY - osecs, 1)
        };

        let omins = other.mins + borrow;
        let (mins, borrow) = if self.mins >= omins {
            (self.mins - omins, 0)
        } else {
            (self.mins + SIXTY - omins, 1)
        };

        Self {
            hours: self.hours - other.hours - borrow,
            mins,
            secs,
            millis,
        }
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, dur: Duration) -> Self {
        self.add(Time::from(dur))
    }
}
impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, dur: Duration) -> Self {
        self.sub(Time::from(dur))
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}
