use super::Time;
use core::fmt::NumBuffer;

pub struct TimeBuf<const N: usize> {
    pub(crate) num_buf: NumBuffer<u16>,
    pub(crate) buf: [u8; N],
}

pub struct AssTimeBuf(pub(crate) TimeBuf<10>);
pub struct SrtTimeBuf(pub(crate) TimeBuf<12>);
pub struct VttTimeBuf(pub(crate) TimeBuf<12>);

impl AssTimeBuf {
    #[inline]
    pub const fn new() -> AssTimeBuf {
        Self::from_num_buffer(NumBuffer::new())
    }

    #[inline]
    pub const fn from_num_buffer(num_buf: NumBuffer<u16>) -> AssTimeBuf {
        AssTimeBuf(TimeBuf {
            num_buf,
            buf: *b"0:00:00.00",
        })
    }

    /// Formats a time into an ASS timestamp format (`H:MM:SS.cc`).
    ///
    /// The resulting timestamp uses centiseconds (two decimal places) for fractional seconds.
    /// Only the first two significant digits of the milliseconds are kept; the third digit
    /// is simply truncated (dropped) without rounding.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::{Time, ass::AssTimeBuf};
    ///
    /// let mut buf = AssTimeBuf::new();
    ///
    /// // Basic formatting
    /// assert_eq!(b"0:00:00.00", buf.format_time(Time::new_unchecked(0, 0, 0, 0)));
    /// assert_eq!(b"1:23:45.67", buf.format_time(Time::new_unchecked(1, 23, 45, 670)));
    ///
    /// // Max hours clamp
    /// assert_eq!(b"9:00:00.00", buf.format_time(Time::new_unchecked(15, 0, 0, 0)));
    ///
    /// // Milliseconds truncation (takes first 2 digits, 3rd is dropped)
    /// assert_eq!(b"0:00:00.12", buf.format_time(Time::new_unchecked(0, 0, 0, 129)));
    /// assert_eq!(b"0:00:00.06", buf.format_time(Time::new_unchecked(0, 0, 0, 67)));
    /// ```
    #[inline]
    pub fn format_time(&mut self, mut time: Time) -> &[u8] {
        time.hours = time.hours.min(9);
        time.millis = time.millis / 10;

        let hours_bytes = time.hours.format_into(&mut self.0.num_buf).as_bytes();
        self.0.buf[0] = hours_bytes[0];
        self.0.write_two_digits(2, time.mins as u16);
        self.0.write_two_digits(5, time.secs as u16);
        self.0.write_two_digits(8, time.millis);

        &self.0.buf
    }

