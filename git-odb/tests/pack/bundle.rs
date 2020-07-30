mod locate {
    use crate::{fixture_path, hex_to_id, pack::SMALL_PACK_INDEX};
    use bstr::ByteSlice;
    use git_object::Kind;
    use git_odb::pack;

    fn locate<'a>(hex_id: &str, out: &'a mut Vec<u8>) -> pack::Object<'a> {
        let bundle = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).expect("pack and idx");
        bundle
            .locate(hex_to_id(hex_id).to_borrowed(), out, &mut pack::cache::DecodeEntryNoop)
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
                        .locate(entry.oid.to_borrowed(), &mut buf, &mut pack::cache::DecodeEntryNoop)
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
