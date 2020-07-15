mod tag {
    use crate::fixture_bytes;
    use git_object::{borrowed, owned};

    #[test]
    fn round_trip() {
        let tag: owned::Tag = borrowed::Tag::from_bytes(&fixture_bytes("tag/empty.txt"))
            .unwrap()
            .into();
    }
}
