use std::io;

use crate::{Sign, Time, SPACE};

impl From<i32> for Sign {
    fn from(v: i32) -> Self {
        if v < 0 {
            Sign::Minus
        } else {
            Sign::Plus
        }
    }
}

impl Time {
    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        itoa::write(&mut out, self.time)?;
        out.write_all(SPACE)?;
        out.write_all(match self.sign {
            Sign::Plus => b"+",
            Sign::Minus => b"-",
        })?;

        const ZERO: &[u8; 1] = b"0";

        const SECONDS_PER_HOUR: i32 = 60 * 60;
        let offset = self.offset.abs();
        let hours = offset / SECONDS_PER_HOUR;
        assert!(hours < 25, "offset is more than a day: {}", hours);
        let minutes = (offset - (hours * SECONDS_PER_HOUR)) / 60;

        if hours < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, hours)?;

        if minutes < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, minutes).map(|_| ())
    }
    /// Computes the number of bytes necessary to render this time
    pub fn size(&self) -> usize {
        // TODO: this is not year 2038 safe…but we also can't parse larger numbers (or represent them) anyway. It's a trap nonetheless
        //       that can be fixed by increasing the size to usize.
        (if self.time >= 1_000_000_000 {
            10
        } else if self.time >= 100_000_000 {
            9
        } else if self.time >= 10_000_000 {
            8
        } else if self.time >= 1_000_000 {
            7
        } else if self.time >= 100_000 {
            6
        } else if self.time >= 10_000 {
            5
        } else if self.time >= 1_000 {
            4
        } else if self.time >= 100 {
            3
        } else if self.time >= 10 {
            2
        } else {
            1
        }) + 2 /*space + sign*/ + 2 /*hours*/ + 2 /*minutes*/
    }
}
