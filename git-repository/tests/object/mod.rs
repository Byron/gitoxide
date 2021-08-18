use git_repository::Oid;

mod data {
    use git_repository::object::DetachedObject;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<DetachedObject<'_>>(),
            32,
            "the size of this structure should not changed unexpectedly"
        )
    }
}

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Oid<'_, git_repository::Easy>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
