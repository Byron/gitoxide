mod data_to_write {
    use crate::packet_line::assert_err_display;
    use bstr::ByteSlice;
    use git_protocol::packet_line::encode::{
        data_to_write, delim_to_write, error_to_write, flush_to_write, response_end_to_write, text_to_write,
    };
    use std::io;

    fn vec_sized(size: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(size, 0);
        v
    }

    #[test]
    fn success_binary_and_non_binary() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(data_to_write(b"\0", &mut out)?, 5);
        assert_eq!(out.as_bstr(), b"0005\0".as_bstr());

        out.clear();
        assert_eq!(data_to_write("hello world, it works\n".as_bytes(), &mut out)?, 26);
        assert_eq!(out.as_bstr(), b"001ahello world, it works\n".as_bstr());
        Ok(())
    }

    #[test]
    fn success_text_to_write_always_appends_a_newline() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(text_to_write(b"a", &mut out)?, 6);
        assert_eq!(out.as_bstr(), b"0006a\n".as_bstr());

        out.clear();
        assert_eq!(text_to_write(b"a\n", &mut out)?, 7);
        assert_eq!(
            out.as_bstr(),
            b"0007a\n\n".as_bstr(),
            "newline must be appended, as the receiving end is likely to remove it"
        );
        Ok(())
    }

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

    #[test]
    fn successful_write_of_error_line() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(error_to_write(b"hello error", &mut out)?, 19);
        assert_eq!(out.as_bstr(), b"0013ERR hello error".as_bstr());
        Ok(())
    }

    #[test]
    fn error_if_data_exceeds_limit() {
        assert_err_display(
            data_to_write(&vec_sized(65516 + 1), io::sink()),
            "Cannot encode more than 65516 bytes, got 65517",
        );
    }
    #[test]
    fn error_if_data_is_empty() {
        assert_err_display(data_to_write(&[], io::sink()), "Empty lines are invalid");
    }
}
