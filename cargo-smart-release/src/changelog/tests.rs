mod repository_url {
    use crate::changelog::write::RepositoryUrl;
    use git_repository as git;

    #[test]
    fn github_https_url() {
        for input in [
            "https://github.com/byron/gitoxide",
            "https://github.com/byron/gitoxide.git",
            "git://github.com/byron/gitoxide",
            "git://github.com/byron/gitoxide.git",
            "git@github.com:byron/gitoxide.git",
            "git@github.com:byron/gitoxide",
        ] {
            let url = RepositoryUrl::from(git::url::parse(input.into()).unwrap());
            assert_eq!(
                url.github_https().expect("possible"),
                "https://github.com/byron/gitoxide"
            )
        }
    }
}
