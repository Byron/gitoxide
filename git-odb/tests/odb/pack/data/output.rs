use git_odb::pack::data::output;

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
        80,
        "The size of the structure shouldn't change unexpectedly"
    )
}

mod entries {
    mod simple_compression {
        use crate::{fixture_path, odb::hex_to_id};
        use git_features::progress;
        use git_odb::{compound, linked, pack, pack::data::output, FindExt};
        use git_traverse::commit;
        use std::{path::PathBuf, sync::Arc};

        enum DbKind {
            AbunchOfRandomObjects,
            DeterministicGeneratedContent,
        }

        fn db(kind: DbKind) -> crate::Result<Arc<linked::Db>> {
            use DbKind::*;
            let path: PathBuf = match kind {
                AbunchOfRandomObjects => fixture_path("objects"),
                DeterministicGeneratedContent => {
                    git_testtools::scripted_fixture_repo_read_only("make_pack_gen_repo.sh")?
                        .join(".git")
                        .join("objects")
                }
            };
            linked::Db::at(path).map_err(Into::into).map(Into::into)
        }

        #[test]
        fn all_input_objects_as_is() -> crate::Result {
            let db = db(DbKind::AbunchOfRandomObjects)?;
            let obj_count = db.iter().count();
            assert_eq!(obj_count, 146);
            let all_objects = db.arc_iter().flat_map(Result::ok);
            let entries: Vec<_> = output::objects_to_entries_iter(
                db.clone(),
                || pack::cache::Never,
                all_objects,
                progress::Discard,
                output::objects_to_entries::Options {
                    input_object_expansion: output::objects_to_entries::ObjectExpansion::AsIs,
                    ..output::objects_to_entries::Options::default()
                },
            )
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
            assert_eq!(entries.len(), obj_count, "each object gets one entry");
            assert!(
                entries
                    .iter()
                    .find(|e| !matches!(e.kind, output::entry::Kind::Base))
                    .is_none(),
                "there should only be base entries"
            );

            write_and_verify(entries)?;
            Ok(())
        }

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
            for (expansion_mode, expected_count) in [
                (
                    output::objects_to_entries::ObjectExpansion::AsIs,
                    Count {
                        trees: 0,
                        commits: 15,
                        blobs: 0,
                        tags: 1,
                    },
                ),
                (output::objects_to_entries::ObjectExpansion::TreeContents, whole_pack),
                (
                    output::objects_to_entries::ObjectExpansion::TreeAdditionsComparedToAncestor,
                    whole_pack,
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
                let entries: Vec<_> = output::objects_to_entries_iter(
                    db.clone(),
                    || pack::cache::Never,
                    commits.chain(std::iter::once(hex_to_id("e3fb53cbb4c346d48732a24f09cf445e49bc63d6"))),
                    progress::Discard,
                    output::objects_to_entries::Options {
                        input_object_expansion: expansion_mode,
                        ..output::objects_to_entries::Options::default()
                    },
                )
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();
                let actual_count = entries.iter().fold(Count::default(), |mut c, e| {
                    c.add(e.object_kind);
                    c
                });
                assert_eq!(actual_count, expected_count);
                assert_eq!(entries.len(), expected_count.total());
                write_and_verify(entries)?;
            }
            Ok(())
        }

        fn write_and_verify(entries: Vec<output::Entry>) -> crate::Result {
            let tmp_dir = tempfile::TempDir::new()?;
            let pack_file_path = tmp_dir.path().join("new.pack");
            let mut pack_file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&pack_file_path)?;
            let num_written_bytes = {
                let num_entries = entries.len();
                let mut pack_writer = output::entries_to_bytes::EntriesToBytesIter::new(
                    std::iter::once(Ok::<_, output::objects_to_entries::Error<compound::find::Error>>(
                        entries,
                    )),
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
                let _ = pack_writer.input;
                let _ = pack_writer.into_write();
                n
            };
            assert_eq!(
                num_written_bytes,
                pack_file.metadata()?.len(),
                "it reports the correct amount of written bytes"
            );
            let pack = pack::data::File::at(&pack_file_path)?;
            pack.verify_checksum(progress::Discard)?;

            // Re-generate the index from the pack for validation.
            let bundle = pack::Bundle::at(
                pack::Bundle::write_to_directory(
                    std::io::BufReader::new(std::fs::File::open(pack_file_path)?),
                    Some(tmp_dir.path()),
                    progress::Discard,
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
            )?;
            Ok(())
        }
    }
}
