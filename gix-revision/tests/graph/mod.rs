#[test]
fn size_of_commit() {
    assert_eq!(
        std::mem::size_of::<gix_revision::graph::Commit<()>>(),
        48,
        "We might see quite a lot of these, so they shouldn't grow unexpectedly"
    )
}
