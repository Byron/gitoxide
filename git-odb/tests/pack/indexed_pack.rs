mod locate {

    use crate::pack::SMALL_PACK_INDEX;
    use crate::{fixture_path, hex_to_id};
    use bstr::ByteSlice;
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
        assert_eq!(obj.kind, git_object::Kind::Blob);
        assert_eq!(obj.decode().unwrap().as_blob().unwrap().data, obj.data);
    }
}
