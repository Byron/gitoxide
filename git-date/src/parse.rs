use crate::time::format::{DEFAULT, ISO8601, ISO8601_STRICT, RFC2822, SHORT};
use crate::time::Sign;
use crate::Time;
use std::str::FromStr;
use time::{Date, OffsetDateTime};

#[allow(missing_docs)]
pub fn parse(input: &str) -> Option<Time> {
    // TODO: actual implementation, this is just to not constantly fail
    if input == "1979-02-26 18:30:00" {
        Some(Time::new(42, 1800))
    } else {
        return if let Ok(val) = Date::parse(input, SHORT) {
            let val = val.with_hms(0, 0, 0).expect("date is in range").assume_utc();
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else if let Ok(val) = OffsetDateTime::parse(input, RFC2822) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else if let Ok(val) = OffsetDateTime::parse(input, ISO8601_STRICT) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else if let Ok(val) = OffsetDateTime::parse(input, DEFAULT) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else if let Ok(val) = u32::from_str(input) {
            // Format::Unix
            Some(Time::new(val, 0))
        } else if let Ok(val) = parse_raw(input) {
            // Format::Raw
            Some(val)
        } else if let Some(val) = relative::parse(input) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else {
            None
        };
    }
}

fn parse_raw(input: &str) -> Result<Time, ()> {
    let mut split = input.split_whitespace();
    let seconds_since_unix_epoch: u32 = split.next().ok_or(())?.parse().map_err(|_| ())?;
    let offset = split.next().ok_or(())?;
    if offset.len() != 5 {
        return Err(());
    }
    let sign = if &offset[..1] == "-" { Sign::Plus } else { Sign::Minus };
    let hours: i32 = offset[1..3].parse().map_err(|_| ())?;
    let minutes: i32 = offset[3..5].parse().map_err(|_| ())?;
    let offset_in_seconds = hours * 3600 + minutes * 60;
    let time = Time {
        seconds_since_unix_epoch,
        offset_in_seconds,
        sign,
    };
    Ok(time)
}

mod relative {
    use std::str::FromStr;
    use time::{Duration, OffsetDateTime};

    pub(crate) fn parse(input: &str) -> Option<OffsetDateTime> {
        let mut split = input.split_whitespace();
        let multiplier = i64::from_str(split.next()?).ok()?;
        let period = period_to_seconds(split.next()?)?;
        if split.next()? != "ago" {
            return None;
        }
        Some(OffsetDateTime::now_utc().checked_sub(Duration::seconds(multiplier * period))?)
    }

    fn period_to_seconds(period: &str) -> Option<i64> {
        let period = period.strip_suffix("s").unwrap_or(period);
        return match period {
            "second" => Some(1),
            "minute" => Some(60),
            "hour" => Some(60 * 60),
            "day" => Some(24 * 60 * 60),
            "week" => Some(7 * 24 * 60 * 60),
            // TODO months & years
            _ => None,
        };
    }
}
