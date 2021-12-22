use git_testtools::hex_to_id;
use std::path::PathBuf;

#[test]
fn access() {
    let path = git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo_multi_index.sh")
        .unwrap()
        .join(".git/objects/pack/multi-pack-index");
    let file = git_pack::multi_index::File::at(&path).unwrap();

    assert_eq!(file.path(), path);
    assert_eq!(file.num_packs(), 1);
    assert_eq!(file.object_hash(), git_hash::Kind::Sha1);
    assert_eq!(file.num_objects(), 868);
    assert_eq!(file.checksum(), hex_to_id("39a3804d0a84de609e4fcb49e66dc1297c75ca11"));
    assert_eq!(
        file.index_names(),
        vec![PathBuf::from("pack-542ad1d1c7c762ea4e36907570ff9e4b5b7dde1b.idx")]
    );

    for (idx, expected_oid) in &[
        (0u32, hex_to_id("000f574443efab4ddbeee3621e49124eb3f8b6d0")),
        (140, hex_to_id("2935a65b1d69fb33c93dabc4cdf65a6f4d30ce4c")),
        (867, hex_to_id("ffea360a6a54c1185eeae4f3cfefc927cf7a35a9")),
    ] {
        let actual_oid = file.oid_at_index(*idx);
        assert_eq!(actual_oid, *expected_oid);
    }
}
