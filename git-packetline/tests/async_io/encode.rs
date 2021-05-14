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

            // out.clear();
            // assert_eq!(text_to_write(b"a\n", &mut out)?, 7);
            // assert_eq!(
            //     out.as_bstr(),
            //     b"0007a\n\n".as_bstr(),
            //     "newline must be appended, as the receiving end is likely to remove it"
            // );
            Ok::<_, encode::Error>(())
        })
        .map_err(Into::into)
    }
}
