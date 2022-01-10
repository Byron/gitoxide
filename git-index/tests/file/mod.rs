mod init {
    use git_index::Version;

    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::index_fixture_path(name), git_hash::Kind::Sha1).unwrap()
    }

    #[test]
    fn read_v2() {
        let file = file("v2");
        assert_eq!(file.version(), Version::V2);
    }
}
