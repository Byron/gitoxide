use bstr::ByteSlice;
use git_packetline::PacketLine;
use std::io;
use std::path::PathBuf;

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from("tests/fixtures").join(path)
}

fn fixture_bytes(path: &str) -> Vec<u8> {
    std::fs::read(fixture_path(path)).expect("readable fixture")
}

mod to_read {
    use crate::packet_line::read::fixture_bytes;
    use bstr::ByteSlice;
    use git_odb::pack;
    use git_packetline::RemoteProgress;
    use std::io::Read;

    #[test]
    fn read_pack_with_progress_extraction() -> crate::Result {
        let buf = fixture_bytes("v1/01-clone.combined-output");
        let mut rd = git_packetline::Reader::new(&buf[..], None);

        // Read without sideband decoding
        let mut out = Vec::new();
        rd.as_read().read_to_end(&mut out)?;
        assert_eq!(out.as_bstr(), b"808e50d724f604f69ab93c6da2919c014667bedb HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master\n".as_bstr());

        rd.reset();
        assert_eq!(
            rd.read_line()
                .expect("line")??
                .to_text()
                .expect("data line")
                .0
                .as_bstr(),
            b"NAK".as_bstr()
        );
        fn no_parsing(_: &[u8]) -> Option<RemoteProgress> {
            None
        }
        let pack_read = rd.as_read_with_sidebands(git_features::progress::Discard, no_parsing);
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
            "150a1045f04dc0fc2dbf72313699fda696bf4126"
        );
        Ok(())
    }
}

fn first_line() -> PacketLine<'static> {
    PacketLine::Data(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n")
}
#[test]
fn peek_non_data() -> crate::Result {
    let mut rd = git_packetline::Reader::new(&b"000000010002"[..], Some(PacketLine::ResponseEnd));
    assert_eq!(rd.read_line().expect("line")??, PacketLine::Flush);
    assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
    rd.reset_with(PacketLine::Flush);
    assert_eq!(rd.read_line().expect("line")??, PacketLine::ResponseEnd);
    for _ in 0..2 {
        assert_eq!(
            rd.peek_line().expect("error").unwrap_err().kind(),
            std::io::ErrorKind::UnexpectedEof,
            "peeks on error/eof repeat the error"
        );
    }
    Ok(())
}

#[test]
fn fail_on_err_lines() -> crate::Result {
    let input = b"00010009ERR e0002";
    let mut rd = git_packetline::Reader::new(&input[..], None);
    assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
    assert_eq!(
        rd.read_line().expect("line")??.as_bstr(),
        Some(b"ERR e".as_bstr()),
        "by default no special handling"
    );

    let mut rd = git_packetline::Reader::new(&input[..], None);
    rd.fail_on_err_lines(true);
    assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
    assert_eq!(
        rd.read_line().expect("line").unwrap_err().to_string(),
        "e",
        "io errors are used to communicate remote errors"
    );
    assert!(rd.read_line().is_none(), "iteration is done after the first error");

    rd.replace(input);
    assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
    assert_eq!(
        rd.read_line().expect("line")??.as_bstr(),
        Some(b"ERR e".as_bstr()),
        "a reset also resets error handling to the default"
    );

    Ok(())
}

#[test]
fn peek() -> crate::Result {
    let bytes = fixture_bytes("v1/fetch/01-many-refs.response");
    let mut rd = git_packetline::Reader::new(&bytes[..], None);
    assert_eq!(rd.peek_line().expect("line")??, first_line(), "peek returns first line");
    assert_eq!(
        rd.peek_line().expect("line")??,
        first_line(),
        "peeked lines are never exhausted, unless they are finally read"
    );
    assert_eq!(
        rd.read_line().expect("line")??,
        first_line(),
        "read_line returns the peek once"
    );
    assert_eq!(
        rd.read_line().expect("line")??.as_bstr(),
        Some(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 refs/heads/master\n".as_bstr()),
        "the next read_line returns the next line"
    );
    assert_eq!(
        rd.peek_line().expect("line")??.as_bstr(),
        Some(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 refs/remotes/origin/HEAD\n".as_bstr()),
        "peek always gets the next line verbatim"
    );
    assert_eq!(exhaust(&mut rd), 1559);
    Ok(())
}

#[test]
fn read_from_file_and_reader_advancement() -> crate::Result {
    let mut bytes = fixture_bytes("v1/fetch/01-many-refs.response");
    bytes.extend(fixture_bytes("v1/fetch/01-many-refs.response").into_iter());
    let mut rd = git_packetline::Reader::new(&bytes[..], None);
    assert_eq!(rd.read_line().expect("line")??, first_line());
    assert_eq!(exhaust(&mut rd) + 1, 1561, "it stops after seeing the flush byte");
    rd.reset();
    assert_eq!(
        exhaust(&mut rd),
        1561,
        "it should read the second part of the identical file from the previously advanced reader"
    );

    // this reset is will cause actual io::Errors to occour
    rd.reset();
    assert_eq!(
        rd.read_line().expect("some error").unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof,
        "trying to keep reading from exhausted input results in Some() containing the original error"
    );
    Ok(())
}

fn exhaust(rd: &mut git_packetline::Reader<&[u8]>) -> i32 {
    let mut count = 0;
    while let Some(_) = rd.read_line() {
        count += 1;
    }
    count
}
