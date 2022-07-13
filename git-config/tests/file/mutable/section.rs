mod push {
    use git_config::parse::section::Key;
    use std::borrow::Cow;
    use std::convert::TryFrom;

    #[test]
    #[ignore]
    fn whitespace_is_derived_from_whitespace_before_first_value() -> crate::Result {
        for (config, expected) in [("[a]\n\t\tb = c", "\t\t")] {
            let mut file: git_config::File = config.parse()?;
            assert_eq!(
                file.section_mut("a", None)?.leading_space().expect("present"),
                expected,
                "{:?} should find {:?} as whitespace",
                config,
                expected
            )
        }
        Ok(())
    }

    #[test]
    fn push_splits_values_into_events() {
        let mut file = git_config::File::default();
        let mut section = file.new_section("core", None).unwrap();
        section.push(Key::try_from("value").unwrap(), Cow::Borrowed("none".into()));
        assert_eq!(file.to_bstring(), "[core]\n        value=none\n");
    }
}
