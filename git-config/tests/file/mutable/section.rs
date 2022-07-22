mod remove {
    use super::multi_value_section;

    #[test]
    fn all() -> crate::Result {
        let mut config = multi_value_section();
        let mut section = config.section_mut("a", None)?;

        assert_eq!(section.num_values(), 5);
        assert_eq!(section.keys().count(), 5);

        let prev_values = vec!["v", "", "", "", "a        b        c"];
        let mut num_values = section.num_values();
        for (key, expected_prev_value) in ('a'..='e').zip(prev_values) {
            let prev_value = section.remove(key.to_string());
            num_values -= 1;
            assert_eq!(prev_value.expect("present").as_ref(), expected_prev_value);
            assert_eq!(section.num_values(), num_values);
        }

        assert!(!section.is_void(), "everything is still there");
        assert_eq!(
            config.to_string(),
            "\n        [a]\n            \n            \n            \n            \n            "
        );
        Ok(())
    }
}

mod pop {
    use super::multi_value_section;

    #[test]
    fn all() -> crate::Result {
        let mut config = multi_value_section();
        let mut section = config.section_mut("a", None)?;

        assert_eq!(section.num_values(), 5);
        assert_eq!(section.keys().count(), 5);

        for key in b'a'..=b'e' {
            assert!(section.contains_key(std::str::from_utf8(&[key])?));
        }
        let mut num_values = section.num_values();
        for _ in 0..section.num_values() {
            section.pop();
            num_values -= 1;
            assert_eq!(section.num_values(), num_values);
        }
        assert!(!section.is_void(), "there still is some whitespace");
        assert_eq!(config.to_string(), "\n        [a]\n");
        Ok(())
    }
}

mod set {
    use super::multi_value_section;
    use std::convert::TryInto;

    #[test]
    fn various_escapes_onto_various_kinds_of_values() -> crate::Result {
        let mut config = multi_value_section();
        let mut section = config.section_mut("a", None)?;
        let values = vec!["", " a", "b\t", "; comment", "a\n\tc  d\\ \"x\""];
        let prev_values = vec!["v", "", "", "", "a        b        c"];
        assert_eq!(section.num_values(), values.len());

        for (key, (new_value, expected_prev_value)) in (b'a'..=b'e').zip(values.into_iter().zip(prev_values)) {
            let key = std::str::from_utf8(std::slice::from_ref(&key))?.to_owned();
            let prev_value = section.set(key.try_into()?, new_value);
            assert_eq!(prev_value.as_deref().expect("prev value set"), expected_prev_value);
        }

        assert_eq!(config.to_string(), "\n        [a]\n            a = \n            b = \" a\"\n            c=\"b\\t\"\n            d\"; comment\"\n            e =a\\n\\tc  d\\\\ \\\"x\\\"\n");
        assert_eq!(
            config
                .section_mut("a", None)?
                .set("new-one".to_owned().try_into()?, "value"),
            None,
            "new values don't replace an existing one"
        );
        Ok(())
    }
}

mod push {
    use std::convert::TryFrom;

    use git_config::parse::section::Key;

    #[test]
    fn whitespace_is_derived_from_whitespace_before_first_value() -> crate::Result {
        for (input, expected_pre_key, expected_sep) in [
            ("[a]\n\t\tb=c", Some("\t\t".into()), (None, None)),
            ("[a]\nb= c", None, (None, Some(" "))),
            ("[a]", Some("\t".into()), (Some(" "), Some(" "))),
            ("[a] b", Some(" ".into()), (None, None)),
            ("[a]\tb = ", Some("\t".into()), (Some(" "), Some(" "))),
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
            ("a b", "$head\tk = a b$nl"),
            (" a b", "$head\tk = \" a b\"$nl"),
            ("a b\t", "$head\tk = \"a b\\t\"$nl"),
            (";c", "$head\tk = \";c\"$nl"),
            ("#c", "$head\tk = \"#c\"$nl"),
            ("a\nb\n\tc", "$head\tk = a\\nb\\n\\tc$nl"),
        ] {
            let mut config = git_config::File::default();
            let mut section = config.new_section("a", None).unwrap();
            section.set_implicit_newline(false);
            section.push(Key::try_from("k").unwrap(), value);
            let expected = expected
                .replace("$head", &format!("[a]{nl}", nl = section.newline()))
                .replace("$nl", &section.newline().to_string());
            assert_eq!(config.to_bstring(), expected);
        }
    }
}

mod set_leading_whitespace {
    use bstr::BString;
    use std::borrow::Cow;
    use std::convert::TryFrom;

    use git_config::parse::section::Key;

    use crate::file::cow_str;

    #[test]
    fn any_whitespace_is_ok() -> crate::Result {
        let mut config = git_config::File::default();
        let mut section = config.new_section("core", None)?;

        let nl = section.newline().to_owned();
        section.set_leading_whitespace(Some(Cow::Owned(BString::from(format!("{nl}\t")))));
        section.push(Key::try_from("a")?, "v");

        assert_eq!(config.to_string(), format!("[core]{nl}{nl}\ta = v{nl}"));
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

fn multi_value_section() -> git_config::File<'static> {
    r#"
        [a]
            a = v
            b = 
            c=
            d
            e =a \
       b \
       c"#
    .parse()
    .unwrap()
}
