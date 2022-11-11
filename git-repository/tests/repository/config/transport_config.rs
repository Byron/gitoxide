#[cfg(feature = "blocking-http-transport")]
mod http {
    use git_repository as git;

    fn base_repo_path() -> String {
        git::path::realpath(
            git_testtools::scripted_fixture_repo_read_only("make_remote_repos.sh")
                .unwrap()
                .join("base"),
        )
        .unwrap()
        .to_string_lossy()
        .into_owned()
    }

    pub(crate) fn repo(name: &str) -> git::Repository {
        let dir = git_testtools::scripted_fixture_repo_read_only_with_args("make_fetch_repos.sh", [base_repo_path()])
            .unwrap();
        git::open_opts(dir.join(name), git::open::Options::isolated()).unwrap()
    }

    #[test]
    fn simple_configuration() {
        let repo = repo("http-config");
        let http_config = repo
            .transport_config("https://example.com/does/not/matter")
            .expect("valid configuration")
            .expect("configuration available for http");
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
        } = http_config
            .downcast_ref::<git_transport::client::http::Options>()
            .expect("http options have been created");
        assert_eq!(extra_headers, &["ExtraHeader: value1", "ExtraHeader: value2"]);
        assert_eq!(
            *follow_redirects,
            git_transport::client::http::options::FollowRedirects::Initial
        );
        assert_eq!(*low_speed_limit_bytes_per_second, 5120);
        assert_eq!(*low_speed_time_seconds, 10);
        assert_eq!(
            proxy.as_deref(),
            Some("localhost:9090"),
            "TODO: turn it into a URL valid for curl"
        );
        assert_eq!(
            proxy_auth_method.as_ref(),
            // Some(&git_transport::client::http::options::ProxyAuthMethod::AnyAuth)
            None,
            "TODO: implement auth"
        );
        assert_eq!(user_agent.as_deref(), Some("agentJustForHttp"));
        assert_eq!(
            *connect_timeout,
            std::time::Duration::from_secs(20),
            "this is an arbitrary default, and it's her to allow adjustments of the default"
        );
        assert!(
            backend.is_none(),
            "backed is never set as it's backend specific, rather custom options typically"
        )
    }
}
