mod blob;
mod commit;
mod tree;

#[test]
fn object_ref_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<gix::Object<'_>>(),
        56,
        "the size of this structure should not changed unexpectedly"
    )
}

#[test]
fn oid_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<gix::Id<'_>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
