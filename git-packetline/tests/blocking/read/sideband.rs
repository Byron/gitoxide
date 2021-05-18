use crate::read::streaming_peek_iter::fixture_bytes;
use bstr::{BString, ByteSlice};
use git_odb::pack;
use git_packetline::PacketLine;
use std::io::{BufRead, Read};

#[test]
fn read_line_trait_method_reads_one_packet_line_at_a_time() -> crate::Result {
    let buf = fixture_bytes("v1/01-clone.combined-output-no-binary");

    let mut rd = git_packetline::StreamingPeekableIter::new(&buf[..], &[PacketLine::Flush]);

    let mut out = String::new();
    let mut r = rd.as_read();
    r.read_line(&mut out)?;
    assert_eq!(out, "808e50d724f604f69ab93c6da2919c014667bedb HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n");
    out.clear();
    r.read_line(&mut out)?;
    assert_eq!(out, "808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master\n");
    out.clear();
    r.read_line(&mut out)?;
    assert_eq!(out, "", "flush means empty lines…");
    out.clear();
    r.read_line(&mut out)?;
    assert_eq!(out, "", "…which can't be overcome unless the reader is reset");
    assert_eq!(
        r.stopped_at(),
        Some(PacketLine::Flush),
        "it knows what stopped the reader"
    );

    drop(r);
    rd.reset();

    let mut r = rd.as_read();
    r.read_line(&mut out)?;
    assert_eq!(out, "NAK\n");

    drop(r);
    let mut r = rd.as_read_with_sidebands(|_, _| ());
    let mut assert_next = |line: &str| -> crate::Result {
        out.clear();
        r.read_line(&mut out)?;
        assert_eq!(out, line);
        Ok(())
    };
    assert_next("&")?;
    assert_next("")?;
    Ok(())
}

#[test]
fn read_pack_with_progress_extraction() -> crate::Result {
    let buf = fixture_bytes("v1/01-clone.combined-output");
    let mut rd = git_packetline::StreamingPeekableIter::new(&buf[..], &[PacketLine::Flush]);

    // Read without sideband decoding
    let mut out = Vec::new();
    rd.as_read().read_to_end(&mut out)?;
    assert_eq!(out.as_bstr(), b"808e50d724f604f69ab93c6da2919c014667bedb HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n808e50d724f604f69ab93c6da2919c014667bedb refs/heads/master\n".as_bstr());

    assert_eq!(
        rd.read_line()
            .expect("line")??
            .to_text()
            .expect("data line")
            .0
            .as_bstr(),
        b"NAK".as_bstr()
    );
    let mut seen_texts = Vec::<BString>::new();
    let mut do_nothing = |is_err: bool, data: &[u8]| {
        assert!(!is_err);
        seen_texts.push(data.as_bstr().into());
    };
    let pack_read = rd.as_read_with_sidebands(&mut do_nothing);
    let pack_entries = pack::data::BytesToEntriesIter::new_from_header(
        pack_read,
        pack::data::input::Mode::Verify,
        pack::data::input::EntryDataMode::Ignore,
    )?;
    let size = pack_entries.size_hint().0 - 1;
    let last = pack_entries.skip(size).next().expect("last entry")?;
    assert_eq!(
        last.trailer
            .expect("trailer to exist on last entry")
            .to_sha1_hex_string(),
        "150a1045f04dc0fc2dbf72313699fda696bf4126"
    );
    assert_eq!(
        seen_texts,
        [
            "Enumerating objects: 3, done.",
            "Counting objects:  33% (1/3)\r",
            "Counting objects:  66% (2/3)\r",
            "Counting objects: 100% (3/3)\r",
            "Counting objects: 100% (3/3), done.",
            "Total 3 (delta 0), reused 0 (delta 0), pack-reused 0"
        ]
        .iter()
        .map(|v| v.as_bytes().as_bstr().to_owned())
        .collect::<Vec<_>>()
    );
    Ok(())
}
#[test]
fn peek_past_an_actual_eof_is_an_error() -> crate::Result {
    let input = b"0009ERR e";
    let mut rd = git_packetline::StreamingPeekableIter::new(&input[..], &[]);
    let mut reader = rd.as_read();
    assert_eq!(reader.peek_data_line().expect("one line")??, b"ERR e");
    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    assert_eq!(
        reader.peek_data_line().expect("an err").expect_err("foo").kind(),
        std::io::ErrorKind::UnexpectedEof,
        "peeking past the end is not an error as the caller should make sure we dont try 'invalid' reads"
    );
    Ok(())
}
