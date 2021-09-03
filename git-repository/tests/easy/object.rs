use git_repository::easy;

#[test]
fn object_ref_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<easy::ObjectRef<'_, git_repository::Easy>>(),
        56,
        "the size of this structure should not changed unexpectedly"
    )
}

#[test]
fn oid_size_in_memory() {
    assert_eq!(
        std::mem::size_of::<easy::Oid<'_, git_repository::Easy>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
