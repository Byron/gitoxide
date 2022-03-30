use git_ref::Category;
use std::convert::TryInto;

#[test]
fn file_name() {
    let name: git_ref::FullName = "refs/heads/main".try_into().unwrap();
    assert_eq!(name.to_ref().file_name(), "main");
}
#[test]
fn shorten_and_category() {
    for (input, expected, category) in [
        ("refs/tags/tag-name", "tag-name", Category::Tag),
        ("refs/heads/main", "main", Category::LocalBranch),
        ("refs/remotes/origin/main", "origin/main", Category::RemoteBranch),
        ("refs/notes/note-name", "notes/note-name", Category::Note),
    ] {
        let name: git_ref::FullName = input.try_into().unwrap();
        let category = Some(category);
        assert_eq!(name.to_ref().shorten(), expected);
        assert_eq!(name.shorten(), expected);
        assert_eq!(name.category(), category);
        assert_eq!(name.to_ref().category(), category);
    }

    let special = "HEAD";
    let name: git_ref::FullName = special.try_into().unwrap();
    assert_eq!(
        name.shorten(),
        special,
        "the whole name is returned if there is no prefix"
    );
    assert_eq!(name.category(), None);
}

#[test]
fn prefix_with_namespace_and_stripping() {
    let ns = git_ref::namespace::expand("foo").unwrap();
    let mut name: git_ref::FullName = "refs/heads/main".try_into().unwrap();
    assert_eq!(
        name.prefix_namespace(&ns).as_bstr(),
        "refs/namespaces/foo/refs/heads/main"
    );
    assert_eq!(
        name.prefix_namespace(&ns).as_bstr(),
        "refs/namespaces/foo/refs/heads/main",
        "idempotent prefixing"
    );
    assert_eq!(name.strip_namespace(&ns).as_bstr(), "refs/heads/main");
    assert_eq!(
        name.strip_namespace(&ns).as_bstr(),
        "refs/heads/main",
        "idempotent stripping"
    );
}
