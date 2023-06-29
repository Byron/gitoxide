use std::ops::Sub;

use crate::{time::Sign, OffsetInSeconds, SecondsSinceUnixEpoch, Time};

/// Instantiation
impl Time {
    /// Create a new instance from seconds and offset.
    pub fn new(seconds: SecondsSinceUnixEpoch, offset: OffsetInSeconds) -> Self {
        Time {
            seconds,
            offset,
            sign: offset.into(),
        }
    }

    /// Return the current time without figuring out a timezone offset
    pub fn now_utc() -> Self {
        let seconds = time::OffsetDateTime::now_utc()
            .sub(std::time::SystemTime::UNIX_EPOCH)
            .whole_seconds();
        Self {
            seconds,
            offset: 0,
            sign: Sign::Plus,
        }
    }

    /// Return the current local time, or `None` if the local time wasn't available.
    pub fn now_local() -> Option<Self> {
        let now = time::OffsetDateTime::now_utc();
        let seconds = now.sub(std::time::SystemTime::UNIX_EPOCH).whole_seconds();
        // TODO: make this work without cfg(unsound_local_offset), see
        //       https://github.com/time-rs/time/issues/293#issuecomment-909158529
        let offset = time::UtcOffset::local_offset_at(now).ok()?.whole_seconds();
        Self {
            seconds,
            offset,
            sign: offset.into(),
        }
        .into()
    }

    /// Return the current local time, or the one at UTC if the local time wasn't available.
    pub fn now_local_or_utc() -> Self {
        let now = time::OffsetDateTime::now_utc();
        let seconds = now.sub(std::time::SystemTime::UNIX_EPOCH).whole_seconds();
        // TODO: make this work without cfg(unsound_local_offset), see
        //       https://github.com/time-rs/time/issues/293#issuecomment-909158529
        let offset = time::UtcOffset::local_offset_at(now)
            .map(time::UtcOffset::whole_seconds)
            .unwrap_or(0);
        Self {
            seconds,
            offset,
            sign: offset.into(),
        }
    }
}
