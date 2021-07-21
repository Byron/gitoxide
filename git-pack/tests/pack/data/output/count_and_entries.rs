use std::sync::Arc;

use crate::pack::{
    data::output::{db, DbKind},
    hex_to_id,
};
use git_features::{parallel::reduce::Finalize, progress};
use git_odb::{compound, pack, FindExt};
use git_pack::data::{
    output,
    output::{count, entry},
};
use git_traverse::commit;
use std::sync::atomic::AtomicBool;

#[test]
#[ignore]
fn traversals() -> crate::Result {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    struct Count {
        trees: usize,
        commits: usize,
        blobs: usize,
        tags: usize,
        delta_ref: usize,
        delta_oid: usize,
    }
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    struct ObjectCount {
        trees: usize,
        commits: usize,
        blobs: usize,
        tags: usize,
    }
    impl ObjectCount {
        fn total(&self) -> usize {
            self.tags + self.trees + self.commits + self.blobs
        }
        fn add(&mut self, kind: git_object::Kind) {
            use git_object::Kind::*;
            match kind {
                Tree => self.trees += 1,
                Commit => self.commits += 1,
                Blob => self.blobs += 1,
                Tag => self.tags += 1,
            }
        }
    }
    impl Count {
        fn total(&self) -> usize {
            self.tags + self.trees + self.commits + self.blobs + self.delta_ref + self.delta_oid
        }
        fn add(&mut self, kind: output::entry::Kind) {
            use git_object::Kind::*;
            use output::entry::Kind::*;
            match kind {
                Base(Tree) => self.trees += 1,
                Base(Commit) => self.commits += 1,
                Base(Blob) => self.blobs += 1,
                Base(Tag) => self.tags += 1,
                DeltaRef { .. } => self.delta_ref += 1,
                DeltaOid { .. } => self.delta_oid += 1,
            }
        }
    }
    let whole_pack = Count {
        trees: 21,
        commits: 16,
        blobs: 288,
        tags: 1,
        delta_ref: 542,
        delta_oid: 0, // these are basically none-existing in non-legacy packs, but are used only in thin packs on the wire
    };
    let whole_pack_obj_count = ObjectCount {
        trees: 40,
        commits: 16,
        blobs: 811,
        tags: 1,
    };
    let db = db(DbKind::DeterministicGeneratedContent)?;
    let some_input_objects = 1;
    for (
        expansion_mode,
        expected_count,
        expected_obj_count,
        expected_counts_outcome,
        expected_entries_outcome,
        expected_pack_hash,
        allow_thin_pack,
    ) in [
        (
            count::iter_from_objects::ObjectExpansion::AsIs,
            Count {
                trees: 0,
                commits: 15,
                blobs: 0,
                tags: 1,
                delta_ref: 0,
                delta_oid: 0,
            },
            ObjectCount {
                trees: 0,
                commits: 15,
                blobs: 0,
                tags: 1,
            },
            output::count::iter_from_objects::Outcome {
                input_objects: 16,
                expanded_objects: 0,
                decoded_objects: 16,
                total_objects: 16,
            },
            output::entry::iter_from_counts::Outcome {
                decoded_and_recompressed_objects: 0,
                objects_copied_from_pack: 16,
            },
            hex_to_id("b920bbb055e1efb9080592a409d3975738b6efb3"),
            false,
        ),
        (
            // TODO: trigger thin pack code
            count::iter_from_objects::ObjectExpansion::TreeContents,
            Count {
                trees: 8,
                commits: 2, // todo: why more?
                blobs: 158,
                tags: 1,
                delta_ref: 444,
                delta_oid: 0,
            },
            ObjectCount {
                trees: 9,
                commits: 2,
                blobs: 601,
                tags: 1,
            },
            output::count::iter_from_objects::Outcome {
                input_objects: 2, // todo: figure out why its more than expected
                expanded_objects: 611,
                decoded_objects: 12,
                total_objects: 613,
            },
            output::entry::iter_from_counts::Outcome {
                decoded_and_recompressed_objects: 0,
                objects_copied_from_pack: 613,
            },
            hex_to_id("d2ea809066bec3f5f2a38ef4adba7ebd4f2eda22"),
            true,
        ),
        (
            count::iter_from_objects::ObjectExpansion::TreeContents,
            whole_pack,
            whole_pack_obj_count,
            output::count::iter_from_objects::Outcome {
                input_objects: 16,
                expanded_objects: 852,
                decoded_objects: 57,
                total_objects: 868,
            },
            output::entry::iter_from_counts::Outcome {
                decoded_and_recompressed_objects: 0,
                objects_copied_from_pack: 868,
            },
            hex_to_id("d2ea809066bec3f5f2a38ef4adba7ebd4f2eda22"),
            false,
        ),
        (
            count::iter_from_objects::ObjectExpansion::TreeAdditionsComparedToAncestor,
            whole_pack,
            whole_pack_obj_count,
            output::count::iter_from_objects::Outcome {
                input_objects: 16,
                expanded_objects: 866,
                decoded_objects: 208,
                total_objects: 868,
            },
            output::entry::iter_from_counts::Outcome {
                decoded_and_recompressed_objects: 0,
                objects_copied_from_pack: 868,
            },
            hex_to_id("d2ea809066bec3f5f2a38ef4adba7ebd4f2eda22"),
            false,
        ),
    ]
    .iter()
    .copied()
    {
        let head = hex_to_id("dfcb5e39ac6eb30179808bbab721e8a28ce1b52e");
        let mut commits = commit::Ancestors::new(Some(head), commit::ancestors::State::default(), {
            let db = Arc::clone(&db);
            move |oid, buf| db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok()
        })
        .map(Result::unwrap)
        .collect::<Vec<_>>();
        if allow_thin_pack {
            commits.resize(some_input_objects, git_hash::ObjectId::null_sha1());
        }

        let deterministic_count_needs_single_thread = Some(1);
        let mut counts_iter = output::count::iter_from_objects(
            db.clone(),
            || pack::cache::Never,
            commits
                .into_iter()
                .chain(std::iter::once(hex_to_id("e3fb53cbb4c346d48732a24f09cf445e49bc63d6"))),
            progress::Discard,
            count::iter_from_objects::Options {
                input_object_expansion: expansion_mode,
                thread_limit: deterministic_count_needs_single_thread,
                ..Default::default()
            },
        );
        let counts: Vec<_> = counts_iter
            .by_ref()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let actual_count = counts.iter().fold(ObjectCount::default(), |mut c, e| {
            let mut buf = Vec::new();
            if let Some(obj) = db.find_existing(e.id, &mut buf, &mut pack::cache::Never).ok() {
                c.add(obj.kind);
            }
            c
        });
        assert_eq!(actual_count, expected_obj_count);
        let counts_len = counts.len();
        assert_eq!(counts_len, expected_obj_count.total());

        let stats = counts_iter.finalize()?;
        assert_eq!(stats, expected_counts_outcome);
        assert_eq!(stats.total_objects, expected_obj_count.total());

        let mut entries_iter = output::entry::iter_from_counts(
            counts,
            db.clone(),
            || pack::cache::Never,
            progress::Discard,
            output::entry::iter_from_counts::Options {
                allow_thin_pack,
                ..Default::default()
            },
        );
        let entries: Vec<_> = output::InOrderIter::from(entries_iter.by_ref())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let actual_count = entries.iter().fold(Count::default(), |mut c, e| {
            c.add(e.kind);
            c
        });
        assert_eq!(actual_count, expected_count);
        assert_eq!(counts_len, expected_count.total());
        let stats = entries_iter.finalize()?;
        assert_eq!(stats, expected_entries_outcome);

        assert_eq!(
            expected_obj_count.total(),
            expected_count.total(),
            "two different ways of counting, still the same in the end"
        );

        write_and_verify(entries, expected_pack_hash, allow_thin_pack)?;
    }

    Ok(())
}

