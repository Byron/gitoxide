mod file {
    use crate::{
        fixture_path, hex_to_id,
        pack::{INDEX_V1, PACK_FOR_INDEX_V1},
        pack::{SMALL_PACK, SMALL_PACK_INDEX},
    };
    use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
    use git_object::{self as object};
    use git_odb::pack;

    mod method {
        mod v1 {
            use crate::{fixture_path, pack::INDEX_V1};
            use git_pack::index;

            #[test]
            fn lookup() -> Result<(), Box<dyn std::error::Error>> {
                let idx = index::File::at(&fixture_path(INDEX_V1))?;
                for (id, desired_index, assertion) in &[
                    (&b"036bd66fe9b6591e959e6df51160e636ab1a682e"[..], Some(0), "first"),
                    (b"f7f791d96b9a34ef0f08db4b007c5309b9adc3d6", Some(65), "close to last"),
                    (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
                ] {
                    assert_eq!(
                        idx.lookup(git_hash::ObjectId::from_hex(*id)?),
                        *desired_index,
                        "{}",
                        assertion
                    );
                }
                for entry in idx.iter() {
                    let index = idx.lookup(entry.oid).expect("id present");
                    assert_eq!(entry.oid.as_ref(), idx.oid_at_index(index));
                    assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                    assert_eq!(entry.crc32, idx.crc32_at_index(index));
                }
                Ok(())
            }
        }

        mod v2 {
            use crate::{fixture_path, pack::INDEX_V2};
            use git_pack::index;

            #[test]
            fn lookup() -> Result<(), Box<dyn std::error::Error>> {
                let idx = index::File::at(&fixture_path(INDEX_V2))?;
                for (id, desired_index, assertion) in &[
                    (&b"0ead45fc727edcf5cadca25ef922284f32bb6fc1"[..], Some(0), "first"),
                    (b"e800b9c207e17f9b11e321cc1fba5dfe08af4222", Some(29), "last"),
                    (b"ffffffffffffffffffffffffffffffffffffffff", None, "not in pack"),
                ] {
                    assert_eq!(
                        idx.lookup(git_hash::ObjectId::from_hex(*id)?),
                        *desired_index,
                        "{}",
                        assertion
                    );
                }
                for entry in idx.iter() {
                    let index = idx.lookup(entry.oid).expect("id present");
                    assert_eq!(entry.oid.as_ref(), idx.oid_at_index(index));
                    assert_eq!(entry.pack_offset, idx.pack_offset_at_index(index));
                    assert_eq!(entry.crc32, idx.crc32_at_index(index), "{} {:?}", index, entry);
                }
                Ok(())
            }
        }

        mod any {
            use crate::{fixture_path, pack::V2_PACKS_AND_INDICES};
            use filebuffer::FileBuffer;
            use git_features::progress;
            use git_odb::pack;
            use git_pack::data::{input, EntryRange};
            use std::{fs, io, sync::atomic::AtomicBool};

            #[test]
            fn write_to_stream() -> Result<(), Box<dyn std::error::Error>> {
                for mode in &[input::Mode::AsIs, input::Mode::Verify, input::Mode::Restore] {
                    for compressed in &[input::EntryDataMode::Crc32, input::EntryDataMode::KeepAndCrc32] {
                        for (index_path, data_path) in V2_PACKS_AND_INDICES {
                            let resolve = {
                                let buf = FileBuffer::open(fixture_path(data_path))?;
                                move |entry: EntryRange, out: &mut Vec<u8>| {
                                    buf.get(entry.start as usize..entry.end as usize)
                                        .map(|slice| out.copy_from_slice(slice))
                                }
                            };
                            assert_index_write(mode, compressed, index_path, data_path, resolve)?;
                        }
                    }
                }
                Ok(())
            }

            fn assert_index_write<F>(
                mode: &input::Mode,
                compressed: &input::EntryDataMode,
                index_path: &&str,
                data_path: &&str,
                resolve: F,
            ) -> Result<(), Box<dyn std::error::Error>>
            where
                F: Fn(pack::data::EntryRange, &mut Vec<u8>) -> Option<()> + Send + Sync,
            {
                let pack_iter = pack::data::input::BytesToEntriesIter::new_from_header(
                    io::BufReader::new(fs::File::open(fixture_path(data_path))?),
                    *mode,
                    *compressed,
                )?;

                let mut actual = Vec::<u8>::new();
                let desired_kind = pack::index::Version::default();
                let num_objects = pack_iter.len() as u32;
                let outcome = pack::index::File::write_data_iter_to_stream(
                    desired_kind,
                    || Ok(resolve),
                    pack_iter,
                    None,
                    progress::Discard,
                    &mut actual,
                    &AtomicBool::new(false),
                )?;

                let expected = fs::read(fixture_path(index_path))?;
                let end_of_header = 4 * 2;
                assert_eq!(&actual[..end_of_header], &expected[..end_of_header], "header");
                let end_of_fanout_table = end_of_header + 256 * 4;
                assert_eq!(
                    &actual[end_of_header..end_of_fanout_table],
                    &expected[end_of_header..end_of_fanout_table],
                    "fan out table"
                );
                let end_of_ids = end_of_fanout_table + 20 * num_objects as usize;
                assert_eq!(
                    &actual[end_of_fanout_table..end_of_ids],
                    &expected[end_of_fanout_table..end_of_ids],
                    "hashes: sha1"
                );
                let end_of_crc32 = end_of_ids + 4 * num_objects as usize;
                assert_eq!(
                    &actual[end_of_ids..end_of_crc32],
                    &expected[end_of_ids..end_of_crc32],
                    "crc32"
                );
                let end_of_offsets = end_of_crc32 + 4 * num_objects as usize;
                assert_eq!(
                    &actual[end_of_crc32..end_of_offsets],
                    &expected[end_of_crc32..end_of_offsets],
                    "offsets"
                );
                let end_of_pack_hash = end_of_offsets + 20;
                assert_eq!(
                    &actual[end_of_offsets..end_of_pack_hash],
                    &expected[end_of_offsets..end_of_pack_hash],
                    "offsets"
                );
                let end_of_index_hash = end_of_pack_hash + 20;
                assert_eq!(
                    &actual[end_of_pack_hash..end_of_index_hash],
                    &expected[end_of_pack_hash..end_of_index_hash],
                    "index hash"
                );
                assert_eq!(
                    actual, expected,
                    "we should be writing a bit-exact version of the original V2 index"
                );
                assert_eq!(
                    outcome.num_objects, num_objects,
                    "it wrote the entire iterator worth of entries"
                );
                assert_eq!(outcome.index_kind, desired_kind);
                assert_eq!(
                    outcome.index_hash,
                    git_hash::ObjectId::from_20_bytes(&expected[end_of_pack_hash..end_of_index_hash])
                );
                Ok(())
            }
        }
    }

    use crate::pack::{INDEX_V2, PACK_FOR_INDEX_V2};
    use common_macros::b_tree_map;
    use git_features::progress;
    use git_pack::cache;
    use git_pack::data::decode_entry::Outcome;
    use git_pack::index;

    static ALGORITHMS: &[index::traverse::Algorithm] = &[
        index::traverse::Algorithm::Lookup,
        index::traverse::Algorithm::DeltaTreeLookup,
    ];

    static MODES: &[index::verify::Mode] = &[
        index::verify::Mode::Sha1Crc32,
        index::verify::Mode::Sha1Crc32Decode,
        index::verify::Mode::Sha1Crc32DecodeEncode,
    ];

    #[test]
    fn pack_lookup() -> Result<(), Box<dyn std::error::Error>> {
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
                    num_commits: 10,
                    num_blobs: 5,
                    num_tags: 0,
                    num_trees: 15,
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
                    num_commits: 2,
                    num_blobs: 63,
                    num_tags: 0,
                    num_trees: 2,
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
                    num_commits: 14,
                    num_blobs: 14,
                    num_tags: 0,
                    num_trees: 14,
                    pack_size: 3732,
                },
            ),
        ] {
            let idx = index::File::at(&fixture_path(index_path))?;
            let pack = pack::data::File::at(&fixture_path(pack_path))?;

            assert_eq!(pack.version(), pack::data::Version::V2);
            assert_eq!(pack.num_objects(), idx.num_objects());
            for algo in ALGORITHMS {
                for mode in MODES {
                    assert_eq!(
                        idx.verify_integrity(
                            Some((&pack, *mode, *algo, || cache::Never)),
                            None,
                            progress::Discard.into(),
                            Default::default()
                        )
                        .map(|(a, b, _)| (a, b))?,
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
            for (entry, offset_from_index) in pack.streaming_iter()?.zip(sorted_offsets.iter().copied()) {
                let entry = entry?;
                assert_eq!(
                    entry.pack_offset, offset_from_index,
                    "iteration should yield the same pack offsets as the index"
                );

                let mut buf = Vec::new();
                buf.resize(entry.decompressed_size as usize, 0);
                let pack_entry = pack.entry(offset_from_index);
                assert_eq!(
                    pack_entry.pack_offset(),
                    entry.pack_offset,
                    "index entry offset and computed pack offset must match"
                );
                pack.decompress_entry(&pack_entry, &mut buf)?;

                assert_eq!(
                    buf.len() as u64,
                    entry.decompressed_size,
                    "the decompressed length are the same no matter what decompressed them"
                );

                let next_offset_index = sorted_offsets
                    .binary_search(&entry.pack_offset)
                    .expect("correct offset")
                    + 1;
                let next_offset = if next_offset_index == sorted_offsets.len() {
                    (pack.data_len() - SHA1_SIZE) as u64
                } else {
                    sorted_offsets[next_offset_index]
                };
                assert_eq!(
                    entry
                        .compressed
                        .expect("bytes present in default configuration of streaming iter")
                        .len() as u64,
                    next_offset - entry.pack_offset - entry.header_size as u64,
                    "we get the compressed bytes region after the head to the next entry"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn iter() -> Result<(), Box<dyn std::error::Error>> {
        for (path, kind, num_objects, index_checksum, pack_checksum) in &[
            (
                INDEX_V1,
                index::Version::V1,
                67,
                "5a2b20ef73ffe911178532df86232b64830cb536",
                "7ebaef998897d903e6e6b6763d3a6ec4dc5b845b",
            ),
            (
                INDEX_V2,
                index::Version::V2,
                30,
                "560eba66e6b391eb83efc3ec9fc8a3087788911c",
                "f1cd3cc7bc63a4a2b357a475a58ad49b40355470",
            ),
            (
                SMALL_PACK_INDEX,
                index::Version::V2,
                42,
                "544a7204a55f6e9cacccf8f6e191ea8f83575de3",
                "0f3ea84cd1bba10c2a03d736a460635082833e59",
            ),
        ] {
            let idx = index::File::at(&fixture_path(path))?;
            assert_eq!(idx.version(), *kind);
            assert_eq!(idx.num_objects(), *num_objects);
            assert_eq!(
                idx.verify_integrity(
                    None::<(_, _, _, fn() -> cache::Never)>,
                    None,
                    progress::Discard.into(),
                    Default::default()
                )
                .map(|(a, b, _)| (a, b))?,
                (idx.index_checksum(), None)
            );
            assert_eq!(idx.index_checksum(), hex_to_id(index_checksum));
            assert_eq!(idx.pack_checksum(), hex_to_id(pack_checksum));
            assert_eq!(idx.iter().count(), *num_objects as usize);
        }
        Ok(())
    }
}
