mod write_to {
    mod invalid {
        use gix_actor::{Sign, Signature, Time};

        #[test]
        fn name() {
            let signature = Signature {
                name: "invalid < middlename".into(),
                email: "ok".into(),
                time: default_time(),
            };
            assert_eq!(
                format!("{:?}", signature.write_to(Vec::new())),
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
                format!("{:?}", signature.write_to(Vec::new())),
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
                format!("{:?}", signature.write_to(Vec::new())),
                "Err(Custom { kind: Other, error: IllegalCharacter })"
            );
        }

        fn default_time() -> Time {
            Time {
                seconds_since_unix_epoch: 0,
                offset_in_seconds: 0,
                sign: Sign::Plus,
            }
        }
    }
}

use bstr::ByteSlice;
use gix_actor::Signature;

#[test]
fn trim() {
    let sig = gix_actor::SignatureRef::from_bytes::<()>(b" \t hello there \t < \t email \t > 1 -0030").unwrap();
    let sig = sig.trim();
    assert_eq!(sig.name, "hello there");
    assert_eq!(sig.email, "email");
}

#[test]
fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
    for input in &[
        &b"Sebastian Thiel <byronimo@gmail.com> 1 -0030"[..],
        ".. ☺️Sebastian 王知明 Thiel🙌 .. <byronimo@gmail.com> 1528473343 +0230".as_bytes(),
        ".. whitespace  \t  is explicitly allowed    - unicode aware trimming must be done elsewhere  <byronimo@gmail.com> 1528473343 +0230"
            .as_bytes(),
    ] {
        let signature: Signature = gix_actor::SignatureRef::from_bytes::<()>(input)?.into();
        let mut output = Vec::new();
        signature.write_to(&mut output)?;
        assert_eq!(output.as_bstr(), input.as_bstr());
    }
    Ok(())
}
