use git_repository::Oid;

mod data {
    use git_repository::Object;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<Object<'_, git_repository::Easy>>(),
            40,
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
