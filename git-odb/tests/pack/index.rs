use crate::{
    fixture_path, hex_to_id,
    pack::{SMALL_PACK, SMALL_PACK_INDEX},
};
use git_odb::pack::{self, decode::DecodeEntryResult, index};
use pretty_assertions::assert_eq;

const INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
const PACK_FOR_INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";

const INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
const PACK_FOR_INDEX_V1: &str = "packs/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";

mod method {
    mod v1 {
        use crate::{fixture_path, pack::index::INDEX_V1};
        use git_odb::pack::index;

        #[test]
        fn lookup_index() {
            let idx = index::File::at(&fixture_path(INDEX_V1)).unwrap();
            for (id, desired_index, assertion) in &[
                (&b"036bd66fe9b6591e959e6df51160e636ab1a682e"[..], Some(0), "first"),
                (b"f7f791d96b9a34ef0f08db4b007c5309b9adc3d6", Some(65), "close to last"),
                (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
            ] {
                assert_eq!(
                    idx.lookup_index(&git_object::Id::from_hex(*id).unwrap()),
                    *desired_index,
                    "{}",
                    assertion
                );
            }
            for entry in idx.iter() {
                let index = idx.lookup_index(&entry.oid).unwrap();
                assert_eq!(entry.oid.as_slice(), idx.oid_at_index(index));
                assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                assert_eq!(entry.crc32, idx.crc32_at_index(index));
            }
        }
    }

    mod v2 {
        use crate::fixture_path;
        use crate::pack::index::INDEX_V2;
        use git_odb::pack::index;

        #[test]
        fn lookup_index() {
            let idx = index::File::at(&fixture_path(INDEX_V2)).unwrap();
            for (id, desired_index, assertion) in &[
                (&b"0ead45fc727edcf5cadca25ef922284f32bb6fc1"[..], Some(0), "first"),
                (b"e800b9c207e17f9b11e321cc1fba5dfe08af4222", Some(29), "last"),
                (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
            ] {
                assert_eq!(
                    idx.lookup_index(&git_object::Id::from_hex(*id).unwrap()),
                    *desired_index,
                    "{}",
                    assertion
                );
            }
            for entry in idx.iter() {
                let index = idx.lookup_index(&entry.oid).unwrap();
                assert_eq!(entry.oid.as_slice(), idx.oid_at_index(index));
                assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                assert_eq!(entry.crc32, idx.crc32_at_index(index), "{} {:?}", index, entry);
            }
        }
    }
}

use common_macros::b_tree_map;
use git_features::progress::Discard;
use git_odb::pack::cache::DecodeEntryNoop;

#[test]
fn pack_lookup() {
    for (index_path, pack_path, stats) in &[
        (
            INDEX_V2,
            PACK_FOR_INDEX_V2,
            index::verify::Outcome {
                average: DecodeEntryResult {
                    kind: git_object::Kind::Tree,
                    num_deltas: 1,
                    decompressed_size: 3456,
                    compressed_size: 1725,
                    object_size: 9621,
                },
                objects_per_chain_length: b_tree_map! {
                    0 => 18,
                    1 => 4,
                    2 => 3,
                    3 => 1,
                    4 => 2,
                    5 => 1,
                    6 => 1,
                },
                total_compressed_entries_size: 51753,
                total_decompressed_entries_size: 103701,
                total_object_size: 288658,
                pack_size: 51875,
            },
        ),
        (
            INDEX_V1,
            PACK_FOR_INDEX_V1,
            index::verify::Outcome {
                average: DecodeEntryResult {
                    kind: git_object::Kind::Tree,
                    num_deltas: 0,
                    decompressed_size: 1982,
                    compressed_size: 729,
                    object_size: 2093,
                },
                objects_per_chain_length: b_tree_map! {
                    0 => 64,
                    1 => 3
                },
                total_compressed_entries_size: 48867,
                total_decompressed_entries_size: 132823,
                total_object_size: 140243,
                pack_size: 49113,
            },
        ),
        (
            SMALL_PACK_INDEX,
            SMALL_PACK,
            index::verify::Outcome {
                average: DecodeEntryResult {
                    kind: git_object::Kind::Tree,
                    num_deltas: 0,
                    decompressed_size: 118,
                    compressed_size: 85,
                    object_size: 293,
                },
                objects_per_chain_length: b_tree_map! {
                    0 => 30,
                    1 => 6,
                    2 => 6,
                },
                total_compressed_entries_size: 3604,
                total_decompressed_entries_size: 4997,
                total_object_size: 12307,
                pack_size: 3732,
            },
        ),
    ] {
        let idx = index::File::at(&fixture_path(index_path)).unwrap();
        let pack = pack::File::at(&fixture_path(pack_path)).unwrap();

        assert_eq!(pack.kind(), pack::Kind::V2);
        assert_eq!(pack.num_objects(), idx.num_objects());
        assert_eq!(
            idx.verify_checksum_of_index(Some(&pack), None, Discard.into(), || DecodeEntryNoop)
                .unwrap(),
            (idx.checksum_of_index(), Some(stats.to_owned()))
        );
        for idx_entry in idx.iter() {
            let pack_entry = pack.entry(idx_entry.pack_offset);
            assert_ne!(pack_entry.data_offset, idx_entry.pack_offset);
        }
    }
}

#[test]
fn iter() {
    for (path, kind, num_objects, version, index_checksum, pack_checksum) in &[
        (
            INDEX_V1,
            index::Kind::V1,
            67,
            1,
            "5a2b20ef73ffe911178532df86232b64830cb536",
            "7ebaef998897d903e6e6b6763d3a6ec4dc5b845b",
        ),
        (
            INDEX_V2,
            index::Kind::V2,
            30,
            2,
            "560eba66e6b391eb83efc3ec9fc8a3087788911c",
            "f1cd3cc7bc63a4a2b357a475a58ad49b40355470",
        ),
        (
            SMALL_PACK_INDEX,
            index::Kind::V2,
            42,
            2,
            "544a7204a55f6e9cacccf8f6e191ea8f83575de3",
            "0f3ea84cd1bba10c2a03d736a460635082833e59",
        ),
    ] {
        let idx = index::File::at(&fixture_path(path)).unwrap();
        assert_eq!(idx.kind(), *kind);
        assert_eq!(idx.version(), *version);
        assert_eq!(idx.num_objects(), *num_objects);
        assert_eq!(
            idx.verify_checksum_of_index(None, None, Discard.into(), || DecodeEntryNoop)
                .unwrap(),
            (idx.checksum_of_index(), None)
        );
        assert_eq!(idx.checksum_of_index(), hex_to_id(index_checksum));
        assert_eq!(idx.checksum_of_pack(), hex_to_id(pack_checksum));
        assert_eq!(idx.iter().count(), *num_objects as usize);
    }
}
