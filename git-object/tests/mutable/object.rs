use git_object::Object;

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Object>(),
        264,
        "Prevent unexpected growth of what should be lightweight objects"
    )
}
