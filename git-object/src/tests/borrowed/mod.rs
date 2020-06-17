use std::path::PathBuf;

mod tag;

fn fixture_bytes(path: &str) -> Vec<u8> {
    super::fixture_bytes(PathBuf::from("tag").join(path).to_str().unwrap())
}

mod parse_signature {
    use crate::borrowed::util::parse_signature;
    use crate::borrowed::Signature;
    use crate::{Sign, Time};
    use bstr::ByteSlice;

    fn signature(
        name: &'static str,
        email: &'static str,
        time: u32,
        sign: Sign,
        offset: i32,
    ) -> Signature<'static> {
        Signature {
            name: name.as_bytes().as_bstr(),
            email: email.as_bytes().as_bstr(),
            time: Time { time, offset, sign },
        }
    }

    #[test]
    fn tz_minus() {
        assert_eq!(
            parse_signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0230")
                .unwrap()
                .1,
            signature(
                "Sebastian Thiel",
                "byronimo@gmail.com",
                1528473343,
                Sign::Minus,
                -9000
            )
        );
    }

    #[test]
    fn tz_plus() {
        assert_eq!(
            parse_signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
                .unwrap()
                .1,
            signature(
                "Sebastian Thiel",
                "byronimo@gmail.com",
                1528473343,
                Sign::Plus,
                9000
            )
        );
    }

    #[test]
    fn negative_offset_0000() {
        assert_eq!(
            parse_signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0000")
                .unwrap()
                .1,
            signature(
                "Sebastian Thiel",
                "byronimo@gmail.com",
                1528473343,
                Sign::Minus,
                0
            )
        );
    }

    #[test]
    fn empty_name_and_email() {
        assert_eq!(
            parse_signature(b" <> 12345 -1215").unwrap().1,
            signature("", "", 12345, Sign::Minus, -44100)
        );
    }
}
