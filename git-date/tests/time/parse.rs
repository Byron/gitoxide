use git_date::time::Sign;
use git_date::Time;
use time::OffsetDateTime;

#[test]
fn special_time_is_ok_for_now() {
    assert_eq!(
        git_date::parse("1979-02-26 18:30:00").unwrap(),
        Time {
            seconds_since_unix_epoch: 42,
            offset_in_seconds: 1800,
            sign: Sign::Plus,
        }
    );
}

#[test]
fn short() {
    assert_eq!(
        git_date::parse("1979-02-26"),
        Some(Time {
            seconds_since_unix_epoch: 288835200,
            offset_in_seconds: 0,
            sign: Sign::Plus,
        }),
        "could not parse with SHORT format"
    );
}

#[test]
fn rfc2822() {
    assert_eq!(
        git_date::parse("Thu, 18 Aug 2022 12:45:06 +0800"),
        Some(Time {
            seconds_since_unix_epoch: 1660797906,
            offset_in_seconds: 28800,
            sign: Sign::Plus,
        }),
        "could not parse with RFC2822 format"
    );
}

#[test]
fn relative() {
    let two_weeks_ago = git_date::parse("2 weeks ago").expect("valid time");
    assert_eq!(Sign::Plus, two_weeks_ago.sign);
    assert_eq!(0, two_weeks_ago.offset_in_seconds);
    assert_eq!(
        OffsetDateTime::from_unix_timestamp(two_weeks_ago.seconds_since_unix_epoch as i64)
            .expect("valid datetime")
            .iso_week(),
        OffsetDateTime::now_utc().iso_week() - 2,
        "weeks numbers differ"
    );
}
