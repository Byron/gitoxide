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
        assert_eq!(time().format(git_date::time::format::SHORT), "1970-01-01");
    }

    #[test]
    fn custom_compile_time() {
        assert_eq!(
            time().format(&format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")),
            "1970-01-01 00:08:20",
        );
    }

    fn time() -> Time {
        Time {
            seconds_since_unix_epoch: 500,
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
            git_date::parse("1979-02-26 18:30:00".into()).unwrap(),
            Time {
                seconds_since_unix_epoch: 42,
                offset_in_seconds: 1800,
                sign: Sign::Plus,
            }
        );
    }
}
