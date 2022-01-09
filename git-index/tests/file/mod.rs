mod init {
    fn file(name: &str) -> git_index::File {
        git_index::File::at(crate::index_fixture_path(name), git_hash::Kind::Sha1).unwrap()
    }

    #[test]
    fn read_v2() {
        let _file = file("v2");
    }
}
