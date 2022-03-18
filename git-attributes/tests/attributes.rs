mod parse {
    mod ignore {
        #[test]
        fn comments_are_ignored() {
            assert!(git_attributes::parse::ignore(b"# hello world").next().is_none());
        }
    }
}
