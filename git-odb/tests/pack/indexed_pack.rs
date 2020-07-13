mod locate {
    use crate::{fixture_path, hex_to_id, pack::SMALL_PACK_INDEX};
    use bstr::ByteSlice;
    use git_object::Kind;
    use git_odb::pack;

    fn locate<'a>(hex_id: &str, out: &'a mut Vec<u8>) -> pack::Object<'a> {
        let idx = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
        idx.locate(&hex_to_id(hex_id), out, &mut pack::cache::DecodeEntryNoop)
            .unwrap()
            .unwrap()
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
