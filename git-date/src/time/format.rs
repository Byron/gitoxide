use crate::Time;
use time::format_description::FormatItem;
use time::formatting::Formattable;
use time::macros::format_description;

/// E.g. `2018-12-24`
pub const SHORT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// E.g. `Thu, 18 Aug 2022 12:45:06 +0800`
pub const RFC2822: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `2022-08-17 22:04:58 +0200`
pub const ISO8601: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]");

/// E.g. `Thu 04 Sep 2022 10:45:06 -0400`
pub const DEFAULT: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short] [month repr:short] [day] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

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
    fn to_time(self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.seconds_since_unix_epoch as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(self.offset_in_seconds).expect("valid offset"))
    }
}
