use git_repository::Object;

mod data {
    use git_repository::object::Data;

    #[test]
    fn size_in_memory() {
        assert_eq!(
            std::mem::size_of::<Data<'_>>(),
            32,
            "the size of this structure should not changed unexpectedly"
        )
    }
}

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Object<'_, git_repository::Easy>>(),
        32,
        "the size of this structure should not changed unexpectedly"
    )
}
