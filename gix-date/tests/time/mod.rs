use bstr::ByteSlice;
use gix_date::{time::Sign, SecondsSinceUnixEpoch, Time};

mod baseline;
mod format;
mod parse;
mod init {
    use gix_date::Time;

    #[test]
    fn utc_local_handles_signs_correctly() {
        for time in [
            Time::now_local_or_utc(),
            Time::now_local().unwrap_or_else(Time::now_utc),
        ] {
            assert_eq!(
                time.sign,
                time.offset.into(),
                "the sign matches the sign of the date offset"
            );
        }
    }
}

#[test]
fn is_set() {
    assert!(!Time::default().is_set());
    assert!(Time {
        seconds: 1,
        ..Default::default()
    }
    .is_set());
}

#[test]
fn write_to() -> Result<(), Box<dyn std::error::Error>> {
    for (time, expected) in [
        (
            Time {
                seconds: SecondsSinceUnixEpoch::MAX,
                offset: 0,
                sign: Sign::Minus,
            },
            "9223372036854775807 -0000",
        ),
        (
            Time {
                seconds: SecondsSinceUnixEpoch::MIN,
                offset: 0,
                sign: Sign::Minus,
            },
            "-9223372036854775808 -0000",
        ),
        (
            Time {
                seconds: 500,
                offset: 9000,
                sign: Sign::Plus,
            },
            "500 +0230",
        ),
        (
            Time {
                seconds: 189009009,
                offset: -36000,
                sign: Sign::Minus,
            },
            "189009009 -1000",
        ),
        (
            Time {
                seconds: 0,
                offset: 0,
                sign: Sign::Minus,
            },
            "0 -0000",
        ),
    ] {
        let mut output = Vec::new();
        time.write_to(&mut output)?;
        assert_eq!(output.as_bstr(), expected);
        assert_eq!(time.size(), output.len());

        let actual = gix_date::parse(&output.as_bstr().to_string(), None).expect("round-trippable");
        assert_eq!(time, actual);
    }
    Ok(())
}