fn write_and_verify(
    entries: Vec<output::Entry>,
    expected_pack_hash: git_hash::ObjectId,
    allow_thin_pack: bool,
) -> crate::Result {
    let tmp_dir = tempfile::TempDir::new()?;
    let pack_file_path = tmp_dir.path().join("new.pack");
    let mut pack_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pack_file_path)?;
    let (num_written_bytes, pack_hash) = {
        let num_entries = entries.len();
        let mut pack_writer = output::bytes::FromEntriesIter::new(
            std::iter::once(Ok::<_, entry::iter_from_counts::Error<compound::find::Error>>(entries)),
            &mut pack_file,
            num_entries as u32,
            pack::data::Version::V2,
            git_hash::Kind::Sha1,
        );
        let mut n = pack_writer.next().expect("one entries bundle was written")?;
        n += pack_writer.next().expect("the trailer was written")?;
        assert!(
            pack_writer.next().is_none(),
            "there is nothing more to iterate this time"
        );
        // verify we can still get the original parts back
        let hash = pack_writer.digest().expect("digest is available when iterator is done");
        let _ = pack_writer.input;
        let _ = pack_writer.into_write();
        (n, hash)
    };
    assert_eq!(
        num_written_bytes,
        pack_file.metadata()?.len(),
        "it reports the correct amount of written bytes"
    );
    let pack = pack::data::File::at(&pack_file_path)?;
    let should_interrupt = AtomicBool::new(false);
    let hash = pack.verify_checksum(progress::Discard, &should_interrupt)?;
    assert_eq!(
        hash, pack_hash,
        "the trailer of the pack matches the actually written trailer"
    );

    assert_eq!(hash, expected_pack_hash, "pack hashes are stable if the input is");

    // TODO: when thin pack writing is supported, let it fail if it's a thin back even though it shouldn't be
    if !allow_thin_pack {
        // Re-generate the index from the pack for validation.
        let bundle = pack::Bundle::at(
            pack::Bundle::write_to_directory(
                std::io::BufReader::new(std::fs::File::open(pack_file_path)?),
                Some(tmp_dir.path()),
                progress::Discard,
                &should_interrupt,
                pack::bundle::write::Options::default(),
            )?
            .data_path
            .expect("directory set"),
        )?;
        bundle.verify_integrity(
            pack::index::verify::Mode::Sha1Crc32DecodeEncode,
            pack::index::traverse::Algorithm::DeltaTreeLookup,
            || pack::cache::Never,
            None,
            progress::Discard.into(),
            Arc::new(should_interrupt),
        )?;
    }
    Ok(())
}
