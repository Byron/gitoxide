mod expand {
    #[test]
    fn each_component_expands_to_the_namespace_prefix_individually() {
        assert_eq!(
            git_ref::namespace::expand("foo/bar").unwrap().as_bstr(),
            "refs/namespaces/foo/refs/namespaces/bar/"
        )
    }

    #[test]
    #[ignore]
    fn only_backslashes_are_valid_component_separators() {}

    #[test]
    #[ignore]
    fn trailing_slashes_do_nothing() {}

    #[test]
    fn empty_namespaces_are_not_allowed() {
        assert!(matches!(
            git_ref::namespace::expand("").expect_err("empty invalid"),
            git_ref::namespace::expand::Error::RefnameValidation(git_ref::name::Error::RefnameValidation {
                err: git_validate::refname::Error::Tag(git_validate::tag::name::Error::Empty),
                ..
            })
        ))
    }
}
