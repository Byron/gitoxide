mod object {
    mod signature {
        mod to_write {
            use bstr::ByteSlice;
            use git_object::{borrowed, owned};

            #[test]
            fn round_trip() {
                let input = b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230";
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
