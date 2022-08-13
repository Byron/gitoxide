use crate::Time;
use time::format_description::FormatItem;
use time::formatting::Formattable;
use time::macros::format_description;

/// E.g. `2018-12-24`
pub const SHORT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// Formatting
impl Time {
    /// Format this instance according to the given `format`.
    ///
    /// Use the [`format_description`] macro to create and validate formats at compile time, courtesy of the [`time`] crate.
    pub fn format(&self, format: &(impl Formattable + ?Sized)) -> String {
        self.to_time()
            .format(&format)
            .expect("well-known format into memory never fails")
    }
}

impl Time {
    fn to_time(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.seconds_since_unix_epoch as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
    }
}
