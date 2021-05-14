mod data_to_write {
    use crate::assert_err_display;
    use bstr::ByteSlice;
    use futures_lite::future;
    use git_packetline::{encode, encode::data_to_write};

    #[test]
    fn binary_and_non_binary() -> crate::Result {
        future::block_on(async {
            let mut out = Vec::new();
            assert_eq!(data_to_write(b"\0", &mut out).await?, 5);
            assert_eq!(out.as_bstr(), b"0005\0".as_bstr());

            out.clear();
            assert_eq!(data_to_write("hello world, it works\n".as_bytes(), &mut out).await?, 26);
            assert_eq!(out.as_bstr(), b"001ahello world, it works\n".as_bstr());
            Ok::<_, encode::Error>(())
        })
        .map_err(Into::into)
    }

    #[test]
    fn error_if_data_exceeds_limit() {
        fn vec_sized(size: usize) -> Vec<u8> {
            let mut v = Vec::new();
            v.resize(size, 0);
            v
        }

        assert_err_display(
            future::block_on(data_to_write(&vec_sized(65516 + 1), futures_lite::io::sink())),
            "Cannot encode more than 65516 bytes, got 65517",
        );
    }

    #[test]
    fn error_if_data_is_empty() {
        assert_err_display(
            future::block_on(data_to_write(&[], futures_lite::io::sink())),
            "Empty lines are invalid",
        );
    }
}

mod text_to_write {
    use bstr::ByteSlice;
    use futures_lite::future;
    use git_packetline::{encode, encode::text_to_write};

    #[test]
    fn always_appends_a_newline() -> crate::Result {
        future::block_on(async {
            let mut out = Vec::new();
            assert_eq!(text_to_write(b"a", &mut out).await?, 6);
            assert_eq!(out.as_bstr(), b"0006a\n".as_bstr());

            out.clear();
            assert_eq!(text_to_write(b"a\n", &mut out).await?, 7);
            assert_eq!(
                out.as_bstr(),
                b"0007a\n\n".as_bstr(),
                "newline must be appended, as the receiving end is likely to remove it"
            );
            Ok::<_, encode::Error>(())
        })
        .map_err(Into::into)
    }
}
