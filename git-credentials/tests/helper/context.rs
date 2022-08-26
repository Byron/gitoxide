use git_credentials::protocol::Context;

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
            quit: Some(true),
        },
        Context::default(),
        Context {
            quit: Some(false),
            ..Context::default()
        },
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

mod write_to {
    use git_credentials::protocol::Context;

    #[test]
    fn null_bytes_and_newlines_are_invalid() {
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

mod from_bytes {
    use git_credentials::protocol::Context;

    #[test]
    fn empty_newlines_cause_skipping_remaining_input() {
        let input = b"protocol=https
host=example.com\n
password=secr3t
username=bob";
        assert_eq!(
            Context::from_bytes(input).unwrap(),
            Context {
                protocol: Some("https".into()),
                host: Some("example.com".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn unknown_field_names_are_skipped() {
        let input = b"protocol=https
unknown=value
username=bob";
        assert_eq!(
            Context::from_bytes(input).unwrap(),
            Context {
                protocol: Some("https".into()),
                username: Some("bob".into()),
                ..Default::default()
            }
        )
    }

    #[test]
    fn quit_supports_git_config_boolean_values() {
        for true_value in ["1", "42", "-42", "true", "on", "yes"] {
            let input = format!("quit={}", true_value);
            assert_eq!(
                Context::from_bytes(input.as_bytes()).unwrap().quit,
                Some(true),
                "{}",
                input
            )
        }
        for false_value in ["0", "false", "off", "no"] {
            let input = format!("quit={}", false_value);
            assert_eq!(
                Context::from_bytes(input.as_bytes()).unwrap().quit,
                Some(false),
                "{}",
                input
            )
        }
    }

    #[test]
    fn null_bytes_when_decoding() {
        let err = Context::from_bytes(b"url=https://foo\0").unwrap_err();
        assert!(matches!(
            err,
            git_credentials::protocol::context::decode::Error::Encoding(_)
        ));
    }
}