    /// Same as [`format_time`](AssTimeBuf::format_time) but returns string.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::{Time, ass::AssTimeBuf};
    ///
    /// let mut buf = AssTimeBuf::new();
    /// assert_eq!("1:23:45.67", buf.format_time_to_str(Time::new_unchecked(1, 23, 45, 670)));
    /// ```
    #[inline]
    pub fn format_time_to_str(&mut self, time: Time) -> &str {
        unsafe { str::from_utf8_unchecked(self.format_time(time)) }
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn num_buffer(&mut self) -> &mut NumBuffer<u16> {
        self.0.num_buffer()
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn into_num_buffer(self) -> NumBuffer<u16> {
        self.0.into_num_buffer()
    }
}

impl SrtTimeBuf {
    #[inline]
    pub const fn new() -> SrtTimeBuf {
        Self::from_num_buffer(NumBuffer::new())
    }

    #[inline]
    pub const fn from_num_buffer(num_buf: NumBuffer<u16>) -> SrtTimeBuf {
        SrtTimeBuf(TimeBuf {
            num_buf,
            buf: *b"00:00:00,000",
        })
    }

    /// Formats a time into an SRT timestamp format (`HH:MM:SS,mms`).
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::{Time, srt::SrtTimeBuf};
    ///
    /// let mut buf = SrtTimeBuf::new();
    ///
    /// // Basic formatting
    /// assert_eq!(b"00:00:00,000", buf.format_time(Time::new_unchecked(0, 0, 0, 0)));
    /// assert_eq!(b"01:23:45,678", buf.format_time(Time::new_unchecked(1, 23, 45, 678)));
    ///
    /// // Max hours clamp
    /// assert_eq!(b"99:00:00,000", buf.format_time(Time::new_unchecked(150, 0, 0, 0)));
    /// ```
    #[inline]
    pub fn format_time(&mut self, time: Time) -> &[u8] {
        self.0.format_time(time)
    }

    /// Same as [`format_time`](SrtTimeBuf::format_time) but returns string.
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::{Time, srt::SrtTimeBuf};
    ///
    /// let mut buf = SrtTimeBuf::new();
    /// assert_eq!("01:23:45,678", buf.format_time_to_str(Time::new_unchecked(1, 23, 45, 678)));
    /// ```
    #[inline]
    pub fn format_time_to_str(&mut self, time: Time) -> &str {
        unsafe { str::from_utf8_unchecked(self.format_time(time)) }
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn num_buffer(&mut self) -> &mut NumBuffer<u16> {
        self.0.num_buffer()
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn into_num_buffer(self) -> NumBuffer<u16> {
        self.0.into_num_buffer()
    }
}

impl VttTimeBuf {
    #[inline]
    pub const fn new() -> VttTimeBuf {
        Self::from_num_buffer(NumBuffer::new())
    }

    #[inline]
    pub const fn from_num_buffer(num_buf: NumBuffer<u16>) -> VttTimeBuf {
        VttTimeBuf(TimeBuf {
            num_buf,
            buf: *b"00:00:00.000",
        })
    }

    /// Formats a time into a VTT timestamp format (`HH:MM:SS.mms`).
    ///
    /// # Examples
    /// ```
    /// use subtitle_lines::{Time, vtt::VttTimeBuf};
    ///
    /// let mut buf = VttTimeBuf::new();
    ///
    /// // Basic formatting
    /// assert_eq!(b"00:00:00.000", buf.format_time(Time::new_unchecked(0, 0, 0, 0)));
    /// assert_eq!(b"01:23:45.678", buf.format_time(Time::new_unchecked(1, 23, 45, 678)));
    ///
    /// // Max hours clamp
    /// assert_eq!(b"99:00:00.000", buf.format_time(Time::new_unchecked(150, 0, 0, 0)));
    /// ```
    #[inline]
    pub fn format_time(&mut self, time: Time) -> &[u8] {
        self.0.format_time(time)
    }

    /// Same as [`format_time`](VttTimeBuf::format_time) but returns string.
    ///
    /// /// # Examples
    /// ```
    /// use subtitle_lines::{Time, vtt::VttTimeBuf};
    ///
    /// let mut buf = VttTimeBuf::new();
    /// assert_eq!("01:23:45.678", buf.format_time_to_str(Time::new_unchecked(1, 23, 45, 678)));
    /// ```
    #[inline]
    pub fn format_time_to_str(&mut self, time: Time) -> &str {
        unsafe { str::from_utf8_unchecked(self.format_time(time)) }
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn num_buffer(&mut self) -> &mut NumBuffer<u16> {
        self.0.num_buffer()
    }

    /// Returns internal num buffer.
    #[inline]
    pub fn into_num_buffer(self) -> NumBuffer<u16> {
        self.0.into_num_buffer()
    }
}

impl TimeBuf<12> {
    #[inline]
    pub(crate) fn format_time(&mut self, mut time: Time) -> &[u8] {
        time.hours = time.hours.min(99);

        self.write_two_digits(0, time.hours);
        self.write_two_digits(3, time.mins as u16);
        self.write_two_digits(6, time.secs as u16);
        self.write_three_digits(9, time.millis);

        &self.buf
    }
}

impl<const N: usize> TimeBuf<N> {
    #[inline]
    fn num_buffer(&mut self) -> &mut NumBuffer<u16> {
        &mut self.num_buf
    }

    #[inline]
    fn into_num_buffer(self) -> NumBuffer<u16> {
        self.num_buf
    }

    fn write_two_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = bytes[0];
            }
            _ => self.buf[start_idx..start_idx + 2].copy_from_slice(bytes),
        }
    }

    fn write_three_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = b'0';
                self.buf[start_idx + 2] = bytes[0];
            }
            2 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1..start_idx + 3].copy_from_slice(bytes);
            }
            _ => self.buf[start_idx..start_idx + 3].copy_from_slice(bytes),
        }
    }
}
