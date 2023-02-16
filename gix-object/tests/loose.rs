use bstr::ByteSlice;
use gix_object::{decode, encode, Kind, ObjectRef};

#[test]
fn all() -> Result<(), Box<dyn std::error::Error>> {
    for (kind, size, expected) in &[
        (Kind::Tree, 1234, "tree 1234\0".as_bytes()),
        (Kind::Blob, 0, b"blob 0\0"),
        (Kind::Commit, 24241, b"commit 24241\0"),
        (Kind::Tag, 9999999999, b"tag 9999999999\0"),
    ] {
        let buf = encode::loose_header(*kind, *size);
        assert_eq!(buf.as_bstr(), expected.as_bstr());
        let (actual_kind, actual_size, actual_read) = decode::loose_header(&buf)?;
        assert_eq!(actual_kind, *kind);
        assert_eq!(actual_size, *size);
        assert_eq!(actual_read, buf.len());
    }
    Ok(())
}

#[test]
fn shorter_than_advertised() {
    assert_eq!(
        ObjectRef::from_loose(b"tree 1000\x00").unwrap_err().to_string(),
        "object data was shorter than its size declared in the header"
    );
}
