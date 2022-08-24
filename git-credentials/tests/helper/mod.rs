mod message {
    mod encode {
        use bstr::ByteSlice;
        use git_credentials::helper::message;

        #[test]
        fn from_url() -> crate::Result {
            let mut out = Vec::new();
            message::encode("https://github.com/byron/gitoxide".into(), &mut out)?;
            assert_eq!(out.as_bstr(), b"url=https://github.com/byron/gitoxide\n\n".as_bstr());
            Ok(())
        }

        mod invalid {
            use git_credentials::helper::message;
            use std::io;

            #[test]
            fn contains_null() {
                assert_eq!(
                    message::encode("https://foo\u{0}".into(), Vec::new())
                        .err()
                        .map(|e| e.kind()),
                    Some(io::ErrorKind::Other)
                );
            }
            #[test]
            fn contains_newline() {
                assert_eq!(
                    message::encode("https://foo\n".into(), Vec::new())
                        .err()
                        .map(|e| e.kind()),
                    Some(io::ErrorKind::Other)
                );
            }
        }
    }

    mod decode {
        use git_credentials::helper::message;

        #[test]
        fn typical_response() -> crate::Result {
            assert_eq!(
                message::decode(
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
}
