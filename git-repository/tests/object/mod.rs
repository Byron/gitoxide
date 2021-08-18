use git_repository::ObjectRef;
use git_repository::Oid;

#[test]
fn object_ref_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<ObjectRef<'_, git_repository::Easy>>(),
        56,
        "the size of this structure should not changed unexpectedly"
    )
}

#[test]
fn oid_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Oid<'_, git_repository::Easy>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
