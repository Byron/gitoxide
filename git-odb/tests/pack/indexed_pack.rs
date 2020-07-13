use crate::pack::SMALL_PACK_INDEX;
use crate::{fixture_path, hex_to_id};
use bstr::ByteSlice;
use git_odb::pack;

#[test]
fn locate() {
    let idx = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
    let mut out = Vec::new();
    // cache: &mut impl cache::DecodeEntry,

    let obj = idx
        .locate(
            &hex_to_id("bd46bb3f5bb4ca5431770c4fde0735fb89d382f3"),
            &mut out,
            &mut pack::cache::DecodeEntryNoop,
        )
        .unwrap()
        .unwrap();
    assert_eq!(
        obj.data.as_bstr(),
        b"GitPython is a python library used to interact with Git repositories.\n\nHi there\n".as_bstr()
    );
    // obj.decode()
}
