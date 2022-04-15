use git_config::values::expiry_date::ExpiryDate;
use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

fn b(v: &str) -> Cow<[u8]> {
    Cow::from(v.as_bytes())
}

#[test]
fn test_exp() {
    let relative = "1 week 5 days 01:20";
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

    let secs_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let secs_relative = 1_036_800 + 3_600 + 1_200;
    assert_eq!(
        ExpiryDate::from(b(relative)).to_timestamp().unwrap(),
        secs_now - secs_relative
    );

    // assert_eq!(ExpiryDate::from(b(local)).to_timestamp().unwrap(), 1275659215);
    assert_eq!(ExpiryDate::from(b(local)).to_timestamp().unwrap(), 1275666415);

    // assert_eq!(ExpiryDate::from(b(valid3)).to_timestamp().unwrap(), 1510438271);
    assert_eq!(ExpiryDate::from(b(valid3)).to_timestamp().unwrap(), 1510441871);

    // assert_eq!(ExpiryDate::from(b(valid4)).to_timestamp().unwrap(), 1510344487);
    assert_eq!(ExpiryDate::from(b(valid4)).to_timestamp().unwrap(), 1510348087);

    assert_eq!(ExpiryDate::from(b(never)).to_timestamp().unwrap(), 0);

    assert_eq!(ExpiryDate::from(b(false_)).to_timestamp().unwrap(), 0);

    assert_eq!(ExpiryDate::from(b(all)).to_timestamp().unwrap(), 18446744073709551615);

    assert_eq!(ExpiryDate::from(b(now)).to_timestamp().unwrap(), 18446744073709551615);

    // not in git tests
    assert_eq!(ExpiryDate::from(b(rfc2822)).to_timestamp().unwrap(), 1275652015);

    assert_eq!(ExpiryDate::from(b(iso8601)).to_timestamp().unwrap(), 1151939923);

    assert_eq!(ExpiryDate::from(b(localtz)).to_timestamp().unwrap(), 1275655615);
}
