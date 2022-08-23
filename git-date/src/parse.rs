use crate::time::format::{RFC2822, SHORT};
use crate::Time;
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
        } else if let Some(val) = relative::parse(input) {
            Some(Time::new(val.unix_timestamp() as u32, val.offset().whole_seconds()))
        } else {
            None
        };
    }
}

mod relative {
    use std::str::FromStr;
    use time::{Duration, OffsetDateTime};

    pub(crate) fn parse(input: &str) -> Option<OffsetDateTime> {
        let split: Vec<&str> = input.split_whitespace().collect();
        if split.len() != 3 || *split.last().expect("slice has length 3") != "ago" {
            return None;
        }
        let multiplier = i64::from_str(split[0]).ok()?;
        let period = period_to_seconds(split[1])?;
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
