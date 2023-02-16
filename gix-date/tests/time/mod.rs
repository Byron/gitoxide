use bstr::ByteSlice;
use git_date::{time::Sign, Time};

mod baseline;
mod format;
mod parse;
mod init {
    use git_date::Time;

    #[test]
    fn utc_local_handles_signs_correctly() {
        for time in [
            Time::now_local_or_utc(),
            Time::now_local().unwrap_or_else(Time::now_utc),
        ] {
            assert_eq!(
                time.sign,
                time.offset_in_seconds.into(),
                "the sign matches the sign of the date offset"
            );
        }
    }
}

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
