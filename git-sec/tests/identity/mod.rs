mod uid {
    #[test]
    fn from_path() {
        let dir = tempfile::tempdir().unwrap();
        let owner = git_sec::identity::user_id::from_path(dir.path().into()).unwrap();
        assert_eq!(owner, git_sec::identity::user_id::from_process().unwrap());
    }
}
