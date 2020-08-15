fn assert_err_display<T: std::fmt::Debug, E: std::error::Error>(res: Result<T, E>, expected: impl AsRef<str>) {
    match res {
        Ok(v) => assert!(false, "Expected error '{}', got value {:?}", expected.as_ref(), v),
        Err(err) => assert_eq!(err.to_string(), expected.as_ref()),
    }
}

mod read {
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
        assert_eq!(
            rd.read_line().expect("a line")??.as_bstr(),
            PacketLine::Data(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n").as_bstr()
        );
        assert_eq!(exhaust(&mut rd) + 1, 1561, "it stops after seeing the flush byte");
        let mut rd = packet_line::Reader::new(rd.inner);
        assert_eq!(
            exhaust(&mut rd),
            1561,
            "it should read the second part of the identical file from the previously advanced reader"
        );
        let mut rd = packet_line::Reader::new(rd.inner);
        assert_eq!(
            rd.read_line().expect("some error").unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof,
            "trying to keep reading from exhausted input propagates the error"
        );
        Ok(())
    }

    fn exhaust(rd: &mut packet_line::Reader<&[u8]>) -> i32 {
        let mut count = 0;
        while let Some(_) = rd.read_line() {
            count += 1;
        }
        count
    }
}
mod decode;
mod encode;
