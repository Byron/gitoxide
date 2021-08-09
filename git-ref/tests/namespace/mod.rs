mod expand {
    #[test]
    fn each_component_expands_to_the_namespace_prefix_individually() {
        assert_eq!(
            git_ref::namespace::expand("foo/bar").unwrap().as_bstr(),
            "refs/namespaces/foo/refs/namespaces/bar"
        )
    }

    #[test]
    fn backslashes_are_no_component_separators_and_invalid() {
        assert!(matches!(
            git_ref::namespace::expand("foo\\bar").expect_err("empty invalid"),
            git_validate::refname::Error::Tag(
                git_validate::tag::name::Error::InvalidByte(byte)
            ) if byte == "\\"
        ));
    }

    #[test]
    fn trailing_slashes_are_not_allowed() {
        assert!(matches!(
            git_ref::namespace::expand("foo/").expect_err("empty invalid"),
            git_validate::refname::Error::Tag(git_validate::tag::name::Error::EndsWithSlash)
        ));
    }

    #[test]
    fn empty_namespaces_are_not_allowed() {
        assert!(matches!(
            git_ref::namespace::expand("").expect_err("empty invalid"),
            git_validate::refname::Error::Tag(git_validate::tag::name::Error::Empty)
        ));
    }
    #[test]
    fn bare_slashes_are_not_allowed() {
        assert!(matches!(
            git_ref::namespace::expand("/").expect_err("empty invalid"),
            git_validate::refname::Error::Tag(git_validate::tag::name::Error::EndsWithSlash)
        ));
    }
    #[test]
    fn repeated_slashes_are_invalid() {
        assert!(matches!(
            git_ref::namespace::expand("foo//bar").expect_err("empty invalid"),
            git_validate::refname::Error::RepeatedSlash
        ));
    }
}
