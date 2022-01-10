mod init {
    use git_index::Version;

    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::index_fixture_path(name), git_hash::Kind::Sha1).unwrap()
    }

    #[test]
    #[ignore]
    fn read_v2_with_single_entry_tree_and_eoie_ext() {
        let file = file("v2");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    #[ignore]
    fn read_v2_with_multiple_entries_without_eoie_ext() {
        let file = file("v2_more_files");
        assert_eq!(file.version(), Version::V2);
    }

    #[test]
    #[ignore]
    fn read_without_any_extension() {}

    #[test]
    #[ignore]
    fn read_v4_with_delta_paths() {}
}
