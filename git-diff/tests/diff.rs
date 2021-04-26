pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod tree {
    mod diff_tree {
        #[test]
        #[should_panic]
        fn file_added() {
            let _ = test_tools::scripted_fixture_repo_read_only("make_diff_repo.sh").unwrap();
            todo!("detect an added file in the root tree")
        }
    }
}
