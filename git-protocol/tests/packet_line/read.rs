use git_protocol::{packet_line, PacketLine};
use std::{io, path::PathBuf};

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture_path(path)).expect("readable fixture")
}

mod to_read {
    use crate::packet_line::read::fixture_bytes;
    use git_odb::pack;
    use git_protocol::packet_line;

    #[test]
    fn read_pack_with_progress_extraction() -> crate::Result {
        let buf = fixture_bytes("v1/01-clone.combined-output");
        let mut rd = packet_line::Reader::new(&buf[..], None);
        let pack_read = rd.to_read_with_sidebands(git_features::progress::Discard);
        let pack_entries = pack::data::Iter::new_from_header(
            pack_read,
            pack::data::iter::Mode::Verify,
            pack::data::iter::CompressedBytesMode::Ignore,
        )?;
        let size = pack_entries.size_hint().0 - 1;
        let last = pack_entries.skip(size).next().expect("last entry")?;
        assert_eq!(
            last.trailer
                .expect("trailer to exist on last entry")
                .to_sha1_hex_string(),
            "foo"
        );
        Ok(())
    }
}

#[test]
fn read_from_file_and_reader_advancement() -> crate::Result {
    let mut bytes = fixture_bytes("v1/fetch/01-many-refs.response");
    bytes.extend(fixture_bytes("v1/fetch/01-many-refs.response").into_iter());
    let mut rd = packet_line::Reader::new(&bytes[..], None);
    assert_eq!(
        rd.read_line()??.as_bstr(),
        PacketLine::Data(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n").as_bstr()
    );
    assert_eq!(exhaust(&mut rd) + 1, 1561, "it stops after seeing the flush byte");
    rd.reset();
    assert_eq!(
        exhaust(&mut rd),
        1561,
        "it should read the second part of the identical file from the previously advanced reader"
    );
    rd.reset();
    assert_eq!(
        rd.read_line().unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof,
        "trying to keep reading from exhausted input propagates the error"
    );
    Ok(())
}

fn exhaust(rd: &mut packet_line::Reader<&[u8]>) -> i32 {
    let mut count = 0;
    while let Ok(_) = rd.read_line() {
        count += 1;
    }
    count
}
