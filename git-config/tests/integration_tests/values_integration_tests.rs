/// Converts string to byte slice
#[cfg(test)]
fn b(s: &str) -> &[u8] {
    s.as_bytes()
}

#[cfg(test)]
mod normalize {
    use git_config::values::normalize_str;
    use std::borrow::Cow;

    #[test]
    fn not_modified_is_borrowed() {
        assert_eq!(normalize_str("hello world"), Cow::Borrowed(b"hello world"));
    }

    #[test]
    fn modified_is_owned() {
        assert_eq!(
            normalize_str("hello \"world\""),
            Cow::<[u8]>::Owned(b"hello world".to_vec())
        );
    }

    #[test]
    fn all_quoted_is_optimized() {
        assert_eq!(normalize_str("\"hello world\""), Cow::Borrowed(b"hello world"));
    }

    #[test]
    fn all_quote_optimization_is_correct() {
        assert_eq!(normalize_str(r#""hello" world\""#), Cow::Borrowed(b"hello world\""));
    }

    #[test]
    fn quotes_right_next_to_each_other() {
        assert_eq!(
            normalize_str("\"hello\"\" world\""),
            Cow::<[u8]>::Owned(b"hello world".to_vec())
        );
    }

    #[test]
    fn escaped_quotes_are_kept() {
        assert_eq!(
            normalize_str(r#""hello \"\" world""#),
            Cow::<[u8]>::Owned(b"hello \"\" world".to_vec())
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(normalize_str(""), Cow::Borrowed(b""));
    }

    #[test]
    fn empty_normalized_string_is_optimized() {
        assert_eq!(normalize_str("\"\""), Cow::Borrowed(b""));
    }
}

#[cfg(test)]
mod boolean {
    use crate::values_integration_tests::b;
    use git_config::values::{Boolean, TrueVariant};
    use std::convert::TryFrom;

    #[test]
    fn from_str_false() {
        assert_eq!(Boolean::try_from(b("no")), Ok(Boolean::False("no".into())));
        assert_eq!(Boolean::try_from(b("off")), Ok(Boolean::False("off".into())));
        assert_eq!(Boolean::try_from(b("false")), Ok(Boolean::False("false".into())));
        assert_eq!(Boolean::try_from(b("zero")), Ok(Boolean::False("zero".into())));
        assert_eq!(Boolean::try_from(b("\"\"")), Ok(Boolean::False("\"\"".into())));
    }

    #[test]
    fn from_str_true() {
        assert_eq!(
            Boolean::try_from(b("yes")),
            Ok(Boolean::True(TrueVariant::Explicit("yes".into())))
        );
        assert_eq!(
            Boolean::try_from(b("on")),
            Ok(Boolean::True(TrueVariant::Explicit("on".into())))
        );
        assert_eq!(
            Boolean::try_from(b("true")),
            Ok(Boolean::True(TrueVariant::Explicit("true".into())))
        );
        assert_eq!(
            Boolean::try_from(b("one")),
            Ok(Boolean::True(TrueVariant::Explicit("one".into())))
        );
    }

    #[test]
    fn ignores_case() {
        // Random subset
        for word in &["no", "yes", "off", "true", "zero"] {
            let first: bool = Boolean::try_from(b(word)).unwrap().into();
            let second: bool = Boolean::try_from(b(&*word.to_uppercase())).unwrap().into();
            assert_eq!(first, second);
        }
    }

    #[test]
    fn from_str_err() {
        assert!(Boolean::try_from(b("yesn't")).is_err());
        assert!(Boolean::try_from(b("yesno")).is_err());
    }
}

#[cfg(test)]
mod integer {
    use crate::values_integration_tests::b;
    use git_config::values::{Integer, IntegerSuffix};
    use std::convert::TryFrom;

    #[test]
    fn from_str_no_suffix() {
        assert_eq!(Integer::try_from(b("1")).unwrap(), Integer { value: 1, suffix: None });

        assert_eq!(
            Integer::try_from(b("-1")).unwrap(),
            Integer {
                value: -1,
                suffix: None
            }
        );
    }

    #[test]
    fn from_str_with_suffix() {
        assert_eq!(
            Integer::try_from(b("1k")).unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Kibi),
            }
        );

        assert_eq!(
            Integer::try_from(b("1m")).unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Mebi),
            }
        );

        assert_eq!(
            Integer::try_from(b("1g")).unwrap(),
            Integer {
                value: 1,
                suffix: Some(IntegerSuffix::Gibi),
            }
        );
    }

    #[test]
    fn invalid_from_str() {
        assert!(Integer::try_from(b("")).is_err());
        assert!(Integer::try_from(b("-")).is_err());
        assert!(Integer::try_from(b("k")).is_err());
        assert!(Integer::try_from(b("m")).is_err());
        assert!(Integer::try_from(b("g")).is_err());
        assert!(Integer::try_from(b("123123123123123123123123")).is_err());
        assert!(Integer::try_from(b("gg")).is_err());
    }

