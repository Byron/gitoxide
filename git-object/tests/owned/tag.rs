use crate::fixture_bytes;
use bstr::ByteSlice;
use git_object::{borrowed, owned};

// Git checks out text files with different line feeds, which causes parsing failure.
// No way to configure this in the checkout action :/
#[cfg_attr(windows, ignore)]
#[test]
fn round_trip() {
    for input in &[
        "tag/empty.txt",
        "tag/whitespace.txt",
        "tag/with-newlines.txt",
        "tag/signed.txt",
    ] {
        let input = fixture_bytes(input);
        let tag: owned::Tag = borrowed::Tag::from_bytes(&input).unwrap().into();
        let mut output = Vec::new();
        tag.write_to(&mut output).unwrap();
        assert_eq!(input.as_bstr(), output.as_bstr());
    }
}
