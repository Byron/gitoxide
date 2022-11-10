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
    #[ignore]
    fn simple_configuration() {
        let repo = repo("http-config");
        let http_config = repo
            .transport_config("https://example.com/does/not/matter")
            .expect("valid configuration")
            .expect("configuration available for http");
        let _options = http_config
            .downcast_ref::<git_transport::client::http::Options>()
            .expect("http options have been created");
    }
}
