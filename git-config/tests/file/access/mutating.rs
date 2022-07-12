mod mutable_section {
    mod push {
        use git_config::parse::section::Key;
        use std::borrow::Cow;
        use std::convert::TryFrom;

        #[test]
        fn push_splits_values_into_events() {
            let mut file = git_config::File::default();
            let mut section = file.new_section("core", None).unwrap();
            section.push(Key::try_from("value").unwrap(), Cow::Borrowed("none".into()));
            assert_eq!(file.to_bstring(), "[core]\n  value=none\n");
        }
    }
}

mod rename_section {
    use git_config::file::rename_section;
    use git_config::parse::section;
    use std::borrow::Cow;
    use std::convert::TryFrom;

    #[test]
    fn section_renaming_validates_new_name() {
        let mut file = git_config::File::try_from("[core] a = b").unwrap();
        assert!(matches!(
            file.rename_section("core", None, "new_core", None),
            Err(rename_section::Error::Section(section::header::Error::InvalidName))
        ));

        assert!(matches!(
            file.rename_section("core", None, "new-core", Cow::from("a\nb")),
            Err(rename_section::Error::Section(
                section::header::Error::InvalidSubSection
            ))
        ));
    }
}
