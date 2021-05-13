mod sideband;

mod streaming_peek_reader {
    use bstr::ByteSlice;
    use git_packetline::PacketLine;
    use std::{io, path::PathBuf};

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from("tests/fixtures").join(path)
    }

    pub fn fixture_bytes(path: &str) -> Vec<u8> {
        std::fs::read(fixture_path(path)).expect("readable fixture")
    }

    fn first_line() -> PacketLine<'static> {
        PacketLine::Data(b"7814e8a05a59c0cf5fb186661d1551c75d1299b5 HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed symref=HEAD:refs/heads/master object-format=sha1 agent=git/2.28.0\n")
    }

    #[test]
    fn peek_follows_read_line_delimiter_logic() -> crate::Result {
        let mut rd = git_packetline::StreamingPeekReader::new(&b"0005a00000005b"[..], &[PacketLine::Flush]);
        assert_eq!(rd.peek_line().expect("line")??, PacketLine::Data(b"a"));
        rd.read_line();
        assert!(rd.peek_line().is_none(), "we hit the delmiter, and thus are EOF");
        assert_eq!(
            rd.stopped_at(),
            Some(PacketLine::Flush),
            "Stopped tracking is done even when peeking"
        );
        assert!(rd.peek_line().is_none(), "we are still done, no way around it");
        rd.reset();
        assert_eq!(
            rd.peek_line().expect("line")??,
            PacketLine::Data(b"b"),
            "after resetting, we get past the delimiter"
        );
        Ok(())
    }

    #[test]
    fn peek_follows_read_line_err_logic() -> crate::Result {
        let mut rd = git_packetline::StreamingPeekReader::new(&b"0005a0009ERR e0000"[..], &[PacketLine::Flush]);
        rd.fail_on_err_lines(true);
        assert_eq!(rd.peek_line().expect("line")??, PacketLine::Data(b"a"));
        rd.read_line();
        assert_eq!(
            rd.peek_line().expect("line").unwrap_err().to_string(),
            "e",
            "io errors are used to communicate remote errors when peeking"
        );
        assert!(rd.peek_line().is_none(), "we are still done, no way around it");
        assert_eq!(rd.stopped_at(), None, "we stopped not because of a delimiter");
        rd.reset();
        assert!(rd.peek_line().is_none(), "it should stop due to the delimiter");
        assert_eq!(
            rd.stopped_at(),
            Some(PacketLine::Flush),
            "Stopped tracking is done even when peeking"
        );
        Ok(())
    }

    #[test]
    fn peek_non_data() -> crate::Result {
        let mut rd = git_packetline::StreamingPeekReader::new(&b"000000010002"[..], &[PacketLine::ResponseEnd]);
        assert_eq!(rd.read_line().expect("line")??, PacketLine::Flush);
        assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
        rd.reset_with(&[PacketLine::Flush]);
        assert_eq!(rd.read_line().expect("line")??, PacketLine::ResponseEnd);
        for _ in 0..2 {
            assert_eq!(
                rd.peek_line().expect("error").unwrap_err().kind(),
                std::io::ErrorKind::UnexpectedEof,
                "peeks on error/eof repeat the error"
            );
        }
        assert_eq!(
            rd.stopped_at(),
            None,
            "The reader is configured to ignore ResponseEnd, and thus hits the end of stream"
        );
        Ok(())
    }

    #[test]
    fn fail_on_err_lines() -> crate::Result {
        let input = b"00010009ERR e0002";
        let mut rd = git_packetline::StreamingPeekReader::new(&input[..], &[]);
        assert_eq!(rd.read_line().expect("line")??, PacketLine::Delimiter);
        assert_eq!(
            rd.read_line().expect("line")??.as_bstr(),
            Some(b"ERR e".as_bstr()),
            "by default no special handling"
        );

        let mut rd = git_packetline::StreamingPeekReader::new(&input[..], &[]);
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
            "a 'replace' also resets error handling to the default: false"
        );
        Ok(())
    }

    #[test]
    fn peek() -> crate::Result {
        let bytes = fixture_bytes("v1/fetch/01-many-refs.response");
        let mut rd = git_packetline::StreamingPeekReader::new(&bytes[..], &[PacketLine::Flush]);
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
        assert_eq!(
            rd.stopped_at(),
            Some(PacketLine::Flush),
            "A flush packet line ends every pack file"
        );
        Ok(())
    }

    #[test]
    fn read_from_file_and_reader_advancement() -> crate::Result {
        let mut bytes = fixture_bytes("v1/fetch/01-many-refs.response");
        bytes.extend(fixture_bytes("v1/fetch/01-many-refs.response").into_iter());
        let mut rd = git_packetline::StreamingPeekReader::new(&bytes[..], &[PacketLine::Flush]);
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

    fn exhaust(rd: &mut git_packetline::StreamingPeekReader<&[u8]>) -> i32 {
        let mut count = 0;
        while let Some(_) = rd.read_line() {
            count += 1;
        }
        count
    }
}