    #[test]
    fn as_decimal() {
        assert_eq!(
            Integer::try_from(b("12")).unwrap().as_decimal().unwrap(),
            12,
            "works without suffix"
        );
        assert_eq!(
            Integer::try_from(b("13k")).unwrap().as_decimal().unwrap(),
            13 * 1024,
            "works with kilobyte suffix"
        );
        assert_eq!(
            Integer::try_from(b("14m")).unwrap().as_decimal().unwrap(),
            14 * 1048576,
            "works with megabyte suffix"
        );
        assert_eq!(
            Integer::try_from(b("15g")).unwrap().as_decimal().unwrap(),
            15 * 1073741824,
            "works with gigabyte suffix"
        );

        let max_i64 = format!("{}g", i64::MAX);
        assert!(
            Integer::try_from(b(&max_i64)).unwrap().as_decimal().is_none(),
            "overflow results in None"
        );
    }
}

#[cfg(test)]
mod color_value {
    use git_config::values::ColorValue;
    use std::str::FromStr;

    #[test]
    fn non_bright() {
        assert_eq!(ColorValue::from_str("normal"), Ok(ColorValue::Normal));
        assert_eq!(ColorValue::from_str("black"), Ok(ColorValue::Black));
        assert_eq!(ColorValue::from_str("red"), Ok(ColorValue::Red));
        assert_eq!(ColorValue::from_str("green"), Ok(ColorValue::Green));
        assert_eq!(ColorValue::from_str("yellow"), Ok(ColorValue::Yellow));
        assert_eq!(ColorValue::from_str("blue"), Ok(ColorValue::Blue));
        assert_eq!(ColorValue::from_str("magenta"), Ok(ColorValue::Magenta));
        assert_eq!(ColorValue::from_str("cyan"), Ok(ColorValue::Cyan));
        assert_eq!(ColorValue::from_str("white"), Ok(ColorValue::White));
    }

    #[test]
    fn bright() {
        assert_eq!(ColorValue::from_str("brightblack"), Ok(ColorValue::BrightBlack));
        assert_eq!(ColorValue::from_str("brightred"), Ok(ColorValue::BrightRed));
        assert_eq!(ColorValue::from_str("brightgreen"), Ok(ColorValue::BrightGreen));
        assert_eq!(ColorValue::from_str("brightyellow"), Ok(ColorValue::BrightYellow));
        assert_eq!(ColorValue::from_str("brightblue"), Ok(ColorValue::BrightBlue));
        assert_eq!(ColorValue::from_str("brightmagenta"), Ok(ColorValue::BrightMagenta));
        assert_eq!(ColorValue::from_str("brightcyan"), Ok(ColorValue::BrightCyan));
        assert_eq!(ColorValue::from_str("brightwhite"), Ok(ColorValue::BrightWhite));
    }

    #[test]
    fn ansi() {
        assert_eq!(ColorValue::from_str("255"), Ok(ColorValue::Ansi(255)));
        assert_eq!(ColorValue::from_str("0"), Ok(ColorValue::Ansi(0)));
    }

    #[test]
    fn hex() {
        assert_eq!(ColorValue::from_str("#ff0010"), Ok(ColorValue::Rgb(255, 0, 16)));
        assert_eq!(ColorValue::from_str("#ffffff"), Ok(ColorValue::Rgb(255, 255, 255)));
        assert_eq!(ColorValue::from_str("#000000"), Ok(ColorValue::Rgb(0, 0, 0)));
    }

    #[test]
    fn invalid() {
        assert!(ColorValue::from_str("brightnormal").is_err());
        assert!(ColorValue::from_str("").is_err());
        assert!(ColorValue::from_str("bright").is_err());
        assert!(ColorValue::from_str("256").is_err());
        assert!(ColorValue::from_str("#").is_err());
        assert!(ColorValue::from_str("#fff").is_err());
        assert!(ColorValue::from_str("#gggggg").is_err());
    }
}

#[cfg(test)]
mod color_attribute {
    use git_config::values::ColorAttribute;
    use std::str::FromStr;

    #[test]
    fn non_inverted() {
        assert_eq!(ColorAttribute::from_str("bold"), Ok(ColorAttribute::Bold));
        assert_eq!(ColorAttribute::from_str("dim"), Ok(ColorAttribute::Dim));
        assert_eq!(ColorAttribute::from_str("ul"), Ok(ColorAttribute::Ul));
        assert_eq!(ColorAttribute::from_str("blink"), Ok(ColorAttribute::Blink));
        assert_eq!(ColorAttribute::from_str("reverse"), Ok(ColorAttribute::Reverse));
        assert_eq!(ColorAttribute::from_str("italic"), Ok(ColorAttribute::Italic));
        assert_eq!(ColorAttribute::from_str("strike"), Ok(ColorAttribute::Strike));
    }

