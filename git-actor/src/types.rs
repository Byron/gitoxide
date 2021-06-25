use crate::{Sign, Time, SPACE};
use std::io;

impl Time {
    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        itoa::write(&mut out, self.time)?;
        out.write_all(SPACE)?;
        out.write_all(&[match self.sign {
            Sign::Plus => b'+',
            Sign::Minus => b'-',
        }])?;

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
}
