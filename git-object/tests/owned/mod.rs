mod tag {
    use crate::fixture_bytes;
    use bstr::ByteSlice;
    use git_object::{borrowed, owned};

    #[test]
    fn round_trip() {
        let input = fixture_bytes("tag/empty.txt");
        let tag: owned::Tag = borrowed::Tag::from_bytes(&input).unwrap().into();
        let mut output = Vec::new();
        tag.to_write(&mut output).unwrap();
        assert_eq!(input.as_bstr(), output.as_bstr());
    }
}
