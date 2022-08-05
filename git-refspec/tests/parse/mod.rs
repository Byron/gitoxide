use git_testtools::scripted_fixture_repo_read_only;

#[test]
#[ignore]
fn baseline() {
    let _dir = scripted_fixture_repo_read_only("make_baseline.sh").unwrap();
}
