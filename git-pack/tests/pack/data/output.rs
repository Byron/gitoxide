use std::{path::PathBuf, sync::Arc};

use git_odb::linked;
use git_pack::data::output;

#[test]
fn size_of_entry() {
    assert_eq!(
        std::mem::size_of::<output::Entry>(),
        80,
        "The size of the structure shouldn't change unexpectedly"
    )
}

#[test]
fn size_of_count() {
    assert_eq!(
        std::mem::size_of::<output::Count>(),
        48,
        "The size of the structure shouldn't change unexpectedly"
    )
}

enum DbKind {
    DeterministicGeneratedContent,
}

fn db(kind: DbKind) -> crate::Result<Arc<linked::Store>> {
    use DbKind::*;
    let path: PathBuf = match kind {
        DeterministicGeneratedContent => git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo.sh")?
            .join(".git")
            .join("objects"),
    };
    linked::Store::at(path).map_err(Into::into).map(Into::into)
}

mod count_and_entries {
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
    fn traversals() -> crate::Result {
        let db = db(DbKind::DeterministicGeneratedContent)?;
        #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
        struct Count {
            trees: usize,
            commits: usize,
            blobs: usize,
            tags: usize,
        }
        impl Count {
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
        let whole_pack = Count {
            trees: 40,
            commits: 16,
            blobs: 811,
            tags: 1,
        };
        for (expansion_mode, expected_count, expected_counts_outcome, expected_entries_outcome, expected_pack_hash) in [
            (
                count::from_objects_iter::ObjectExpansion::AsIs,
                Count {
                    trees: 0,
                    commits: 15,
                    blobs: 0,
                    tags: 1,
                },
                output::count::from_objects_iter::Outcome {
                    input_objects: 16,
                    expanded_objects: 0,
                    decoded_objects: 16,
                    total_objects: 16,
                },
                output::entry::from_counts_iter::Outcome {
                    decoded_and_recompressed_objects: 0,
                    objects_copied_from_pack: 16,
                },
                hex_to_id("a84ddea36a6504a7385761ede0ccc8eb4451392e"),
            ),
            (
                count::from_objects_iter::ObjectExpansion::TreeContents,
                whole_pack,
                output::count::from_objects_iter::Outcome {
                    input_objects: 16,
                    expanded_objects: 852,
                    decoded_objects: 57,
                    total_objects: 868,
                },
                output::entry::from_counts_iter::Outcome {
                    decoded_and_recompressed_objects: 542,
                    objects_copied_from_pack: 326,
                },
                hex_to_id("c0f566c050fc5ff41d1b68cc56e13f5aa96c2df7"),
            ),
            (
                count::from_objects_iter::ObjectExpansion::TreeAdditionsComparedToAncestor,
                whole_pack,
                output::count::from_objects_iter::Outcome {
                    input_objects: 16,
                    expanded_objects: 866,
                    decoded_objects: 208,
                    total_objects: 868,
                },
                output::entry::from_counts_iter::Outcome {
                    decoded_and_recompressed_objects: 542,
                    objects_copied_from_pack: 326,
                },
                hex_to_id("e3134a132fd77335d4b99f60f8ed3698d4babc89"),
            ),
        ]
        .iter()
        .copied()
        {
            let head = hex_to_id("dfcb5e39ac6eb30179808bbab721e8a28ce1b52e");
            let commits = commit::Ancestors::new(Some(head), commit::ancestors::State::default(), {
                let db = Arc::clone(&db);
                move |oid, buf| db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok()
            })
            .map(Result::unwrap);

            let deterministic_count_needs_single_thread = Some(1);
            let mut counts_iter = output::count::from_objects_iter(
                db.clone(),
                || pack::cache::Never,
                commits.chain(std::iter::once(hex_to_id("e3fb53cbb4c346d48732a24f09cf445e49bc63d6"))),
                progress::Discard,
                count::from_objects_iter::Options {
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
            let actual_count = counts.iter().fold(Count::default(), |mut c, e| {
                let mut buf = Vec::new();
                if let Some(obj) = db.find_existing(e.id, &mut buf, &mut pack::cache::Never).ok() {
                    c.add(obj.kind);
                }
                c
            });
            assert_eq!(actual_count, expected_count);
            let counts_len = counts.len();
            assert_eq!(counts_len, expected_count.total());

            let stats = counts_iter.finalize()?;
            assert_eq!(stats, expected_counts_outcome);
            assert_eq!(stats.total_objects, expected_count.total());

            let mut entries_iter = output::entry::from_counts_iter(
                counts,
                db.clone(),
                || pack::cache::Never,
                progress::Discard,
                output::entry::from_counts_iter::Options::default(),
            );
            let entries: Vec<_> = output::InOrderIter::from(entries_iter.by_ref())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();
            let actual_count = entries.iter().fold(Count::default(), |mut c, e| {
                c.add(e.object_kind);
                c
            });
            assert_eq!(actual_count, expected_count);
            assert_eq!(counts_len, expected_count.total());
            let stats = entries_iter.finalize()?;
            assert_eq!(stats, expected_entries_outcome);

            write_and_verify(entries, expected_pack_hash)?;
        }
        Ok(())
    }

    fn write_and_verify(entries: Vec<output::Entry>, _expected_pack_hash: git_hash::ObjectId) -> crate::Result {
        let tmp_dir = tempfile::TempDir::new()?;
        let pack_file_path = tmp_dir.path().join("new.pack");
        let mut pack_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&pack_file_path)?;
        let (num_written_bytes, pack_hash) = {
            let num_entries = entries.len();
            let mut pack_writer = output::bytes::FromEntriesIter::new(
                std::iter::once(Ok::<_, entry::from_counts_iter::Error<compound::find::Error>>(entries)),
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

        assert_eq!(hash, _expected_pack_hash, "pack hashes are stable if the input is");

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
        Ok(())
    }
}

mod in_order_iter {
    use git_odb::data::output::InOrderIter;
    use std::convert::Infallible;

    #[test]
    fn in_order_stays_in_order() {
        assert_eq!(
            InOrderIter::from(vec![Ok::<_, Infallible>((0usize, 'a')), Ok((1, 'b')), Ok((2, 'c'))].into_iter())
                .collect::<Result<Vec<_>, _>>()
                .expect("infallible"),
            vec!['a', 'b', 'c']
        )
    }

    #[test]
    fn out_of_order_items_are_held_until_the_sequence_is_complete() {
        assert_eq!(
            InOrderIter::from(
                vec![
                    Ok::<_, Infallible>((2usize, 'c')),
                    Ok((1, 'b')),
                    Ok((0, 'a')),
                    Ok((3, 'd'))
                ]
                .into_iter()
            )
            .collect::<Result<Vec<_>, _>>()
            .expect("infallible"),
            vec!['a', 'b', 'c', 'd']
        )
    }

    #[test]
    fn in_sequence_errors_immediately_trigger_a_fuse() {
        let mut iter =
            InOrderIter::from(vec![Ok::<_, &'static str>((0usize, 'a')), Err("err"), Ok((1, 'b'))].into_iter());
        assert_eq!(iter.next(), Some(Ok('a')));
        assert_eq!(iter.next(), Some(Err("err")));
        assert_eq!(
            iter.next(),
            None,
            "fuse should have triggered so we don't see anything else"
        );
    }

    #[test]
    fn out_of_sequence_errors_immediately_trigger_a_fuse() {
        let mut iter =
            InOrderIter::from(vec![Ok::<_, &'static str>((1usize, 'b')), Err("err"), Ok((0, 'a'))].into_iter());
        assert_eq!(iter.next(), Some(Err("err")));
        assert_eq!(
            iter.next(),
            None,
            "fuse should have triggered so we don't see anything else"
        );
    }
}
