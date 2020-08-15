use git_protocol::{packet_line, PacketLine};
use std::{io, path::PathBuf};

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture_path(path)).expect("readable fixture")
}

#[test]
fn read_from_file_and_reader_advancement() -> crate::Result {
    let mut bytes = fixture_bytes("v1/fetch/01-many-refs.response");
    bytes.extend(fixture_bytes("v1/fetch/01-many-refs.response").into_iter());
    let mut rd = packet_line::Reader::new(&bytes[..]);
    let mut iter = rd.iter();
    assert_eq!(
        iter.next().expect("a line")??.as_bstr(),
        PacketLine::Data(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n").as_bstr()
    );
    assert_eq!(iter.count() + 1, 1561, "it stops after seeing the flush byte");
    iter = rd.iter();
    assert_eq!(
        iter.count(),
        1561,
        "it should read the second part of the identical file from the previously advanced reader"
    );
    iter = rd.iter();
    assert_eq!(
        iter.next().expect("some error").unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof,
        "trying to keep reading from exhausted input propagates the error"
    );
    Ok(())
}
