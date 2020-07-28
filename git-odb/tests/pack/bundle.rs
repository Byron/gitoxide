mod locate {
    use crate::{fixture_path, hex_to_id, pack::SMALL_PACK_INDEX};
    use bstr::ByteSlice;
    use git_object::Kind;
    use git_odb::pack;

    fn locate<'a>(hex_id: &str, out: &'a mut Vec<u8>) -> pack::Object<'a> {
        let bundle = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
        bundle
            .locate(hex_to_id(hex_id).to_borrowed(), out, &mut pack::cache::DecodeEntryNoop)
            .unwrap()
            .unwrap()
    }

    mod locate_and_verify {
        use crate::{
            fixture_path,
            pack::{INDEX_V1, PACK_FOR_INDEX_V1, SMALL_PACK, SMALL_PACK_INDEX},
        };
        use git_odb::pack;

        #[test]
        fn all() {
            for (index_path, data_path) in &[(SMALL_PACK_INDEX, SMALL_PACK), (INDEX_V1, PACK_FOR_INDEX_V1)] {
                // both paths are equivalent
                pack::Bundle::at(fixture_path(index_path)).unwrap();
                let bundle = pack::Bundle::at(fixture_path(data_path)).unwrap();

                let mut buf = Vec::new();
                for entry in bundle.index.iter() {
                    let obj = bundle
                        .locate(entry.oid.to_borrowed(), &mut buf, &mut pack::cache::DecodeEntryNoop)
                        .unwrap()
                        .unwrap();
                    obj.verify_checksum(entry.oid.to_borrowed()).unwrap();
                }
            }
        }
    }

    #[test]
    fn blob() {
        let mut out = Vec::new();
        let obj = locate("bd46bb3f5bb4ca5431770c4fde0735fb89d382f3", &mut out);

        assert_eq!(
            obj.data.as_bstr(),
            b"GitPython is a python library used to interact with Git repositories.\n\nHi there\n".as_bstr()
        );
        assert_eq!(obj.kind, Kind::Blob);
        let object = obj.decode().unwrap();
        assert_eq!(object.kind(), Kind::Blob);
        assert_eq!(object.as_blob().unwrap().data, obj.data);
    }

    #[test]
    fn tree() {
        let mut out = Vec::new();
        let obj = locate("e90926b07092bccb7bf7da445fae6ffdfacf3eae", &mut out);

        assert_eq!(obj.kind, Kind::Tree);
        assert_eq!(obj.decode().unwrap().kind(), Kind::Tree);
    }

    #[test]
    fn commit() {
        let mut out = Vec::new();
        let obj = locate("779c5451ba9fe210ffd1f55db202e55f51acecac", &mut out);

        assert_eq!(obj.kind, Kind::Commit);
        assert_eq!(obj.decode().unwrap().kind(), Kind::Commit);
    }
}
