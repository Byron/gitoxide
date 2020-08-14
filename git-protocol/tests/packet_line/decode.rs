mod streaming {
    use git_protocol::{
        packet_line::decode::{streaming, ParseResult},
        PacketLine,
    };

    fn assert_complete(res: ParseResult, expected_consumed: usize, expected_value: PacketLine) -> crate::Result {
        match res {
            ParseResult::Complete { result, bytes_consumed } => {
                assert_eq!(bytes_consumed, expected_consumed);
                assert_eq!(result?, expected_value);
            }
            ParseResult::Incomplete { .. } => panic!("expected parsing to be complete, not partial"),
        }
        Ok(())
    }

    #[test]
    fn flush() -> crate::Result {
        assert_complete(streaming(b"0000someotherstuff"), 4, PacketLine::Flush)
    }
}
