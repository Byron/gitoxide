#[test]
fn with() {
    git_diff::text::with(
        "old".into(),
        "new".into(),
        git_diff::text::Algorithm::Myers,
        git_diff::text::imara::intern::InternedInput::new,
        |input| git_diff::text::imara::UnifiedDiffBuilder::new(input),
    );
}
