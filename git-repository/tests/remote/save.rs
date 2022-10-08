mod save_as_to {
    use crate::basic_repo;
    use git_repository as git;
    use std::convert::TryInto;

    #[test]
    fn new_anonymous_remote_with_name() {
        let repo = basic_repo().unwrap();
        let mut remote = repo
            .remote_at("https://example.com/path")
            .unwrap()
            .push_url("https://ein.hub/path")
            .unwrap();
        let remote_name = "origin";
        assert!(
            repo.find_remote(remote_name).is_err(),
            "there is no remote of that name"
        );
        assert_eq!(remote.name(), None);
        let mut config = git::config::File::default();
        remote
            .save_as_to(remote_name.try_into().expect("valid name"), &mut config)
            .unwrap();
        assert_eq!(
            uniformize(config.to_string()),
            "[remote \"origin\"]\n\turl = https://example.com/path\n\tpushurl = https://ein.hub/path\n"
        )
    }

    fn uniformize(input: String) -> String {
        input.replace("\r\n", "\n")
    }
}
