use std::path::PathBuf;

use git_pack::multi_index::File;
use git_testtools::hex_to_id;

fn multi_index() -> (File, PathBuf) {
    let path = git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo_multi_index.sh")
        .expect("test fixture exists")
        .join(".git/objects/pack/multi-pack-index");
    let file = git_pack::multi_index::File::at(&path).unwrap();
    (file, path)
}

#[test]
fn access() {
    let (file, path) = multi_index();

    assert_eq!(file.version(), git_pack::multi_index::Version::V1);
    assert_eq!(file.path(), path);
    assert_eq!(file.num_indices(), 1);
    assert_eq!(file.object_hash(), git_hash::Kind::Sha1);
    assert_eq!(file.num_objects(), 868);
    assert_eq!(file.checksum(), hex_to_id("39a3804d0a84de609e4fcb49e66dc1297c75ca11"));
    // assert_eq!()
    assert_eq!(
        file.index_names(),
        vec![PathBuf::from("pack-542ad1d1c7c762ea4e36907570ff9e4b5b7dde1b.idx")]
    );

    for (idx, expected_pack_offset, expected_oid) in &[
        (0u32, 25267u64, hex_to_id("000f574443efab4ddbeee3621e49124eb3f8b6d0")),
        (140, 30421, hex_to_id("2935a65b1d69fb33c93dabc4cdf65a6f4d30ce4c")),
        (867, 24540, hex_to_id("ffea360a6a54c1185eeae4f3cfefc927cf7a35a9")),
    ] {
        let actual_oid = file.oid_at_index(*idx);
        assert_eq!(actual_oid, *expected_oid);
        assert_eq!(file.lookup(actual_oid), Some(*idx));
        let (pack_id, pack_offset) = file.pack_id_and_pack_offset_at_index(*idx);
        assert_eq!(pack_id, 0, "we only have one pack here");
        assert_eq!(pack_offset, *expected_pack_offset);
    }

    let mut count = 0;
    for (idx, entry) in file.iter().enumerate() {
        assert_eq!(entry.oid, file.oid_at_index(idx as u32));
        let (pack_index, pack_offset) = file.pack_id_and_pack_offset_at_index(idx as u32);
        assert_eq!(pack_index, entry.pack_index);
        assert_eq!(pack_offset, entry.pack_offset);
        count += 1;
    }
    assert_eq!(count, file.num_objects());
}

mod verify {
    use std::sync::atomic::AtomicBool;

    use common_macros::b_tree_map;
    use git_features::progress;

    use crate::pack::multi_index::multi_index;

    #[test]
    fn checksum() {
        let (file, _) = multi_index();
        assert_eq!(
            file.verify_checksum(progress::Discard, &AtomicBool::new(false))
                .unwrap(),
            file.checksum()
        );
    }

    #[test]
    fn integrity() {
        let (file, _) = multi_index();
        let outcome = file
            .verify_integrity(
                git_pack::index::verify::Mode::HashCrc32DecodeEncode,
                git_pack::index::traverse::Algorithm::DeltaTreeLookup,
                || git_pack::cache::Never,
                None,
                progress::Discard,
                &AtomicBool::new(false),
            )
            .unwrap();
        assert_eq!(outcome.actual_index_checksum, file.checksum());
        assert_eq!(
            outcome.pack_traverse_outcomes,
            vec![git_pack::index::traverse::Outcome {
                average: git_pack::data::decode_entry::Outcome {
                    kind: git_object::Kind::Tree,
                    num_deltas: 1,
                    decompressed_size: 47,
                    compressed_size: 46,
                    object_size: 152
                },
                objects_per_chain_length: b_tree_map! {
                    0 => 326,
                    1 => 106,
                    2 => 326,
                    3 => 108,
                    4 => 2,
                },
                total_compressed_entries_size: 40628,
                total_decompressed_entries_size: 40919,
                total_object_size: 131993,
                pack_size: 42856,
                num_commits: 16,
                num_trees: 40,
                num_tags: 1,
                num_blobs: 811
            }]
        );
    }
}
