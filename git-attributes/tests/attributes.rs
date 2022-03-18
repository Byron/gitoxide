mod parse {
    mod ignore {
        use git_attributes::ignore::pattern::Mode;

        #[test]
        fn comments_are_ignored() {
            assert!(git_attributes::parse::ignore(b"# hello world").next().is_none());
        }

        #[test]
        fn backslashes_before_hashes_are_no_comments() {
            assert_eq!(
                git_attributes::parse::ignore(br"\#hello").next(),
                Some((r"#hello".into(), Mode::NO_SUB_DIR, 1))
            );
        }

        #[test]
        fn backslashes_are_part_of_the_pattern_if_not_in_specific_positions() {
            assert_eq!(
                git_attributes::parse::ignore(br"\hello\world").next(),
                Some((r"\hello\world".into(), Mode::NO_SUB_DIR, 1))
            );
        }

        #[test]
        fn leading_exclamation_mark_negates_pattern() {
            assert_eq!(
                git_attributes::parse::ignore(b"!hello").next(),
                Some(("hello".into(), Mode::NEGATIVE | Mode::NO_SUB_DIR, 1))
            );
        }

        #[test]
        fn leading_exclamation_marks_can_be_escaped_with_backslash() {
            assert_eq!(
                git_attributes::parse::ignore(br"\!hello").next(),
                Some(("!hello".into(), Mode::NO_SUB_DIR, 1))
            );
        }

        #[test]
        fn absence_of_sub_directories_are_marked() {
            assert_eq!(
                git_attributes::parse::ignore(br"a/b").next(),
                Some(("a/b".into(), Mode::empty(), 1))
            );
            assert_eq!(
                git_attributes::parse::ignore(br"ab").next(),
                Some(("ab".into(), Mode::NO_SUB_DIR, 1))
            );
        }

        #[test]
        fn trailing_slashes_are_marked_and_removed() {
            assert_eq!(
                git_attributes::parse::ignore(b"dir/").next(),
                Some(("dir".into(), Mode::MUST_BE_DIR | Mode::NO_SUB_DIR, 1))
            );
            assert_eq!(
                git_attributes::parse::ignore(b"dir///").next(),
                Some(("dir//".into(), Mode::MUST_BE_DIR, 1)),
                "but only the last slash is removed"
            );
        }

        #[test]
        fn trailing_spaces_are_ignored() {
            assert_eq!(
                git_attributes::parse::ignore(br"a   ").next(),
                Some(("a".into(), Mode::NO_SUB_DIR, 1))
            );
            assert_eq!(
                git_attributes::parse::ignore(b"a\t\t  ").next(),
                Some(("a\t\t".into(), Mode::NO_SUB_DIR, 1)),
                "trailing tabs are not ignored"
            );
        }
        #[test]
        fn trailing_spaces_can_be_escaped_to_be_literal() {
            assert_eq!(
                git_attributes::parse::ignore(br"a  \ ").next(),
                Some(("a   ".into(), Mode::NO_SUB_DIR, 1)),
                "a single escape in front of the last desired space is enough"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a  b  c ").next(),
                Some(("a  b  c".into(), Mode::NO_SUB_DIR, 1)),
                "spaces in the middle are fine"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a\ \ \ ").next(),
                Some(("a   ".into(), Mode::NO_SUB_DIR, 1)),
                "one can also escape every single one"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a \  ").next(),
                Some(("a  ".into(), Mode::NO_SUB_DIR, 1)),
                "or just the one in the middle, losing the last actual space"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \").next(),
                Some(("a   ".into(), Mode::NO_SUB_DIR, 1)),
                "escaping nothing also works as a whitespace protection"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \\\ ").next(),
                Some((r"a    ".into(), Mode::NO_SUB_DIR, 1)),
                "strange things like these work too"
            );
            assert_eq!(
                git_attributes::parse::ignore(br"a   \\ ").next(),
                Some((r"a   ".into(), Mode::NO_SUB_DIR, 1)),
                "strange things like these work as well"
            );
        }
    }
}
