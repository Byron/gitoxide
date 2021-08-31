use bstr::ByteSlice;
use git_actor::{Sign, Time};

#[test]
fn write_to() -> Result<(), Box<dyn std::error::Error>> {
    for (time, expected) in &[
        (
            Time {
                time: 500,
                offset: 9000,
                sign: Sign::Plus,
            },
            "500 +0230",
        ),
        (
            Time {
                time: 189009009,
                offset: 36000,
                sign: Sign::Minus,
            },
            "189009009 -1000",
        ),
        (
            Time {
                time: 0,
                offset: 0,
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
