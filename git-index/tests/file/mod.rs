mod init {

    #[test]
    #[ignore]
    fn v2() {
        let _file = git_index::File::at(crate::index_fixture_path("v2"), git_hash::Kind::Sha1).unwrap();
    }
}
