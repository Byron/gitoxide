use bstr::ByteSlice;
use git_date::{time::Sign, Time};

mod format;
mod parse;

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
