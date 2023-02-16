mod destructure_url_in_place {
    use gix_credentials::protocol::Context;

    fn url_ctx(url: &str) -> Context {
        Context {
            url: Some(url.into()),
            ..Default::default()
        }
    }

    fn assert_eq_parts(
        url: &str,
        proto: &str,
        user: impl Into<Option<&'static str>>,
        host: &str,
        path: impl Into<Option<&'static str>>,
        use_http_path: bool,
    ) {
        let mut ctx = url_ctx(url);
        ctx.destructure_url_in_place(use_http_path).expect("splitting works");
        assert_eq!(ctx.protocol.expect("set proto"), proto);
        match user.into() {
            Some(expected) => assert_eq!(ctx.username.expect("set user"), expected),
            None => assert!(ctx.username.is_none()),
        }
        assert_eq!(ctx.host.expect("set host"), host);
        match path.into() {
            Some(expected) => assert_eq!(ctx.path.expect("set path"), expected),
            None => assert!(ctx.path.is_none()),
        }
    }

    #[test]
    fn parts_are_verbatim_with_non_http_url() {
        // path is always used for non-http
        assert_eq_parts("ssh://user@host:21/path", "ssh", "user", "host:21", "path", false);
        assert_eq_parts("ssh://host.org/path", "ssh", None, "host.org", "path", true);
    }
    #[test]
    fn http_and_https_ignore_the_path_by_default() {
        assert_eq_parts(
            "http://user@example.com/path",
            "http",
            Some("user"),
            "example.com",
            None,
            false,
        );
        assert_eq_parts(
            "https://github.com/byron/gitoxide",
            "https",
            None,
            "github.com",
            None,
            false,
        );
        assert_eq_parts(
            "https://github.com/byron/gitoxide/",
            "https",
            None,
            "github.com",
            "byron/gitoxide",
            true,
        );
    }
}

mod to_prompt {
    use gix_credentials::protocol::Context;

    #[test]
    fn no_scheme_means_no_url() {
        assert_eq!(Context::default().to_prompt("Username"), "Username: ");
    }

    #[test]
    fn any_scheme_means_url_is_included() {
        assert_eq!(
            Context {
                protocol: Some("https".into()),
                host: Some("host".into()),
                ..Default::default()
            }
            .to_prompt("Password"),
            "Password for https://host: "
        );
    }
}

mod to_url {
    use gix_credentials::protocol::Context;

    #[test]
    fn no_protocol_is_nothing() {
        assert_eq!(Context::default().to_url(), None);
    }
    #[test]
    fn protocol_alone_is_enough() {
        assert_eq!(
            Context {
                protocol: Some("https".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "https://"
        );
    }
    #[test]
    fn username_is_appended() {
        assert_eq!(
            Context {
                protocol: Some("https".into()),
                username: Some("user".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "https://user@"
        );
    }
    #[test]
    fn host_is_appended() {
        assert_eq!(
            Context {
                protocol: Some("https".into()),
                host: Some("host".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "https://host"
        );
    }
    #[test]
    fn path_is_appended_with_leading_slash_placed_as_needed() {
        assert_eq!(
            Context {
                protocol: Some("file".into()),
                path: Some("dir/git".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "file:///dir/git"
        );
        assert_eq!(
            Context {
                protocol: Some("file".into()),
                path: Some("/dir/git".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "file:///dir/git"
        );
    }

    #[test]
    fn all_fields_with_port_but_password_is_never_shown() {
        assert_eq!(
            Context {
                protocol: Some("https".into()),
                username: Some("user".into()),
                password: Some("secret".into()),
                host: Some("example.com:8080".into()),
                path: Some("Byron/gitoxide".into()),
                ..Default::default()
            }
            .to_url()
            .unwrap(),
            "https://user@example.com:8080/Byron/gitoxide"
        );
    }
}
