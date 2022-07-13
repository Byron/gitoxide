mod section {
    mod push {
        use git_config::parse::section::Key;
        use std::borrow::Cow;
        use std::convert::TryFrom;

        #[test]
        fn push_splits_values_into_events() {
            let mut file = git_config::File::default();
            let mut section = file.new_section("core", None).unwrap();
            section.push(Key::try_from("value").unwrap(), Cow::Borrowed("none".into()));
            assert_eq!(file.to_bstring(), "[core]\n        value=none\n");
        }
    }
}
