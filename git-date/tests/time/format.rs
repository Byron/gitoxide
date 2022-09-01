use git_date::time::{format, Format, Sign};
use git_date::Time;
use time::macros::format_description;

#[test]
fn short() {
    assert_eq!(time().format(format::SHORT), "1973-11-29");
}

#[test]
fn unix() {
    let expected = "123456789";
    assert_eq!(time().format(Format::Unix), expected);
    assert_eq!(time().format(format::UNIX), expected);
}

#[test]
fn raw() {
    let expected = "123456789 +0230";
    assert_eq!(time().format(Format::Raw), expected);
    assert_eq!(time().format(format::RAW), expected);
}

#[test]
fn iso8601() {
    assert_eq!(time().format(format::ISO8601), "1973-11-29 21:33:09 +0230");
}

#[test]
fn iso8601_strict() {
    assert_eq!(time().format(format::ISO8601_STRICT), "1973-11-29T21:33:09+02:30");
}

#[test]
fn rfc2822() {
    assert_eq!(time().format(format::RFC2822), "Thu, 29 Nov 1973 21:33:09 +0230");
}

#[test]
fn default() {
    assert_eq!(
        time().format(git_date::time::format::DEFAULT),
        "Thu Nov 29 1973 21:33:09 +0230"
    );
}

#[test]
fn custom_compile_time() {
    assert_eq!(
        time().format(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")),
        "1973-11-29 21:33:09",
    );
}

fn time() -> Time {
    Time {
        seconds_since_unix_epoch: 123456789,
        offset_in_seconds: 9000,
        sign: Sign::Plus,
    }
}
