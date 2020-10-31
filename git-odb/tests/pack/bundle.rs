mod locate {
    use crate::{fixture_path, hex_to_id, pack::SMALL_PACK_INDEX};
    use bstr::ByteSlice;
    use git_object::Kind;
    use git_odb::pack;

    fn locate<'a>(hex_id: &str, out: &'a mut Vec<u8>) -> git_odb::borrowed::Object<'a> {
        let bundle = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).expect("pack and idx");
        bundle
            .locate(hex_to_id(hex_id).to_borrowed(), out, &mut pack::cache::Noop)
            .expect("id present")
            .expect("read success")
    }

    mod locate_and_verify {
        use crate::{fixture_path, pack::PACKS_AND_INDICES};
        use git_odb::pack;

        #[test]
        fn all() -> Result<(), Box<dyn std::error::Error>> {
            for (index_path, data_path) in PACKS_AND_INDICES {
                // both paths are equivalent
                pack::Bundle::at(fixture_path(index_path))?;
                let bundle = pack::Bundle::at(fixture_path(data_path))?;

                let mut buf = Vec::new();
                for entry in bundle.index.iter() {
                    let obj = bundle
                        .locate(entry.oid.to_borrowed(), &mut buf, &mut pack::cache::Noop)
                        .expect("id present")?;
                    obj.verify_checksum(entry.oid.to_borrowed())?;
                }
            }
            Ok(())
        }
    }

    #[test]
    fn blob() -> Result<(), Box<dyn std::error::Error>> {
        let mut out = Vec::new();
        let obj = locate("bd46bb3f5bb4ca5431770c4fde0735fb89d382f3", &mut out);

        assert_eq!(
            obj.data.as_bstr(),
            b"GitPython is a python library used to interact with Git repositories.\n\nHi there\n".as_bstr()
        );
        assert_eq!(obj.kind, Kind::Blob);
        let object = obj.decode()?;
        assert_eq!(object.kind(), Kind::Blob);
        assert_eq!(object.as_blob().expect("blob").data, obj.data);
        Ok(())
    }

    #[test]
    fn tree() -> Result<(), Box<dyn std::error::Error>> {
        let mut out = Vec::new();
        let obj = locate("e90926b07092bccb7bf7da445fae6ffdfacf3eae", &mut out);

        assert_eq!(obj.kind, Kind::Tree);
        assert_eq!(obj.decode()?.kind(), Kind::Tree);
        Ok(())
    }

    #[test]
    fn commit() -> Result<(), Box<dyn std::error::Error>> {
        let mut out = Vec::new();
        let obj = locate("779c5451ba9fe210ffd1f55db202e55f51acecac", &mut out);

        assert_eq!(obj.kind, Kind::Commit);
        assert_eq!(obj.decode()?.kind(), Kind::Commit);
        Ok(())
    }
}

mod write_to_directory {
    use crate::pack::SMALL_PACK_INDEX;
    use crate::{fixture_path, pack::SMALL_PACK};
    use git_features::progress;
    use git_object::owned;
    use git_odb::pack::{self, bundle};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn expected_outcome() -> Result<bundle::write::Outcome, Box<dyn std::error::Error>> {
        Ok(pack::bundle::write::Outcome {
            index: pack::index::write::Outcome {
                index_kind: pack::index::Kind::V2,
                index_hash: owned::Id::from_40_bytes_in_hex(b"544a7204a55f6e9cacccf8f6e191ea8f83575de3")?,
                data_hash: owned::Id::from_40_bytes_in_hex(b"0f3ea84cd1bba10c2a03d736a460635082833e59")?,
                num_objects: 42,
            },
            pack_kind: pack::data::Kind::V2,
            index_path: None,
            data_path: None,
        })
    }

    #[test]
    fn without_providing_one() -> Result<(), Box<dyn std::error::Error>> {
        let res = write_pack(None::<&Path>, SMALL_PACK)?;
        assert_eq!(res, expected_outcome()?);
        assert_eq!(
            res.index.index_hash,
            pack::index::File::at(fixture_path(SMALL_PACK_INDEX))?.index_checksum()
        );
        assert!(res.to_bundle().is_none());
        Ok(())
    }

    #[test]
    fn given_a_directory() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let mut res = write_pack(Some(&dir), SMALL_PACK)?;
        let (index_path, data_path) = (res.index_path.take(), res.data_path.take());
        assert_eq!(res, expected_outcome()?);
        let mut sorted_entries = fs::read_dir(&dir)?.filter_map(Result::ok).collect::<Vec<_>>();
        sorted_entries.sort_by_key(|e| e.file_name());
        assert_eq!(sorted_entries.len(), 2, "we want a pack and the corresponding index");

        let pack_hash = res.index.data_hash.to_sha1_hex_string();
        assert_eq!(file_name(&sorted_entries[0]), format!("{}.idx", pack_hash));
        assert_eq!(Some(sorted_entries[0].path()), index_path);

        assert_eq!(file_name(&sorted_entries[1]), format!("{}.pack", pack_hash));
        assert_eq!(Some(sorted_entries[1].path()), data_path);

        res.index_path = index_path;
        assert!(res.to_bundle().transpose()?.is_some());
        Ok(())
    }

    fn file_name(entry: &fs::DirEntry) -> String {
        entry.path().file_name().unwrap().to_str().unwrap().to_owned()
    }

    fn write_pack(
        directory: Option<impl AsRef<Path>>,
        pack_file: &str,
    ) -> Result<bundle::write::Outcome, Box<dyn std::error::Error>> {
        let pack_file = fs::File::open(fixture_path(pack_file))?;
        pack::Bundle::write_to_directory_eagerly(
            pack_file,
            None,
            directory,
            progress::Discard,
            bundle::write::Options {
                thread_limit: None,
                iteration_mode: pack::data::iter::Mode::Verify,
                index_kind: pack::index::Kind::V2,
            },
        )
        .map_err(Into::into)
    }
}
