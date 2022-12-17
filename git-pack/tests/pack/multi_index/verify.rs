use std::sync::atomic::AtomicBool;

use git_features::progress;
use maplit::btreemap;

use crate::pack::multi_index::multi_index;

#[test]
fn checksum() -> crate::Result {
    let (file, _) = multi_index();
    assert_eq!(
        file.verify_checksum(progress::Discard, &AtomicBool::new(false))?,
        file.checksum()
    );
    Ok(())
}

#[test]
fn integrity() {
    let (file, _) = multi_index();
    let outcome = file
        .verify_integrity(progress::Discard, &AtomicBool::new(false), Default::default())
        .unwrap();
    assert_eq!(outcome.actual_index_checksum, file.checksum());
    assert_eq!(
        outcome.pack_traverse_statistics,
        vec![git_pack::index::traverse::Statistics {
            average: git_pack::data::decode::entry::Outcome {
                kind: git_object::Kind::Tree,
                num_deltas: 1,
                decompressed_size: 47,
                compressed_size: 46,
                object_size: 152
            },
            objects_per_chain_length: btreemap! {
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
