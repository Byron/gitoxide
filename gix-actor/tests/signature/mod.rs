mod write_to {
    mod invalid {
        use gix_actor::Signature;
        use gix_date::{time::Sign, Time};

        #[test]
        fn name() {
            let signature = Signature {
                name: "invalid < middlename".into(),
                email: "ok".into(),
                time: default_time(),
            };
            assert_eq!(
                format!("{:?}", signature.write_to(&mut Vec::new())),
                "Err(Custom { kind: Other, error: IllegalCharacter })"
            );
        }

        #[test]
        fn email() {
            let signature = Signature {
                name: "ok".into(),
                email: "server>.example.com".into(),
                time: default_time(),
            };
            assert_eq!(
                format!("{:?}", signature.write_to(&mut Vec::new())),
                "Err(Custom { kind: Other, error: IllegalCharacter })"
            );
        }

        #[test]
        fn name_with_newline() {
            let signature = Signature {
                name: "hello\nnewline".into(),
                email: "name@example.com".into(),
                time: default_time(),
            };
            assert_eq!(
                format!("{:?}", signature.write_to(&mut Vec::new())),
                "Err(Custom { kind: Other, error: IllegalCharacter })"
            );
        }

        fn default_time() -> Time {
            Time {
                seconds: 0,
                offset: 0,
                sign: Sign::Plus,
            }
        }
    }
}

use bstr::ByteSlice;
use gix_actor::{Signature, SignatureRef};

#[test]
fn trim() {
    let sig = gix_actor::SignatureRef::from_bytes::<()>(b" \t hello there \t < \t email \t > 1 -0030").unwrap();
    let sig = sig.trim();
    assert_eq!(sig.name, "hello there");
    assert_eq!(sig.email, "email");
}

#[test]
fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
    static DEFAULTS: &[&[u8]] =     &[
        b"Sebastian Thiel <byronimo@gmail.com> 1 -0030",
        b"Sebastian Thiel <byronimo@gmail.com> -1500 -0030",
        ".. ☺️Sebastian 王知明 Thiel🙌 .. <byronimo@gmail.com> 1528473343 +0230".as_bytes(),
        b".. whitespace  \t  is explicitly allowed    - unicode aware trimming must be done elsewhere  <byronimo@gmail.com> 1528473343 +0230"
    ];

    for input in DEFAULTS {
        let signature: Signature = gix_actor::SignatureRef::from_bytes::<()>(input).unwrap().into();
        let mut output = Vec::new();
        signature.write_to(&mut output)?;
        assert_eq!(output.as_bstr(), input.as_bstr());
    }
    Ok(())
}

#[test]
fn parse_timestamp_with_trailing_digits() {
    let signature = gix_actor::SignatureRef::from_bytes::<()>(b"first last <name@example.com> 1312735823 +051800")
        .expect("deal with trailing zeroes in timestamp by discarding it");
    assert_eq!(
        signature,
        SignatureRef {
            name: "first last".into(),
            email: "name@example.com".into(),
            time: gix_actor::date::Time::new(1312735823, 0),
        }
    );

    let signature = gix_actor::SignatureRef::from_bytes::<()>(b"first last <name@example.com> 1312735823 +0518")
        .expect("this naturally works as the timestamp does not have trailing zeroes");
    assert_eq!(
        signature,
        SignatureRef {
            name: "first last".into(),
            email: "name@example.com".into(),
            time: gix_actor::date::Time::new(1312735823, 19080),
        }
    );
}
