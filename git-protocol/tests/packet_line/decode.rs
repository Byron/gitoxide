mod streaming {
    use crate::packet_line::assert_err_display;
    use git_protocol::packet_line::Channel;
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
                assert_eq!(line.as_bstr(), expected_value.as_bstr());
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
    fn trailing_line_feeds_are_removed() -> crate::Result {
        assert_complete(streaming(b"0006a\n"), 6, PacketLine::Data(b"a"))
    }

    #[test]
    fn ignore_extra_bytes() -> crate::Result {
        assert_complete(streaming(b"0006a\nhello"), 6, PacketLine::Data(b"a"))
    }

    #[test]
    fn error_on_oversized_line() {
        assert_err_display(
            streaming(b"ffff"),
            "The data received claims to be larger than than the maximum allowed size: got 65535, exceeds 65516",
        );
    }

    #[test]
    fn error_on_error_line() {
        assert_err_display(
            streaming(b"0011ERR the error-and just ignored because not part of the size"),
            "the error",
        );
    }

    #[test]
    fn error_on_invalid_hex() {
        assert_err_display(
            streaming(b"fooo"),
            "Failed to decode the first four hex bytes indicating the line length",
        );
    }

    #[test]
    fn error_on_empty_line() {
        assert_err_display(streaming(b"0004"), "Received an invalid empty line");
    }

    #[test]
    fn round_trips() -> crate::Result {
        for (line, bytes) in &[
            (PacketLine::ResponseEnd, 4),
            (PacketLine::Delimiter, 4),
            (PacketLine::Flush, 4),
            (PacketLine::Data(b"hello there"), 15),
        ] {
            let mut out = Vec::new();
            line.to_write(&mut out)?;
            assert_complete(streaming(&out), *bytes, *line)?;
        }
        Ok(())
    }

    #[test]
    fn roundtrip_error_line() -> crate::Result {
        let mut out = Vec::new();
        PacketLine::Data(b"the error").to_error().to_write(&mut out)?;
        assert_err_display(streaming(&out), "the error");
        Ok(())
    }

    #[test]
    fn roundtrip_side_bands() -> crate::Result {
        for channel in &[Channel::Data, Channel::Error, Channel::Progress] {
            let mut out = Vec::new();
            let band = PacketLine::Data(b"band data").to_band(*channel);
            band.to_write(&mut out)?;
            match streaming(&out)? {
                Stream::Complete { line, bytes_consumed } => {
                    assert_eq!(bytes_consumed, out.len());
                    assert_eq!(line.decode_band(), band);
                }
                Stream::Incomplete { .. } => panic!("roundtrips are never incomplete"),
            }
        }
        Ok(())
    }

    mod incomplete {
        use git_protocol::packet_line::decode::{self, streaming, Stream};

        fn assert_incomplete(res: Result<Stream, decode::Error>, expected_missing: usize) -> crate::Result {
            match res? {
                Stream::Complete { .. } => {
                    panic!("expected parsing to be partial, not complete");
                }
                Stream::Incomplete { bytes_needed } => {
                    assert_eq!(bytes_needed, expected_missing);
                }
            }
            Ok(())
        }

        #[test]
        fn missing_hex_bytes() -> crate::Result {
            assert_incomplete(streaming(b"0"), 3)?;
            assert_incomplete(streaming(b"00"), 2)?;
            Ok(())
        }

        #[test]
        fn missing_data_bytes() -> crate::Result {
            assert_incomplete(streaming(b"0005"), 1)?;
            assert_incomplete(streaming(b"0006a"), 1)?;
            Ok(())
        }
    }
}
