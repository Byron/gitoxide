mod flush_delim_response_end {
    use bstr::ByteSlice;
    use git_packetline::encode::{delim_to_write, flush_to_write, response_end_to_write};

    #[test]
    fn success_flush_delim_response_end() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(flush_to_write(&mut out)?, 4);
        assert_eq!(out.as_bstr(), b"0000".as_bstr());

        out.clear();
        assert_eq!(delim_to_write(&mut out)?, 4);
        assert_eq!(out.as_bstr(), b"0001".as_bstr());

        out.clear();
        assert_eq!(response_end_to_write(&mut out)?, 4);
        assert_eq!(out.as_bstr(), b"0002".as_bstr());
        Ok(())
    }
}

mod error {
    use bstr::ByteSlice;
    use git_packetline::encode::error_to_write;

    #[test]
    fn write_line() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(error_to_write(b"hello error", &mut out)?, 19);
        assert_eq!(out.as_bstr(), b"0013ERR hello error".as_bstr());
        Ok(())
    }
}
