use git_date::time::Sign;
use git_date::Time;

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
