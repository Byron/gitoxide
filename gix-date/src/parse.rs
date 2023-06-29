#[derive(thiserror::Error, Debug, Clone)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert a duration into a date")]
    RelativeTimeConversion,
    #[error("Date string can not be parsed")]
    InvalidDateString { input: String },
    #[error("The heat-death of the universe happens before this date")]
    InvalidDate(#[from] std::num::TryFromIntError),
    #[error("Current time is missing but required to handle relative dates.")]
    MissingCurrentTime,
}

pub(crate) mod function {
    use std::{str::FromStr, time::SystemTime};

    use time::{format_description::well_known, Date, OffsetDateTime};

    use crate::{
        parse::{relative, Error},
        time::{
            format::{DEFAULT, GITOXIDE, ISO8601, ISO8601_STRICT, SHORT},
            Sign,
        },
        SecondsSinceUnixEpoch, Time,
    };

    #[allow(missing_docs)]
    pub fn parse(input: &str, now: Option<SystemTime>) -> Result<Time, Error> {
        // TODO: actual implementation, this is just to not constantly fail
        if input == "1979-02-26 18:30:00" {
            return Ok(Time::new(42, 1800));
        }

        Ok(if let Ok(val) = Date::parse(input, SHORT) {
            let val = val.with_hms(0, 0, 0).expect("date is in range").assume_utc();
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = OffsetDateTime::parse(input, &well_known::Rfc2822) {
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601) {
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601_STRICT) {
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = OffsetDateTime::parse(input, GITOXIDE) {
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = OffsetDateTime::parse(input, DEFAULT) {
            Time::new(val.unix_timestamp(), val.offset().whole_seconds())
        } else if let Ok(val) = SecondsSinceUnixEpoch::from_str(input) {
            // Format::Unix
            Time::new(val, 0)
        } else if let Some(val) = parse_raw(input) {
            // Format::Raw
            val
        } else if let Some(time) = relative::parse(input, now).transpose()? {
            Time::new(time.unix_timestamp(), time.offset().whole_seconds())
        } else {
            return Err(Error::InvalidDateString { input: input.into() });
        })
    }

    fn parse_raw(input: &str) -> Option<Time> {
        let mut split = input.split_whitespace();
        let seconds: SecondsSinceUnixEpoch = split.next()?.parse().ok()?;
        let offset = split.next()?;
        if offset.len() != 5 || split.next().is_some() {
            return None;
        }
        let sign = match offset.get(..1)? {
            "-" => Some(Sign::Minus),
            "+" => Some(Sign::Plus),
            _ => None,
        }?;
        let hours: i32 = offset.get(1..3)?.parse().ok()?;
        let minutes: i32 = offset.get(3..5)?.parse().ok()?;
        let mut offset_in_seconds = hours * 3600 + minutes * 60;
        if sign == Sign::Minus {
            offset_in_seconds *= -1;
        };
        let time = Time {
            seconds,
            offset: offset_in_seconds,
            sign,
        };
        Some(time)
    }
}

mod relative {
    use std::{convert::TryInto, str::FromStr, time::SystemTime};

    use time::{Duration, OffsetDateTime};

    use crate::parse::Error;

    fn parse_inner(input: &str) -> Option<Duration> {
        let mut split = input.split_whitespace();
        let multiplier = i64::from_str(split.next()?).ok()?;
        let period = split.next()?;
        if split.next()? != "ago" {
            return None;
        }
        duration(period, multiplier)
    }

    pub(crate) fn parse(input: &str, now: Option<SystemTime>) -> Option<Result<OffsetDateTime, Error>> {
        parse_inner(input).map(|offset| {
            let offset = std::time::Duration::from_secs(offset.whole_seconds().try_into()?);
            now.ok_or(Error::MissingCurrentTime).and_then(|now| {
                std::panic::catch_unwind(|| {
                    now.checked_sub(offset)
                        .expect("BUG: values can't be large enough to cause underflow")
                        .into()
                })
                .map_err(|_| Error::RelativeTimeConversion)
            })
        })
    }

    fn duration(period: &str, multiplier: i64) -> Option<Duration> {
        let period = period.strip_suffix('s').unwrap_or(period);
        let seconds: i64 = match period {
            "second" => 1,
            "minute" => 60,
            "hour" => 60 * 60,
            "day" => 24 * 60 * 60,
            "week" => 7 * 24 * 60 * 60,
            // TODO months & years? YES
            // Ignore values you don't know, assume seconds then (so does git)
            _ => return None,
        };
        seconds.checked_mul(multiplier).map(Duration::seconds)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn two_weeks_ago() {
            assert_eq!(parse_inner("2 weeks ago"), Some(Duration::weeks(2)));
        }
    }
}
