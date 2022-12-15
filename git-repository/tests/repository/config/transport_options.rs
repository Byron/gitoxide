#[cfg(any(
    feature = "blocking-http-transport-reqwest",
    feature = "blocking-http-transport-curl"
))]
mod http {
    use git_repository as git;
    use git_transport::client::http::options::{FollowRedirects, ProxyAuthMethod};

    pub(crate) fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_read_only("make_config_repos.sh").unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }

    fn http_options(
        repo: &git::Repository,
        remote_name: Option<&str>,
        url: &str,
    ) -> git_transport::client::http::Options {
        let opts = repo
            .transport_options(url, remote_name.map(Into::into))
            .expect("valid configuration")
            .expect("configuration available for http");
        opts.downcast_ref::<git_transport::client::http::Options>()
            .expect("http options have been created")
            .to_owned()
    }

    #[test]
    fn remote_overrides() {
        let repo = repo("http-remote-override");
        let git_transport::client::http::Options {
            proxy,
            proxy_auth_method,
            follow_redirects,
            ..
        } = http_options(&repo, Some("origin"), "https://example.com/does/not/matter");

        assert_eq!(proxy_auth_method, ProxyAuthMethod::Negotiate);
        assert_eq!(proxy.as_deref(), Some("http://overridden"));
        assert_eq!(follow_redirects, FollowRedirects::Initial);
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
            no_proxy,
            proxy_auth_method,
            proxy_authenticate,
            user_agent,
            connect_timeout,
            verbose,
            backend,
        } = http_options(&repo, None, "https://example.com/does/not/matter");
        assert_eq!(
            extra_headers,
            &["ExtraHeader: value2", "ExtraHeader: value3"],
            "it respects empty values to clear prior values"
        );
        assert_eq!(follow_redirects, FollowRedirects::All);
        assert_eq!(low_speed_limit_bytes_per_second, 5120);
        assert_eq!(low_speed_time_seconds, 10);
        assert_eq!(proxy.as_deref(), Some("http://localhost:9090"),);
        assert!(
            proxy_authenticate.is_none(),
            "no username means no authentication required"
        );
        assert_eq!(proxy_auth_method, ProxyAuthMethod::Basic);
        assert_eq!(user_agent.as_deref(), Some("agentJustForHttp"));
        assert_eq!(connect_timeout, Some(std::time::Duration::from_millis(60 * 1024)));
        assert_eq!(no_proxy, None);
        assert!(!verbose, "verbose is disabled by default");
        assert!(
            backend.is_none(),
            "backed is never set as it's backend specific, rather custom options typically"
        )
    }

    #[test]
    fn http_verbose() {
        let repo = repo("http-verbose");
        let opts = http_options(&repo, None, "https://example.com/does/not/matter");
        assert!(opts.verbose);
    }

    #[test]
    fn http_no_proxy() {
        let repo = repo("http-no-proxy");
        let opts = http_options(&repo, None, "https://example.com/does/not/matter");
        assert_eq!(opts.no_proxy.as_deref(), Some("no validation done here"));
    }

    #[test]
    fn http_proxy_with_username() {
        let repo = repo("http-proxy-authenticated");

        let opts = http_options(&repo, None, "https://example.com/does/not/matter");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://user@localhost:9090"),
            "usernames in proxy urls trigger authentication before making a connection…"
        );
        assert!(
            opts.proxy_authenticate.is_some(),
            "…and credential-helpers are used to do that. This could be overridden in remotes one day"
        );
        assert_eq!(
            opts.follow_redirects,
            FollowRedirects::All,
            "an empty value is true, so we can't take shortcuts for these"
        );
    }

    #[test]
    fn empty_proxy_string_turns_it_off() {
        let repo = repo("http-proxy-empty");

        let opts = http_options(&repo, None, "https://example.com/does/not/matter");
        assert_eq!(
            opts.proxy.as_deref(),
            Some(""),
            "empty strings indicate that the proxy is to be unset by the transport"
        );
        assert_eq!(opts.follow_redirects, FollowRedirects::None);
    }

    #[test]
    fn https_specific_proxy_only() {
        let repo = repo("https-proxy-only");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://https"),
            "if there is no other proxy setting, it will be used for https"
        );

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(opts.proxy, None, "non-https urls don't use this proxy at all");
    }

    #[test]
    fn env_http_proxy_only() {
        let repo = repo("gitoxide-http-proxy-only");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://http-fallback"),
            "the `http_proxy` env var derived value serves as fallback…"
        );

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://http-fallback"),
            "…for http-urls as well, as there is nothing else set."
        );
    }

    #[test]
    fn all_proxy_only() {
        let repo = repo("gitoxide-all-proxy-only");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://all-proxy-fallback"),
            "the `all_proxy` env var derived value serves as fallback…"
        );

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://all-proxy-fallback"),
            "…for http-urls as well, as there is nothing else set."
        );
    }

    #[test]
    fn all_proxy_is_fallback() {
        let repo = repo("gitoxide-all-proxy");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(opts.proxy.as_deref(), Some("http://http"));

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(opts.proxy.as_deref(), Some("http://http"));
    }

    #[test]
    fn env_http_proxy_is_fallback() {
        let repo = repo("gitoxide-http-proxy");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(opts.proxy.as_deref(), Some("http://http"));

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(opts.proxy.as_deref(), Some("http://http"));
    }

    #[test]
    fn https_specific_proxy_is_only_a_fallback() {
        let repo = repo("https-proxy");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://http"),
            "if the http proxy is set, it will be used even for https as the latter is only a fallback (by env vars)"
        );

        let opts = http_options(&repo, None, "http://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some("http://http"),
            "http urls use the http proxy configuration like normal"
        );
    }

    #[test]
    fn https_specific_proxy_empty() {
        let repo = repo("https-proxy-empty");

        let opts = http_options(&repo, None, "https://host.local/repo");
        assert_eq!(
            opts.proxy.as_deref(),
            Some(""),
            "empty strings work just like they do for http.proxy (and empty strings indicate to unset it)"
        );
    }

    #[test]
    fn proxy_without_protocol_is_defaulted_to_http() {
        let repo = repo("http-proxy-auto-prefix");

        let opts = http_options(&repo, None, "https://example.com/does/not/matter");
        assert_eq!(opts.proxy.as_deref(), Some("http://localhost:9090"));
        assert_eq!(opts.follow_redirects, FollowRedirects::Initial);
    }
}
