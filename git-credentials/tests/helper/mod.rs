mod context {
    use git_credentials::helper::Context;

    #[test]
    fn encode_decode_roundtrip() {
        for ctx in [
            Context {
                protocol: Some("https".into()),
                host: Some("github.com".into()),
                path: Some("byron/gitoxide".into()),
                username: Some("user".into()),
                password: Some("pass".into()),
                url: Some("https://github.com/byron/gitoxide".into()),
            },
            Context::default(),
            Context {
                url: Some("/path/to/repo".into()),
                ..Context::default()
            },
        ] {
            let mut buf = Vec::<u8>::new();
            ctx.write_to(&mut buf).unwrap();
            let actual = Context::from_bytes(&buf).unwrap();
            assert_eq!(actual, ctx, "ctx should encode itself losslessly");
        }
    }

    #[test]
    fn null_bytes_when_decoding() {
        let err = Context::from_bytes(b"url=https://foo\0").unwrap_err();
        assert!(matches!(
            err,
            git_credentials::helper::context::decode::Error::Encoding(_)
        ));
    }

    #[test]
    fn null_bytes_and_newlines_are_invalid_during_encoding() {
        for input in [&b"https://foo\0"[..], b"https://foo\n"] {
            let ctx = Context {
                url: Some(input.into()),
                ..Default::default()
            };
            let mut buf = Vec::<u8>::new();
            let err = ctx.write_to(&mut buf).unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::Other);
        }
    }
}
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
                .map(|(k, v)| (k.to_string(), (*v).into()))
                .collect::<Vec<_>>()
            );
            Ok(())
        }
    }
}
