use crate::pack::SMALL_PACK_INDEX;
use crate::{fixture_path, hex_to_id};
use git_odb::pack;

#[test]
fn locate() {
    let idx = pack::Bundle::at(fixture_path(SMALL_PACK_INDEX)).unwrap();
    let mut out = Vec::new();
    pack::cache::DecodeEntryNoop;
    // cache: &mut impl cache::DecodeEntry,

    let obj = idx
        .locate(
            &hex_to_id("531ea8f97a99eee41a7678d94f14d0dba6587c66"),
            &mut out,
            pack::cache::DecodeEntryNoop,
        )
        .unwrap();
    // obj.decode()
}
