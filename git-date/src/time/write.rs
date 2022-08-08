use crate::time::Sign;
use crate::Time;
use bstr::BString;
use time::format_description::well_known::Iso8601;
use time::formatting::Formattable;

impl Time {
    /// Serialize this instance into a Iso8601 BString.
    pub fn to_bstring(&self) -> BString {
        time::OffsetDateTime::from_unix_timestamp(self.seconds_since_unix_epoch as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
            .format(&Iso8601::DEFAULT)
            .unwrap()
            .into()
    }

    /// Serialize this instance into a BString, formatting it using the provided `time::format_description`
    pub fn to_bstring_with_formatter(&self, formatter: &(impl Formattable + ?Sized)) -> BString {
        time::OffsetDateTime::from_unix_timestamp(self.seconds_since_unix_epoch as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
            .format(formatter)
            .unwrap()
            .into()
    }

    /// Serialize this instance to `out` in a format suitable for use in header fields of serialized git commits or tags.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
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
