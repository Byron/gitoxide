use git_testtools::hex_to_id;

#[test]
fn access() {
    let path = git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo_multi_index.sh")
        .unwrap()
        .join(".git/objects/pack/multi-pack-index");
    let file = git_pack::multi_index::File::at(path).unwrap();

    assert_eq!(file.num_packs(), 1);
    assert_eq!(file.object_hash(), git_hash::Kind::Sha1);
    assert_eq!(file.num_objects(), 868);
    assert_eq!(file.checksum(), hex_to_id("39a3804d0a84de609e4fcb49e66dc1297c75ca11"));
}
