#[test]
#[cfg(feature = "gix-features-parallel")]
fn is_send_and_sync() {
    pub fn store_at(name: &str) -> crate::Result<gix_ref::file::Store> {
        let path = gix_testtools::scripted_fixture_read_only_standalone(name)?;
        Ok(gix_ref::file::Store::at(
            path.join(".git"),
            gix_ref::store::init::Options {
                write_reflog: gix_ref::store::WriteReflog::Normal,
                object_hash: gix_hash::Kind::Sha1,
                ..Default::default()
            },
        ))
    }

    pub fn store_with_packed_refs() -> crate::Result<gix_ref::file::Store> {
        store_at("make_packed_ref_repository.sh")
    }
    fn assert_type<T: Send + Sync>(_t: T) {}
    let store = store_with_packed_refs().unwrap();
    assert_type(&store);
    assert_type(store);
}
