use std::convert::TryInto;
use std::io;
use std::ops::Sub;

use crate::Time;

/// Indicates if a number is positive or negative for use in [`Time`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Sign {
    Plus,
    Minus,
}

impl From<i32> for Sign {
    fn from(v: i32) -> Self {
        if v < 0 {
            Sign::Minus
        } else {
            Sign::Plus
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            seconds_since_unix_epoch: 0,
            offset_in_seconds: 0,
            sign: Sign::Plus,
        }
    }
}

/// Instantiation
impl Time {
    /// Create a new instance from seconds and offset.
    pub fn new(seconds_since_unix_epoch: u32, offset_in_seconds: i32) -> Self {
        Time {
            seconds_since_unix_epoch,
            offset_in_seconds,
            sign: offset_in_seconds.into(),
        }
    }

    /// Return the current time without figuring out a timezone offset
    pub fn now_utc() -> Self {
        let seconds_since_unix_epoch = time::OffsetDateTime::now_utc()
            .sub(std::time::SystemTime::UNIX_EPOCH)
            .whole_seconds()
            .try_into()
            .expect("this is not year 2038");
        Self {
            seconds_since_unix_epoch,
            offset_in_seconds: 0,
            sign: Sign::Plus,
        }
    }

    /// Return the current local time, or `None` if the local time wasn't available.
    pub fn now_local() -> Option<Self> {
        let now = time::OffsetDateTime::now_utc();
        let seconds_since_unix_epoch = now
            .sub(std::time::SystemTime::UNIX_EPOCH)
            .whole_seconds()
            .try_into()
            .expect("this is not year 2038");
        // TODO: make this work without cfg(unsound_local_offset), see
        //       https://github.com/time-rs/time/issues/293#issuecomment-909158529
        let offset = time::UtcOffset::local_offset_at(now).ok()?;
        Self {
            seconds_since_unix_epoch,
            offset_in_seconds: offset.whole_seconds(),
            sign: Sign::Plus,
        }
        .into()
    }

    /// Return the current local time, or the one at UTC if the local time wasn't available.
    pub fn now_local_or_utc() -> Self {
        let now = time::OffsetDateTime::now_utc();
        let seconds_since_unix_epoch = now
            .sub(std::time::SystemTime::UNIX_EPOCH)
            .whole_seconds()
            .try_into()
            .expect("this is not year 2038");
        // TODO: make this work without cfg(unsound_local_offset), see
        //       https://github.com/time-rs/time/issues/293#issuecomment-909158529
        let offset_in_seconds = time::UtcOffset::local_offset_at(now)
            .map(|ofs| ofs.whole_seconds())
            .unwrap_or(0);
        Self {
            seconds_since_unix_epoch,
            offset_in_seconds,
            sign: Sign::Plus,
        }
    }
}

impl Time {
    /// Return true if this time has been initialized to anything non-default, i.e. 0.
    pub fn is_set(&self) -> bool {
        *self != Self::default()
    }

    /// Return the passed seconds since epoch since this signature was made.
    pub fn seconds(&self) -> u32 {
        self.seconds_since_unix_epoch
    }

    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        let mut itoa = itoa::Buffer::new();
        out.write_all(itoa.format(self.seconds_since_unix_epoch).as_bytes())?;
        out.write_all(b" ")?;
        out.write_all(match self.sign {
            Sign::Plus => b"+",
            Sign::Minus => b"-",
        })?;

        const ZERO: &[u8; 1] = b"0";

        const SECONDS_PER_HOUR: i32 = 60 * 60;
        let offset = self.offset_in_seconds.abs();
        let hours = offset / SECONDS_PER_HOUR;
        assert!(hours < 25, "offset is more than a day: {}", hours);
        let minutes = (offset - (hours * SECONDS_PER_HOUR)) / 60;

        if hours < 10 {
            out.write_all(ZERO)?;
        }
        out.write_all(itoa.format(hours).as_bytes())?;

        if minutes < 10 {
            out.write_all(ZERO)?;
        }
        out.write_all(itoa.format(minutes).as_bytes()).map(|_| ())
    }

    /// Computes the number of bytes necessary to render this time.
    pub fn size(&self) -> usize {
        // TODO: this is not year 2038 safe…but we also can't parse larger numbers (or represent them) anyway. It's a trap nonetheless
        //       that can be fixed by increasing the size to usize.
        (if self.seconds_since_unix_epoch >= 1_000_000_000 {
            10
        } else if self.seconds_since_unix_epoch >= 100_000_000 {
            9
        } else if self.seconds_since_unix_epoch >= 10_000_000 {
            8
        } else if self.seconds_since_unix_epoch >= 1_000_000 {
            7
        } else if self.seconds_since_unix_epoch >= 100_000 {
            6
        } else if self.seconds_since_unix_epoch >= 10_000 {
            5
        } else if self.seconds_since_unix_epoch >= 1_000 {
            4
        } else if self.seconds_since_unix_epoch >= 100 {
            3
        } else if self.seconds_since_unix_epoch >= 10 {
            2
        } else {
            1
        }) + 2 /*space + sign*/ + 2 /*hours*/ + 2 /*minutes*/
    }
}
