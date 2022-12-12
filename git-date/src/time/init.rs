use std::{convert::TryInto, ops::Sub};

use crate::{time::Sign, Time};

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
        let offset_in_seconds = time::UtcOffset::local_offset_at(now).ok()?.whole_seconds();
        Self {
            seconds_since_unix_epoch,
            offset_in_seconds,
            sign: offset_in_seconds.into(),
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
            sign: offset_in_seconds.into(),
        }
    }
}
