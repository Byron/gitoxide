#[test]
fn access() {
    let path = git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo_multi_index.sh")
        .unwrap()
        .join(".git/objects/pack/multi-pack-index");
    let _file = git_pack::multi_index::File::at(path).unwrap();
}
