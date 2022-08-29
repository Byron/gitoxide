mod context {
    mod to_prompt {
        use git_credentials::protocol::Context;

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
        use git_credentials::protocol::Context;

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
        fn all_fields_with_port() {
            assert_eq!(
                Context {
                    protocol: Some("https".into()),
                    username: Some("user".into()),
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
}
