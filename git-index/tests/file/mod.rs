mod init {
    use git_index::Version;

    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::index_fixture_path(name), git_hash::Kind::Sha1).unwrap()
    }

    #[test]
    #[ignore]
    fn read_v2_with_single_entry_tree() {
        let file = file("v2");
        assert_eq!(file.version(), Version::V2);
    }
}
