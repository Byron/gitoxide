use bstr::BString;

use crate::{time::Sign, Time};

/// Serialization with standard `git` format
impl Time {
    /// Serialize this instance into memory, similar to what [`write_to()`][Self::write_to()] would do with arbitrary `Write` implementations.
    pub fn to_bstring(&self) -> BString {
        let mut buf = Vec::with_capacity(64);
        self.write_to(&mut buf).expect("write to memory cannot fail");
        buf.into()
    }

    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        let mut itoa = itoa::Buffer::new();
        out.write_all(itoa.format(self.seconds).as_bytes())?;
        out.write_all(b" ")?;
        out.write_all(match self.sign {
            Sign::Plus => b"+",
            Sign::Minus => b"-",
        })?;

        const ZERO: &[u8; 1] = b"0";

        const SECONDS_PER_HOUR: i32 = 60 * 60;
        let offset = self.offset.abs();
        let hours = offset / SECONDS_PER_HOUR;
        assert!(hours < 25, "offset is more than a day: {hours}");
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

    /// Computes the number of bytes necessary to write it using [`Time::write_to()`].
    pub fn size(&self) -> usize {
        (if self.seconds >= 1_000_000_000_000_000_000 {
            19
        } else if self.seconds >= 100_000_000_000_000_000 {
            18
        } else if self.seconds >= 10_000_000_000_000_000 {
            17
        } else if self.seconds >= 1_000_000_000_000_000 {
            16
        } else if self.seconds >= 100_000_000_000_000 {
            15
        } else if self.seconds >= 10_000_000_000_000 {
            14
        } else if self.seconds >= 1_000_000_000_000 {
            13
        } else if self.seconds >= 100_000_000_000 {
            12
        } else if self.seconds >= 10_000_000_000 {
            11
        } else if self.seconds >= 1_000_000_000 {
            10
        } else if self.seconds >= 100_000_000 {
            9
        } else if self.seconds >= 10_000_000 {
            8
        } else if self.seconds >= 1_000_000 {
            7
        } else if self.seconds >= 100_000 {
            6
        } else if self.seconds >= 10_000 {
            5
        } else if self.seconds >= 1_000 {
            4
        } else if self.seconds >= 100 {
            3
        } else if self.seconds >= 10 {
            2
        } else if self.seconds >= 0 {
            1
            // from here, it's sign + num-digits characters
        } else if self.seconds >= -10 {
            2
        } else if self.seconds >= -100 {
            3
        } else if self.seconds >= -1_000 {
            4
        } else if self.seconds >= -10_000 {
            5
        } else if self.seconds >= -100_000 {
            6
        } else if self.seconds >= -1_000_000 {
            7
        } else if self.seconds >= -10_000_000 {
            8
        } else if self.seconds >= -100_000_000 {
            9
        } else if self.seconds >= -1_000_000_000 {
            10
        } else if self.seconds >= -10_000_000_000 {
            11
        } else if self.seconds >= -100_000_000_000 {
            12
        } else if self.seconds >= -1_000_000_000_000 {
            13
        } else if self.seconds >= -10_000_000_000_000 {
            14
        } else if self.seconds >= -100_000_000_000_000 {
            15
        } else if self.seconds >= -1_000_000_000_000_000 {
            16
        } else if self.seconds >= -10_000_000_000_000_000 {
            17
        } else if self.seconds >= -100_000_000_000_000_000 {
            18
        } else if self.seconds >= -1_000_000_000_000_000_000 {
            19
        } else {
            20
        }) + 2 /*space + offset sign*/ + 2 /*offset hours*/ + 2 /*offset minutes*/
    }
}
