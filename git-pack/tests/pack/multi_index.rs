#[test]
fn access() {
    let path = git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo_multi_index.sh")
        .unwrap()
        .join(".git/objects/pack/multi-pack-index");
    let file = git_pack::multi_index::File::at(path).unwrap();

    assert_eq!(file.num_packs(), 1);
    assert_eq!(file.hash_kind(), git_hash::Kind::Sha1);
}
