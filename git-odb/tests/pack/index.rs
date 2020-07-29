use crate::{
    fixture_path, hex_to_id,
    pack::{INDEX_V1, PACK_FOR_INDEX_V1},
    pack::{SMALL_PACK, SMALL_PACK_INDEX},
};
use git_object::{self as object};
use git_odb::pack::{self, data::decode::Outcome, index};

const INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.idx";
const PACK_FOR_INDEX_V2: &str = "packs/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack";

mod method {
    mod v1 {
        use crate::{fixture_path, pack::INDEX_V1};
        use git_object::owned;
        use git_odb::pack::index;

        #[test]
        fn lookup() {
            let idx = index::File::at(&fixture_path(INDEX_V1)).unwrap();
            for (id, desired_index, assertion) in &[
                (&b"036bd66fe9b6591e959e6df51160e636ab1a682e"[..], Some(0), "first"),
                (b"f7f791d96b9a34ef0f08db4b007c5309b9adc3d6", Some(65), "close to last"),
                (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
            ] {
                assert_eq!(
                    idx.lookup(owned::Id::from_40_bytes_in_hex(*id).unwrap().to_borrowed()),
                    *desired_index,
                    "{}",
                    assertion
                );
            }
            for entry in idx.iter() {
                let index = idx.lookup(entry.oid.to_borrowed()).unwrap();
                assert_eq!(entry.oid.to_borrowed(), idx.oid_at_index(index));
                assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                assert_eq!(entry.crc32, idx.crc32_at_index(index));
            }
        }
    }

    mod v2 {
        use crate::{fixture_path, pack::index::INDEX_V2};
        use git_object::owned;
        use git_odb::pack::index;

        #[test]
        fn lookup() {
            let idx = index::File::at(&fixture_path(INDEX_V2)).unwrap();
            for (id, desired_index, assertion) in &[
                (&b"0ead45fc727edcf5cadca25ef922284f32bb6fc1"[..], Some(0), "first"),
                (b"e800b9c207e17f9b11e321cc1fba5dfe08af4222", Some(29), "last"),
                (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
            ] {
                assert_eq!(
                    idx.lookup(owned::Id::from_40_bytes_in_hex(*id).unwrap().to_borrowed()),
                    *desired_index,
                    "{}",
                    assertion
                );
            }
            for entry in idx.iter() {
                let index = idx.lookup(entry.oid.to_borrowed()).unwrap();
                assert_eq!(entry.oid.to_borrowed(), idx.oid_at_index(index));
                assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                assert_eq!(entry.crc32, idx.crc32_at_index(index), "{} {:?}", index, entry);
            }
        }
    }
}

use common_macros::b_tree_map;
use git_features::progress::Discard;
use git_odb::pack::cache::DecodeEntryNoop;

static ALGOS: &[index::traverse::Algorithm] = &[
    index::traverse::Algorithm::Lookup,
    index::traverse::Algorithm::DeltaTreeLookup,
];

static MODES: &[index::verify::Mode] = &[
    index::verify::Mode::Sha1CRC32,
    index::verify::Mode::Sha1CRC32Decode,
    index::verify::Mode::Sha1CRC32DecodeEncode,
];

#[test]
fn pack_lookup() {
    for (index_path, pack_path, stats) in &[
        (
            INDEX_V2,
            PACK_FOR_INDEX_V2,
            index::traverse::Outcome {
                average: Outcome {
                    kind: object::Kind::Tree,
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
            index::traverse::Outcome {
                average: Outcome {
                    kind: object::Kind::Tree,
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
            index::traverse::Outcome {
                average: Outcome {
                    kind: object::Kind::Tree,
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
        let pack = pack::data::File::at(&fixture_path(pack_path)).unwrap();

        assert_eq!(pack.kind(), pack::data::Kind::V2);
        assert_eq!(pack.num_objects(), idx.num_objects());
        for algo in ALGOS {
            for mode in MODES {
                assert_eq!(
                    idx.verify_integrity(Some((&pack, *mode, *algo)), None, Discard.into(), || DecodeEntryNoop)
                        .map(|(a, b, _)| (a, b))
                        .unwrap(),
                    (idx.index_checksum(), Some(stats.to_owned())),
                    "{:?} -> {:?}",
                    algo,
                    mode
                );
            }
        }
        let num_objects = stats
            .objects_per_chain_length
            .values()
            .map(|v| *v as usize)
            .sum::<usize>();
        let sorted_offsets = idx.sorted_offsets();
        assert_eq!(num_objects, sorted_offsets.len());
        for idx_entry in idx.iter() {
            let pack_entry = pack.entry(idx_entry.pack_offset);
            assert_ne!(pack_entry.data_offset, idx_entry.pack_offset);
            assert!(sorted_offsets.binary_search(&idx_entry.pack_offset).is_ok());
        }
        for (entry, offset_from_index) in pack
            .iter(pack::data::iter::Mode::DiscardDecompressedBytes)
            .unwrap()
            .zip(sorted_offsets.into_iter())
        {
            assert_eq!(
                entry.unwrap().pack_offset,
                offset_from_index,
                "iteration should yield the same pack offsets as the index"
            );
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
            idx.verify_integrity(None, None, Discard.into(), || { DecodeEntryNoop })
                .map(|(a, b, _)| (a, b))
                .unwrap(),
            (idx.index_checksum(), None)
        );
        assert_eq!(idx.index_checksum(), hex_to_id(index_checksum));
        assert_eq!(idx.pack_checksum(), hex_to_id(pack_checksum));
        assert_eq!(idx.iter().count(), *num_objects as usize);
    }
}
