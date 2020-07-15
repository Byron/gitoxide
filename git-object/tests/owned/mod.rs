mod object {
    mod time {
        use bstr::ByteSlice;
        use git_object::{Sign, Time};

        #[test]
        fn write_to() {
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
                time.write_to(&mut output).unwrap();
                assert_eq!(output.as_bstr(), expected);
            }
        }
    }

    mod signature {
        use bstr::ByteSlice;
        use git_object::{borrowed, owned};

        #[test]
        fn round_trip() {
            for input in &[&b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230"[..]] {
                let signature: owned::Signature = borrowed::Signature::from_bytes(input).unwrap().into();
                let mut output = Vec::new();
                signature.write_to(&mut output).unwrap();
                assert_eq!(input.as_bstr(), output.as_bstr());
            }
        }
    }
}

mod tag {
    use crate::fixture_bytes;
    use bstr::ByteSlice;
    use git_object::{borrowed, owned};

    #[test]
    fn round_trip() {
        let input = fixture_bytes("tag/empty.txt");
        let tag: owned::Tag = borrowed::Tag::from_bytes(&input).unwrap().into();
        let mut output = Vec::new();
        tag.write_to(&mut output).unwrap();
        assert_eq!(input.as_bstr(), output.as_bstr());
    }
}
