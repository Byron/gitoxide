mod repository_url {

    use crate::changelog::write::RepositoryUrl;

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
            let url = RepositoryUrl::from(gix::url::parse(input.into()).unwrap());
            assert_eq!(
                url.github_https().expect("possible"),
                "https://github.com/byron/gitoxide"
            )
        }
    }
}
