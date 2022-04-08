mod parse;
mod matching {

    #[test]
    fn compare_baseline_with_ours() {
        let _dir = git_testtools::scripted_fixture_repo_read_only("make_baseline.sh");
    }
}
