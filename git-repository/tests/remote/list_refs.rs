#[cfg(feature = "blocking-network-client")]
mod blocking_io {
    use crate::remote;
    use git_features::progress;
    use git_repository as git;
    use git_repository::remote::Direction::Fetch;

    #[test]
    fn all() {
        for version in [
            None,
            Some(git::protocol::transport::Protocol::V2),
            Some(git::protocol::transport::Protocol::V1),
        ] {
            let repo = remote::repo("clone");
            let remote = repo.find_remote("origin").unwrap().force_transport_protocol(version);
            let connection = remote.connect(Fetch, progress::Discard).unwrap();
            let refs = connection.list_refs().unwrap();
            assert_eq!(refs.len(), 14, "it gets all remote refs, independently of the refspec.");
        }
    }
}