    #[test]
    fn inverted_no_dash() {
        assert_eq!(ColorAttribute::from_str("nobold"), Ok(ColorAttribute::NoBold));
        assert_eq!(ColorAttribute::from_str("nodim"), Ok(ColorAttribute::NoDim));
        assert_eq!(ColorAttribute::from_str("noul"), Ok(ColorAttribute::NoUl));
        assert_eq!(ColorAttribute::from_str("noblink"), Ok(ColorAttribute::NoBlink));
        assert_eq!(ColorAttribute::from_str("noreverse"), Ok(ColorAttribute::NoReverse));
        assert_eq!(ColorAttribute::from_str("noitalic"), Ok(ColorAttribute::NoItalic));
        assert_eq!(ColorAttribute::from_str("nostrike"), Ok(ColorAttribute::NoStrike));
    }

    #[test]
    fn inverted_dashed() {
        assert_eq!(ColorAttribute::from_str("no-bold"), Ok(ColorAttribute::NoBold));
        assert_eq!(ColorAttribute::from_str("no-dim"), Ok(ColorAttribute::NoDim));
        assert_eq!(ColorAttribute::from_str("no-ul"), Ok(ColorAttribute::NoUl));
        assert_eq!(ColorAttribute::from_str("no-blink"), Ok(ColorAttribute::NoBlink));
        assert_eq!(ColorAttribute::from_str("no-reverse"), Ok(ColorAttribute::NoReverse));
        assert_eq!(ColorAttribute::from_str("no-italic"), Ok(ColorAttribute::NoItalic));
        assert_eq!(ColorAttribute::from_str("no-strike"), Ok(ColorAttribute::NoStrike));
    }

    #[test]
    fn invalid() {
        assert!(ColorAttribute::from_str("a").is_err());
        assert!(ColorAttribute::from_str("no bold").is_err());
        assert!(ColorAttribute::from_str("").is_err());
        assert!(ColorAttribute::from_str("no").is_err());
        assert!(ColorAttribute::from_str("no-").is_err());
    }
}
#[cfg(test)]
mod interpolate_tests {
    use crate::values_integration_tests::b;
    use git_config::values::path::interpolate;
    use git_config::values::Path;
    use std::borrow::Cow;

    #[test]
    fn no_interpolation_for_paths_without_tilde_or_prefix() {
        let path = &b"/foo/bar"[..];
        let actual = Path::from(Cow::Borrowed(path));
        assert_eq!(&*actual, path);
        assert!(
            matches!(&actual.value, Cow::Borrowed(_)),
            "it does not unnecessarily copy values"
        );
    }

    #[test]
    fn empty_path_is_error() {
        assert!(matches!(
            Path::from(Cow::Borrowed(b(""))).interpolate(None),
            Err(interpolate::Error::Missing { what: "path" })
        ));
    }

    #[test]
    fn prefix_substitutes_git_install_dir() {
        for git_install_dir in &["/tmp/git", "C:\\git"] {
            for (val, expected) in &[
                (&b"%(prefix)/foo/bar"[..], "foo/bar"),
                (b"%(prefix)/foo\\bar", "foo\\bar"),
            ] {
                let expected =
                    &std::path::PathBuf::from(format!("{}{}{}", git_install_dir, std::path::MAIN_SEPARATOR, expected));
                assert_eq!(
                    &*Path::from(Cow::Borrowed(*val))
                        .interpolate(Some(std::path::Path::new(git_install_dir)))
                        .unwrap(),
                    expected,
                    "prefix interpolation keeps separators as they are"
                );
            }
        }
    }

    #[test]
    fn prefix_substitution_skipped_with_dot_slash() {
        let path = "./%(prefix)/foo/bar";
        let git_install_dir = "/tmp/git";
        assert_eq!(
            Path::from(Cow::Borrowed(b(path)))
                .interpolate(Some(std::path::Path::new(git_install_dir)))
                .unwrap(),
            std::path::Path::new(path)
        );
    }

    #[test]
    fn tilde_substitutes_current_user() {
        let path = &b"~/foo/bar"[..];
        let expected = format!(
            "{}{}foo/bar",
            dirs::home_dir().expect("empty home").display(),
            std::path::MAIN_SEPARATOR
        );
        assert_eq!(
            Path::from(Cow::Borrowed(path)).interpolate(None).unwrap().as_ref(),
            std::path::Path::new(&expected),
            "note that path separators are not turned into slashes as we work with `std::path::Path`"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn tilde_with_given_user_is_unsupported_on_windows() {
        assert!(matches!(
            Path::from(Cow::Borrowed(&b"~baz/foo/bar"[..])).interpolate(None),
            Err(interpolate::Error::UserInterpolationUnsupported)
        ));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn tilde_with_given_user() {
        let user = std::env::var("USER").unwrap();
        let home = std::env::var("HOME").unwrap();
        let specific_user_home = format!("~{}", user);

        for path_suffix in &["foo/bar", "foo\\bar", ""] {
            let path = format!("{}{}{}", specific_user_home, std::path::MAIN_SEPARATOR, path_suffix);
            let expected = format!("{}{}{}", home, std::path::MAIN_SEPARATOR, path_suffix);
            assert_eq!(
                Path::from(Cow::Borrowed(b(&path))).interpolate(None).unwrap(),
                std::path::Path::new(&expected),
                "it keeps path separators as is"
            );
        }
    }
}
