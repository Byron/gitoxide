use std::convert::TryInto;

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
