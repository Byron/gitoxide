use chrono::{DateTime, TimeZone, Utc};
use git_config::values::ExpiryDate;
use nom::AsBytes;
use std::borrow::Cow;

#[test]
fn test_exp() {
    let relative = "3.weeks.5.days 00:00";
    let local = "Fri Jun 4 15:46:55 2010";
    let valid3 = "2017/11/11 11:11:11PM";
    let valid4 = "2017/11/10 09:08:07 PM";
    let never = "never";
    let false_ = "false";
    let all = "all";
    let now = "now";
    let rfc2822 = "Fri, 4 Jun 2010 15:46:55 +0400";
    let iso8601 = "2006-07-03 17:18:43 +0200";
    let localtz = "Fri Jun 4 15:46:55 2010 +0300";

    let date = ExpiryDate::from(Cow::from(local.as_bytes()));
    // TODO check why git returns 1275659215
    assert_eq!(date.to_timestamp().unwrap(), 1275666415);

    let date = ExpiryDate::from(Cow::from(valid3.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 1510441871);

    let date = ExpiryDate::from(Cow::from(valid4.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 1510348087);

    let date = ExpiryDate::from(Cow::from(never.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 0);

    let date = ExpiryDate::from(Cow::from(false_.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 0);

    let date = ExpiryDate::from(Cow::from(all.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 18446744073709551615);

    let date = ExpiryDate::from(Cow::from(now.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 18446744073709551615);

    // not in git tests
    let date = ExpiryDate::from(Cow::from(rfc2822.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 1275652015);

    let date = ExpiryDate::from(Cow::from(iso8601.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 1151939923);

    let date = ExpiryDate::from(Cow::from(localtz.as_bytes()));
    assert_eq!(date.to_timestamp().unwrap(), 1275655615);
}
