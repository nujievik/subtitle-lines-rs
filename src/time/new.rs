use super::*;

impl Time {
    /// Tries construct a new [`Time`].
    ///
    /// # Errors
    ///
    /// Returns an error in the next cases:
    /// - minutes >= 60
    /// - seconds >= 60
    /// - milliseconds >= 1000
    pub const fn new(hours: u16, mins: u8, secs: u8, millis: u16) -> Result<Time> {
        if mins >= SIXTY {
            Err(Error::ValueValidation("minutes must be < 60"))
        } else if secs >= SIXTY {
            Err(Error::ValueValidation("seconds must be < 60"))
        } else if millis >= THOUSAND {
            Err(Error::ValueValidation("milliseconds must be < 1000"))
        } else {
            Ok(Self::new_unchecked(hours, mins, secs, millis))
        }
    }

    /// Constructs a new [`Time`] without checks. User must ensures:
    /// - minutes < 60
    /// - seconds < 60
    /// - milliseconds < 1000
    pub const fn new_unchecked(hours: u16, mins: u8, secs: u8, millis: u16) -> Time {
        Self {
            hours,
            mins,
            secs,
            millis,
        }
    }

    /// Creates a new `Time` from the specified duration.
    ///
    /// # Panics
    /// Panics if the given duration overflows the `Time` size.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_duration(Duration::new(2, 569_000_000));
    ///
    /// assert_eq!(2, time.as_secs());
    /// assert_eq!(569, time.subsec_millis());
    /// ```
    pub const fn from_duration(duration: Duration) -> Time {
        let millis = duration.subsec_millis() as u16;
        let mut time = Time::from_secs(duration.as_secs() as u32);
        time.millis = millis;
        time
    }

    /// Creates a new `Time` from the specified number of whole seconds.
    ///
    /// # Panics
    /// Panics if the given number of seconds overflows the `Time` size.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_secs(5);
    ///
    /// assert_eq!(5, time.as_secs());
    /// assert_eq!(0, time.subsec_millis());
    /// ```
    pub const fn from_secs(secs: u32) -> Time {
        if secs > Time::MAX_SECONDS {
            panic!("overflow in Time::from_secs");
        }

        let s = (secs % 60) as u8;
        let total_mins = secs / 60;

        let mins = (total_mins % 60) as u8;
        let hours = (total_mins / 60) as u16;

        Time::new_unchecked(hours, mins, s, 0)
    }

    /// Creates a new `Time` from the specified number of milliseconds.
    ///
    /// # Panics
    /// Panics if the given number of milliseconds overflows the `Time` size.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_millis(2_569);
    ///
    /// assert_eq!(2, time.as_secs());
    /// assert_eq!(569, time.subsec_millis());
    /// ```
    pub const fn from_millis(millis: u64) -> Time {
        if millis > Self::MAX_MILLIS {
            panic!("overflow in Time::from_millis");
        }

        let m = (millis % 1000) as u16;
        let mut time = Time::from_secs((millis / 1000) as u32);
        time.millis = m;
        time
    }

    /// Creates a new `Time` from the specified number of hours.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_hours(6);
    ///
    /// assert_eq!(6 * 60 * 60, time.as_secs());
    /// assert_eq!(0, time.subsec_millis());
    /// ```
    pub const fn from_hours(hours: u16) -> Time {
        Time::new_unchecked(hours, 0, 0, 0)
    }

    /// Creates a new `Time` from the specified number of minutes.
    ///
    /// # Panics
    /// Panics if the given number of minutes overflows the `Time` size.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_mins(10);
    ///
    /// assert_eq!(10 * 60, time.as_secs());
    /// assert_eq!(0, time.subsec_millis());
    /// ```
    pub const fn from_mins(mins: u32) -> Time {
        if mins > (u16::MAX as u32 * 60 + 59) {
            panic!("overflow in Time::from_mins");
        }

        let hours = (mins / 60) as u16;
        let mins = (mins % 60) as u8;

        Time::new_unchecked(hours, mins, 0, 0)
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Time {
        Time::from_duration(duration)
    }
}
