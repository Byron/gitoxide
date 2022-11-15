#[cfg(feature = "blocking-http-transport")]
mod http {
    use git_repository as git;

    pub(crate) fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_config_repos.sh").unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }

    fn http_options(repo: &git::Repository) -> git_transport::client::http::Options {
        let opts = repo
            .transport_options("https://example.com/does/not/matter")
            .expect("valid configuration")
            .expect("configuration available for http");
        opts.downcast_ref::<git_transport::client::http::Options>()
            .expect("http options have been created")
            .to_owned()
    }

    #[test]
    fn simple_configuration() {
        let repo = repo("http-config");
        let git_transport::client::http::Options {
            extra_headers,
            follow_redirects,
            low_speed_limit_bytes_per_second,
            low_speed_time_seconds,
            proxy,
            proxy_auth_method,
            user_agent,
            connect_timeout,
            backend,
        } = http_options(&repo);
        assert_eq!(
            extra_headers,
            &["ExtraHeader: value2", "ExtraHeader: value3"],
            "it respects empty values to clear prior values"
        );
        assert_eq!(
            follow_redirects,
            git_transport::client::http::options::FollowRedirects::Initial
        );
        assert_eq!(low_speed_limit_bytes_per_second, 5120);
        assert_eq!(low_speed_time_seconds, 10);
        assert_eq!(proxy.as_deref(), Some("http://localhost:9090"),);
        assert_eq!(
            proxy_auth_method.as_ref(),
            // Some(&git_transport::client::http::options::ProxyAuthMethod::AnyAuth)
            None,
            "TODO: implement auth"
        );
        assert_eq!(user_agent.as_deref(), Some("agentJustForHttp"));
        assert_eq!(
            connect_timeout,
            std::time::Duration::from_secs(20),
            "this is an arbitrary default, and it's her to allow adjustments of the default"
        );
        assert!(
            backend.is_none(),
            "backed is never set as it's backend specific, rather custom options typically"
        )
    }

    #[test]
    fn empty_proxy_string_turns_it_off() {
        let repo = repo("http-proxy-empty");

        let opts = http_options(&repo);
        assert_eq!(
            opts.proxy.as_deref(),
            Some(""),
            "empty strings indicate that the proxy is to be unset by the transport"
        );
    }

    #[test]
    fn proxy_without_protocol_is_defaulted_to_http() {
        let repo = repo("http-proxy-auto-prefix");

        let opts = http_options(&repo);
        assert_eq!(opts.proxy.as_deref(), Some("http://localhost:9090"));
    }
}
