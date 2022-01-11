mod init {
    use git_index::Version;
    use git_testtools::hex_to_id;

    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::index_fixture_path(name), git_hash::Kind::Sha1).unwrap()
    }

    #[test]
    fn read_v2_with_single_entry_tree_and_eoie_ext() {
        let file = file("v2");
        assert_eq!(file.version(), Version::V2);

        assert_eq!(file.entries().len(), 1);

        let entry = &file.entries()[0];
        assert_eq!(entry.id, hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"));
        assert_eq!(entry.path(&file.state), "a");
    }

    #[test]
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
