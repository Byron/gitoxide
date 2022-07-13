mod push {
    use crate::file::cow_str;
    use git_config::parse::section::Key;
    use std::convert::TryFrom;

    #[test]
    fn whitespace_is_derived_from_whitespace_before_first_value() -> crate::Result {
        for (input, expected_pre_key, expected_sep) in [
            ("[a]\n\t\tb=c", Some("\t\t".into()), (None, None)),
            ("[a]\nb= c", None, (None, Some(" "))),
            ("[a]", Some("\t".into()), (Some(" "), Some(" "))),
            ("[a]\t\tb =c", Some("\t\t".into()), (Some(" "), None)),
            (
                "[a]\n\t\t  \n    \t    b =  c",
                Some("    \t    ".into()),
                (Some(" "), Some("  ")),
            ),
        ] {
            let mut config: git_config::File = input.parse()?;
            let section = config.section_mut("a", None)?;
            assert_eq!(
                section.leading_whitespace(),
                expected_pre_key,
                "{:?} should find {:?} as leading whitespace",
                input,
                expected_pre_key
            );

            let (pre_sep, post_sep) = expected_sep;
            assert_eq!(
                section.separator_whitespace(),
                (pre_sep.map(|s| s.into()), post_sep.map(|s| s.into())),
                "{:?} should find {:?} as sep whitespace",
                input,
                expected_sep
            );
        }
        Ok(())
    }

    #[test]
    fn values_are_escaped() {
        for (value, expected) in [
            ("a b", "[a]\n\tk = a b"),
            (" a b", "[a]\n\tk = \" a b\""),
            ("a b\t", "[a]\n\tk = \"a b\\t\""),
            (";c", "[a]\n\tk = \";c\""),
            ("#c", "[a]\n\tk = \"#c\""),
            ("a\nb\n\tc", "[a]\n\tk = a\\nb\\n\\tc"),
        ] {
            let mut config = git_config::File::default();
            let mut section = config.new_section("a", None).unwrap();
            section.set_implicit_newline(false);
            section.push(Key::try_from("k").unwrap(), cow_str(value));
            assert_eq!(config.to_bstring(), expected);
        }
    }
}

mod set_leading_whitespace {
    use crate::file::cow_str;
    use git_config::parse::section::Key;
    use std::convert::TryFrom;

    #[test]
    fn any_whitespace_is_ok() -> crate::Result {
        let mut config = git_config::File::default();
        let mut section = config.new_section("core", None)?;
        section.set_leading_whitespace(cow_str("\n\t").into());
        section.push(Key::try_from("a")?, cow_str("v"));
        assert_eq!(config.to_string(), "[core]\n\n\ta = v\n");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn panics_if_non_whitespace_is_used() {
        let mut config = git_config::File::default();
        let mut section = config.new_section("core", None).unwrap();
        section.set_leading_whitespace(cow_str("foo").into());
    }
}
