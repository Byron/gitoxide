use crate::{hex_to_id, loose::db::locate};
use git_object::owned;
use git_odb::Write;

fn id_to_hex(id: &owned::Id) -> String {
    std::str::from_utf8(&id.to_sha1_hex()[..]).unwrap().to_owned()
}

#[test]
fn write() {
    let oid = hex_to_id("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f");
    let obj = locate(&id_to_hex(&oid));
    // assert_eq!(git_odb::Sink.write(&obj.into()).unwrap(), oid);
}
