mod parse {
    mod ignore {
        use git_attributes::ignore;

        #[test]
        fn comments_are_ignored() {
            assert!(git_attributes::parse::ignore(b"# hello world").next().is_none());
        }

        #[test]
        fn backslashes_before_hashes_are_no_comments() {
            assert_eq!(
                git_attributes::parse::ignore(br"\#hello").next(),
                Some((r"#hello".into(), ignore::pattern::Mode::empty()))
            );
        }

        #[test]
        fn backslashes_are_part_of_the_pattern_if_not_in_specific_positions() {
            assert_eq!(
                git_attributes::parse::ignore(br"\hello\world").next(),
                Some((r"\hello\world".into(), ignore::pattern::Mode::empty()))
            );
        }

        #[test]
        fn leading_exclamation_mark_negates_pattern() {
            assert_eq!(
                git_attributes::parse::ignore(b"!hello").next(),
                Some(("hello".into(), ignore::pattern::Mode::NEGATIVE))
            );
        }

        #[test]
        fn leading_exclamation_marks_can_be_escaped_with_backslash() {
            assert_eq!(
                git_attributes::parse::ignore(br"\!hello").next(),
                Some(("!hello".into(), ignore::pattern::Mode::empty()))
            );
        }

        #[test]
        fn trailing_spaces_are_ignored() {
            assert_eq!(
                git_attributes::parse::ignore(br"a   ").next(),
                Some(("a".into(), ignore::pattern::Mode::empty()))
            );
            assert_eq!(
                git_attributes::parse::ignore(b"a\t\t  ").next(),
                Some(("a\t\t".into(), ignore::pattern::Mode::empty()),),
                "trailing tabs are not ignored"
            );
        }
        #[test]
        fn trailing_spaces_can_be_escaped_to_be_literal() {
            assert_eq!(
                git_attributes::parse::ignore(br"a  \ ").next(),
                Some(("a   ".into(), ignore::pattern::Mode::empty())),
                "a single escape in front of the last desired space is enough"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a  b  c ").next(),
                Some(("a  b  c".into(), ignore::pattern::Mode::empty())),
                "spaces in the middle are fine"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a\ \ \ ").next(),
                Some(("a   ".into(), ignore::pattern::Mode::empty())),
                "one can also escape every single one"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a \  ").next(),
                Some(("a  ".into(), ignore::pattern::Mode::empty())),
                "or just the one in the middle, losing the last actual space"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \").next(),
                Some(("a   ".into(), ignore::pattern::Mode::empty())),
                "escaping nothing also works as a whitespace protection"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \\\ ").next(),
                Some((r"a    ".into(), ignore::pattern::Mode::empty())),
                "strange things like these work too"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \\ ").next(),
                Some((r"a   ".into(), ignore::pattern::Mode::empty())),
                "strange things like these work as well"
            );
        }
    }
}
