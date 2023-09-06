use time::{format_description::FormatItem, macros::format_description};

use crate::{time::Format, Time};

/// E.g. `2018-12-24`
pub const SHORT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

/// E.g. `Thu, 18 Aug 2022 12:45:06 +0800`
pub const RFC2822: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `Thu, 8 Aug 2022 12:45:06 +0800`. This is output by `git log --pretty=%aD`.
pub const GIT_RFC2822: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], \
     [day padding:none] \
     [month repr:short] \
     [year] \
     [hour]:[minute]:[second] \
     [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `2022-08-17 22:04:58 +0200`
pub const ISO8601: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]");

/// E.g. `2022-08-17T21:43:13+08:00`
pub const ISO8601_STRICT: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]");

/// E.g. `123456789`
pub const UNIX: Format<'static> = Format::Unix;

/// E.g. `1660874655 +0800`
pub const RAW: Format<'static> = Format::Raw;

/// E.g. `Thu Sep 04 2022 10:45:06 -0400`, like the git `DEFAULT`, but with the year and time fields swapped.
pub const GITOXIDE: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short] [month repr:short] [day] [year] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
);

/// E.g. `Thu Sep 4 10:45:06 2022 -0400`. This is output by `git log --pretty=%ad`.
pub const DEFAULT: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short] \
     [month repr:short] \
     [day padding:none] \
     [hour]:[minute]:[second] \
     [year] \
     [offset_hour sign:mandatory][offset_minute]"
);

mod format_impls {
    use time::format_description::FormatItem;

    use crate::time::Format;

    impl<'a> From<&'a [FormatItem<'a>]> for Format<'a> {
        fn from(f: &'a [FormatItem<'a>]) -> Self {
            Format::Custom(f)
        }
    }
}

/// Formatting
impl Time {
    /// Format this instance according to the given `format`.
    ///
    /// Use the [`format_description`](https://time-rs.github.io/book/api/format-description.html) macro to create and
    /// validate formats at compile time, courtesy of the [`time`] crate.
    pub fn format<'a>(&self, format: impl Into<Format<'a>>) -> String {
        self.format_inner(format.into())
    }

    fn format_inner(&self, format: Format<'_>) -> String {
        match format {
            Format::Custom(format) => self
                .to_time()
                .format(&format)
                .expect("well-known format into memory never fails"),
            Format::Unix => self.seconds.to_string(),
            Format::Raw => self.to_bstring().to_string(),
        }
    }
}

impl Time {
    fn to_time(self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.seconds)
            .expect("always valid unix time")
            .to_offset(time::UtcOffset::from_whole_seconds(self.offset).expect("valid offset"))
    }
}
