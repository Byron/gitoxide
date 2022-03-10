mod describe {
    #[test]
    #[ignore]
    fn it_names_the_ref_the_described_commit_is_on() {
        let _dir = git_testtools::scripted_fixture_repo_read_only("make_repo.sh").unwrap();
    }
}
