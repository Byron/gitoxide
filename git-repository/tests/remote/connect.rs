#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use crate::remote;
    use git_repository::remote::Direction::Fetch;

    #[test]
    fn refs() {
        let repo = remote::repo("clone");
        let remote = repo.find_remote("origin").unwrap();
        let _connection = remote.connect(Fetch).unwrap();
    }
}
