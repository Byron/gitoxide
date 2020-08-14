mod streaming {
    use git_protocol::{
        packet_line::decode::{self, streaming, Stream},
        PacketLine,
    };

    fn assert_complete(
        res: Result<Stream, decode::Error>,
        expected_consumed: usize,
        expected_value: PacketLine,
    ) -> crate::Result {
        match res? {
            Stream::Complete { line, bytes_consumed } => {
                assert_eq!(bytes_consumed, expected_consumed);
                assert_eq!(line, expected_value);
            }
            Stream::Incomplete { .. } => panic!("expected parsing to be complete, not partial"),
        }
        Ok(())
    }

    #[test]
    fn flush() -> crate::Result {
        assert_complete(streaming(b"0000someotherstuff"), 4, PacketLine::Flush)
    }

    #[test]
    fn round_trips() -> crate::Result {
        for (line, bytes) in &[(PacketLine::Flush, 4), (PacketLine::Data(b"hello there"), 15)] {
            let mut out = Vec::new();
            line.to_write(&mut out)?;
            assert_complete(streaming(&out), *bytes, *line)?;
        }
        Ok(())
    }
}
