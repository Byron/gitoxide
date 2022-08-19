use bstr::ByteSlice;
use git_date::{time::Sign, Time};

#[test]
fn is_set() {
    assert!(!Time::default().is_set());
    assert!(Time {
        seconds_since_unix_epoch: 1,
        ..Default::default()
    }
    .is_set());
}

#[test]
fn write_to() -> Result<(), Box<dyn std::error::Error>> {
    for (time, expected) in &[
        (
            Time {
                seconds_since_unix_epoch: 500,
                offset_in_seconds: 9000,
                sign: Sign::Plus,
            },
            "500 +0230",
        ),
        (
            Time {
                seconds_since_unix_epoch: 189009009,
                offset_in_seconds: 36000,
                sign: Sign::Minus,
            },
            "189009009 -1000",
        ),
        (
            Time {
                seconds_since_unix_epoch: 0,
                offset_in_seconds: 0,
                sign: Sign::Minus,
            },
            "0 -0000",
        ),
    ] {
        let mut output = Vec::new();
        time.write_to(&mut output)?;
        assert_eq!(output.as_bstr(), expected);
    }
    Ok(())
}

mod format {
    use git_date::time::Sign;
    use git_date::Time;
    use time::macros::format_description;

    #[test]
    fn year_month_day() {
        assert_eq!(time().format(git_date::time::format::SHORT), "1973-11-29");
    }

    #[test]
    fn iso8601() {
        assert_eq!(
            time().format(git_date::time::format::ISO8601),
            "1973-11-29 21:33:09 +0230"
        );
    }

    #[test]
    fn rfc2822() {
        assert_eq!(
            time().format(git_date::time::format::RFC2822),
            "Thu, 29 Nov 1973 21:33:09 +0230"
        );
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
            time().format(&format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")),
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
}

mod parse {
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
}
