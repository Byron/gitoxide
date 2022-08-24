mod encode_message {
    use bstr::ByteSlice;
    use git_credentials::helper::encode_message;

    #[test]
    fn from_url() -> crate::Result {
        let mut out = Vec::new();
        encode_message("https://github.com/byron/gitoxide".into(), &mut out)?;
        assert_eq!(out.as_bstr(), b"url=https://github.com/byron/gitoxide\n\n".as_bstr());
        Ok(())
    }

    mod invalid {
        use git_credentials::helper::encode_message;
        use std::io;

        #[test]
        fn contains_null() {
            assert_eq!(
                encode_message("https://foo\u{0}".into(), Vec::new())
                    .err()
                    .map(|e| e.kind()),
                Some(io::ErrorKind::Other)
            );
        }
        #[test]
        fn contains_newline() {
            assert_eq!(
                encode_message("https://foo\n".into(), Vec::new())
                    .err()
                    .map(|e| e.kind()),
                Some(io::ErrorKind::Other)
            );
        }
    }
}

mod decode_message {
    use git_credentials::helper::decode_message;

    #[test]
    fn typical_response() -> crate::Result {
        assert_eq!(
            decode_message(
                "protocol=https
host=example.com
username=bob
password=secr3t\n\n
this=is-skipped-past-empty-line"
                    .as_bytes()
            )?,
            vec![
                ("protocol", "https"),
                ("host", "example.com"),
                ("username", "bob"),
                ("password", "secr3t")
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
        );
        Ok(())
    }
}
