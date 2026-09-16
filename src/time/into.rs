use super::Time;
use std::time::Duration;

impl Time {
    /// Returns the duration represented by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    /// use subtitle_lines::Time;
    ///
    /// assert_eq!(Time::from_secs(5).as_duration(), Duration::from_secs(5));
    /// assert_eq!(Time::from_millis(5_432).as_duration(), Duration::from_millis(5_432));
    /// ```
    #[inline]
    pub const fn as_duration(&self) -> Duration {
        Duration::new(self.as_secs() as u64, self.millis as u32 * 1_000_000)
    }

    /// Returns the number of whole seconds contained by this `Time`.
    ///
    /// The returned value does not include the fractional (millisecond) part of the time, which
    /// can be obtained using [`subsec_millis`](Time::subsec_millis).
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// assert_eq!(Time::new_unchecked(0, 0, 5, 730).as_secs(), 5);
    /// assert_eq!(Time::new_unchecked(0, 1, 5, 730).as_secs(), 65);
    /// assert_eq!(Time::new_unchecked(1, 1, 5, 730).as_secs(), 3665);
    /// ```
    pub const fn as_secs(&self) -> u32 {
        self.hours as u32 * 3600 + self.mins as u32 * 60 + self.secs as u32
    }

    /// Returns the fractional part of this `Time`, in whole milliseconds.
    ///
    /// This method does **not** return the length of the duration when represented by
    /// milliseconds. The returned number always represents a fractional portion of a second
    /// (i.e., it is less than one thousand).
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::from_millis(5_432);
    /// assert_eq!(time.as_secs(), 5);
    /// assert_eq!(time.subsec_millis(), 432);
    /// ```
    pub const fn subsec_millis(&self) -> u16 {
        self.millis
    }

    /// Returns the total number of whole milliseconds contained by this `Time`.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// assert_eq!(Time::from_millis(730_023).as_millis(), 730_023);
    /// ```
    pub const fn as_millis(&self) -> u64 {
        self.as_secs() as u64 * 1000 + self.millis as u64
    }

    /// Returns the number of seconds contained by this `Time` as `f32`.
    ///
    /// The returned value includes the fractional (millisecond) part of the time.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::Time;
    ///
    /// let time = Time::new_unchecked(0, 0, 2, 700);
    /// assert_eq!(time.as_secs_f32(), 2.7);
    /// ```
    pub const fn as_secs_f32(&self) -> f32 {
        self.as_secs() as f32 + self.millis as f32 / 1000.0
    }
}

impl From<Time> for Duration {
    fn from(time: Time) -> Duration {
        time.as_duration()
    }
}
