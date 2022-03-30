use std::convert::TryInto;

#[test]
fn file_name() {
    let name: git_ref::FullName = "refs/heads/main".try_into().unwrap();
    assert_eq!(name.to_ref().file_name(), "main");
}
#[test]
fn strip_prefix() {
    for (input, expected) in [
        ("refs/tags/tag-name", "tag-name"),
        ("refs/heads/main", "main"),
        ("refs/remotes/origin/main", "origin/main"),
        ("refs/notes/note-name", "notes/note-name"),
    ] {
        let name: git_ref::FullName = input.try_into().unwrap();
        assert_eq!(name.to_ref().strip_prefix(), expected);
        assert_eq!(name.strip_prefix(), expected);
    }

    let special = "HEAD";
    let name: git_ref::FullName = special.try_into().unwrap();
    assert_eq!(
        name.strip_prefix(),
        special,
        "the whole name is returned if there is no prefix"
    );
    assert_eq!(name.strip_prefix(), name.to_ref().strip_prefix());
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
