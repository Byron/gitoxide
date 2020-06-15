use super::Error;
use crate::Sign;

pub(crate) fn split2_at_space(
    d: &[u8],
    is_valid: impl FnOnce(&[u8], &[u8]) -> bool,
) -> Result<(&[u8], &[u8]), Error> {
    let mut t = d.splitn(2, |&b| b == b' ');
    Ok(match (t.next(), t.next()) {
        (Some(t1), Some(t2)) => {
            if !is_valid(t1, t2) {
                return Err(Error::ParseError(
                    "Invalid space separated tokens - validation failed",
                    d.to_owned(),
                ));
            }
            (t1, t2)
        }
        _ => {
            return Err(Error::ParseError(
                "Invalid tokens - expected 2 when split at space",
                d.to_owned(),
            ))
        }
    })
}

pub(crate) fn parse_timezone_offset(d: &str) -> Result<(i32, Sign), Error> {
    let db = d.as_bytes();
    if d.len() < 5 || !(db[0] == b'+' || db[0] == b'-') {
        return Err(Error::ParseError(
            "invalid timezone offset",
            d.as_bytes().to_owned(),
        ));
    }
    let sign = if db[0] == b'-' {
        Sign::Minus
    } else {
        Sign::Plus
    };
    let hours = std::str::from_utf8(&db[..3])
        .expect("valid utf8")
        .parse::<i32>()
        .map_err(|_| Error::ParseError("invalid 'hours' string", db[..3].to_owned()))?;
    let minutes = std::str::from_utf8(&db[3..])
        .expect("valid utf8")
        .parse::<i32>()
        .map_err(|_| Error::ParseError("invalid 'minutes' string", db[3..].to_owned()))?;
    Ok((hours * 3600 + minutes * 60, sign))
}
