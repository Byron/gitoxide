mod entries {
    mod simple_compression {
        use crate::fixture_path;
        use git_features::progress;
        use git_odb::{compound, linked, pack, pack::data::output};
        use std::{path::PathBuf, sync::Arc};

        enum DbKind {
            AbunchOfRandomObjects,
        }

        fn db(kind: DbKind) -> crate::Result<Arc<linked::Db>> {
            use DbKind::*;
            let path: PathBuf = match kind {
                AbunchOfRandomObjects => fixture_path("objects"),
            };
            linked::Db::at(path).map_err(Into::into).map(Into::into)
        }

        #[test]
        fn all_input_objects() -> crate::Result {
            let db = db(DbKind::AbunchOfRandomObjects)?;
            let obj_count = db.iter().count();
            assert_eq!(obj_count, 146);
            let all_objects = db.arc_iter().flat_map(Result::ok);
            let entries: Vec<_> = output::objects_to_entries_iter(
                db.clone(),
                || pack::cache::Never,
                all_objects,
                progress::Discard,
                output::Options::default(),
            )
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
            assert_eq!(entries.len(), obj_count, "each object gets one entry");
            assert!(
                entries
                    .iter()
                    .find(|e| !matches!(e.entry_kind, output::entry::Kind::Base))
                    .is_none(),
                "there should only be base entries"
            );

            let tmp_dir = tempfile::TempDir::new()?;
            let pack_file_path = tmp_dir.path().join("new.pack");
            let mut pack_file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&pack_file_path)?;
            let num_written_bytes = {
                let num_entries = entries.len();
                let mut pack_writer = output::entries_to_bytes::EntriesToBytesIter::new(
                    std::iter::once(Ok::<_, output::Error<compound::locate::Error>>(entries)),
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
                pack::Bundle::write_stream_to_directory(
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
