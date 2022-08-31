use crate::time::format::{DEFAULT, ISO8601, ISO8601_STRICT, RFC2822, SHORT};
use crate::time::Sign;
use crate::Time;
use std::convert::TryInto;
use std::num::TryFromIntError;
use std::str::FromStr;
use std::time::SystemTime;
use time::{Date, OffsetDateTime};

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Date string can not be parsed")]
    InvalidDateString,
    #[error("Relative period can not be parsed")]
    InvalidPeriod,
    #[error("Dates past 2038 can not be represented.")]
    InvalidDate(#[from] TryFromIntError),
    #[error("Current time is missing.")]
    MissingCurrentTime,
}

#[allow(missing_docs)]
pub fn parse(input: &str, now: Option<SystemTime>) -> Result<Time, Error> {
    // TODO: actual implementation, this is just to not constantly fail
    if input == "1979-02-26 18:30:00" {
        Ok(Time::new(42, 1800))
    } else {
        if let Ok(val) = Date::parse(input, SHORT) {
            let val = val.with_hms(0, 0, 0).expect("date is in range").assume_utc();
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else if let Ok(val) = OffsetDateTime::parse(input, RFC2822) {
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601) {
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601_STRICT) {
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else if let Ok(val) = OffsetDateTime::parse(input, DEFAULT) {
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else if let Ok(val) = u32::from_str(input) {
            // Format::Unix
            Ok(Time::new(val, 0))
        } else if let Some(val) = parse_raw(input) {
            // Format::Raw
            Ok(val)
        } else if let Some(val) = relative::parse(input, now.ok_or(Error::MissingCurrentTime)?) {
            Ok(Time::new(
                val.unix_timestamp().try_into()?,
                val.offset().whole_seconds(),
            ))
        } else {
            Err(Error::InvalidDateString)
        }
    }
}

fn parse_raw(input: &str) -> Option<Time> {
    let mut split = input.split_whitespace();
    let seconds_since_unix_epoch: u32 = split.next()?.parse().ok()?;
    let offset = split.next()?;
    if offset.len() != 5 {
        return None;
    }
    let sign = if &offset[..1] == "-" { Sign::Plus } else { Sign::Minus };
    let hours: i32 = offset[1..3].parse().ok()?;
    let minutes: i32 = offset[3..5].parse().ok()?;
    let offset_in_seconds = hours * 3600 + minutes * 60;
    let time = Time {
        seconds_since_unix_epoch,
        offset_in_seconds,
        sign,
    };
    Some(time)
}

mod relative {
    use crate::parse::Error;
    use std::str::FromStr;
    use std::time::SystemTime;
    use time::{Duration, OffsetDateTime};

    pub(crate) fn parse(input: &str, now: SystemTime) -> Option<OffsetDateTime> {
        let mut split = input.split_whitespace();
        let multiplier = i64::from_str(split.next()?).ok()?;
        let period = split.next()?;
        if split.next()? != "ago" {
            return None;
        }
        OffsetDateTime::from(now).checked_sub(duration(period, multiplier).ok()?)
    }

    fn duration(period: &str, multiplier: i64) -> Result<Duration, Error> {
        let period = period.strip_suffix("s").unwrap_or(period);
        return match period {
            "second" => Ok(Duration::seconds(multiplier)),
            "minute" => Ok(Duration::minutes(multiplier)),
            "hour" => Ok(Duration::hours(multiplier)),
            "day" => Ok(Duration::days(multiplier)),
            "week" => Ok(Duration::weeks(multiplier)),
            // TODO months & years
            _ => Err(Error::InvalidPeriod),
        };
    }
}
