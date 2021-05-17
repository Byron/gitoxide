mod streaming {
    use bstr::ByteSlice;
    use git_packetline::{
        decode::{self, streaming, Stream},
        Channel, PacketLine,
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
    fn trailing_line_feeds_are_removed_explicitly_roundtrip() -> crate::Result {
        let line = decode::all_at_once(b"0006a\n")?;
        assert_eq!(line.to_text().expect("text").0.as_bstr(), b"a".as_bstr());
        let mut out = Vec::new();
        line.to_text()
            .expect("text")
            .to_write(&mut out)
            .expect("write to memory works");
        assert_eq!(out, b"0006a\n", "it appends a newline in text mode");
        Ok(())
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
        PacketLine::Data(b"the error")
            .to_error()
            .expect("data line")
            .to_write(&mut out)?;
        let line = decode::all_at_once(&out)?;
        assert_eq!(line.check_error().expect("err").0, b"the error");
        Ok(())
    }

    #[test]
    fn roundtrip_side_bands() -> crate::Result {
        for channel in &[Channel::Data, Channel::Error, Channel::Progress] {
            let mut out = Vec::new();
            let band = PacketLine::Data(b"band data")
                .to_band(*channel)
                .expect("data is valid for band");
            band.to_write(&mut out)?;
            let line = decode::all_at_once(&out)?;
            assert_eq!(line.decode_band().expect("valid band"), band);
        }
        Ok(())
    }
}
