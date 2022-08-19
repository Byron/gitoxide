#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use crate::remote;
    use git_features::progress;
    use git_repository::remote::Direction::Fetch;

    #[test]
    #[ignore]
    fn all() {
        let repo = remote::repo("clone");
        let remote = repo.find_remote("origin").unwrap();
        let mut connection = remote.connect(Fetch, progress::Discard).unwrap();
        let _refs = connection.list_refs().unwrap();
    }
}
