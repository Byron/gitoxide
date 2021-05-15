mod data_to_write {
    use crate::assert_err_display;
    use bstr::ByteSlice;
    #[cfg(all(feature = "async-io", not(feature = "blocking-io")))]
    use futures_lite::io;
    use git_packetline::encode::data_to_write;
    #[cfg(feature = "blocking-io")]
    use std::io;

    #[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
    async fn binary_and_non_binary() -> crate::Result {
        let mut out = Vec::new();
        let res = data_to_write(b"\0", &mut out).await?;
        assert_eq!(res, 5);
        assert_eq!(out.as_bstr(), b"0005\0".as_bstr());

        out.clear();
        let res = data_to_write("hello world, it works\n".as_bytes(), &mut out).await?;
        assert_eq!(res, 26);
        assert_eq!(out.as_bstr(), b"001ahello world, it works\n".as_bstr());

        Ok(())
    }

    #[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
    async fn error_if_data_exceeds_limit() {
        fn vec_sized(size: usize) -> Vec<u8> {
            let mut v = Vec::new();
            v.resize(size, 0);
            v
        }

        let res = data_to_write(&vec_sized(65516 + 1), io::sink()).await;
        assert_err_display(res, "Cannot encode more than 65516 bytes, got 65517");
    }

    #[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
    async fn error_if_data_is_empty() {
        assert_err_display(data_to_write(&[], io::sink()).await, "Empty lines are invalid");
    }
}

mod text_to_write {
    use bstr::ByteSlice;
    use git_packetline::encode::text_to_write;

    #[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
    async fn always_appends_a_newline() -> crate::Result {
        let mut out = Vec::new();
        let res = text_to_write(b"a", &mut out).await?;
        assert_eq!(res, 6);
        assert_eq!(out.as_bstr(), b"0006a\n".as_bstr());

        out.clear();
        let res = text_to_write(b"a\n", &mut out).await?;
        assert_eq!(res, 7);
        assert_eq!(
            out.as_bstr(),
            b"0007a\n\n".as_bstr(),
            "newline must be appended, as the receiving end is likely to remove it"
        );
        Ok(())
    }
}
