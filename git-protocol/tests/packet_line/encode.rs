mod data_to_write {
    use crate::packet_line::assert_err_display;
    use bstr::ByteSlice;
    use git_protocol::packet_line::encode::{data_to_write, flush_to_write};
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
    fn success_flush() -> crate::Result {
        let mut out = Vec::new();
        assert_eq!(flush_to_write(&mut out)?, 4);
        assert_eq!(out.as_bstr(), b"0000".as_bstr());
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
