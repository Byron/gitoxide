mod encode_message {
    use bstr::ByteSlice;
    use git_protocol::credential;

    #[test]
    fn from_url() -> crate::Result {
        let mut out = Vec::new();
        credential::encode_message("https://github.com/byron/gitoxide", &mut out)?;
        assert_eq!(out.as_bstr(), b"url=https://github.com/byron/gitoxide\n\n".as_bstr());
        Ok(())
    }

    mod invalid {
        use git_protocol::credential;
        use std::io;

        #[test]
        fn contains_null() {
            assert_eq!(
                credential::encode_message("https://foo\u{0}", Vec::new())
                    .err()
                    .map(|e| e.kind()),
                Some(io::ErrorKind::Other)
            );
        }
        #[test]
        fn contains_newline() {
            assert_eq!(
                credential::encode_message("https://foo\n", Vec::new())
                    .err()
                    .map(|e| e.kind()),
                Some(io::ErrorKind::Other)
            );
        }
    }
}
