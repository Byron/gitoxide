mod ansi_c {
    mod undo {
        use bstr::ByteSlice;
        use git_quote::ansi_c;

        macro_rules! test {
            ($name:ident, $input:literal, $expected:literal) => {
                #[test]
                fn $name() {
                    assert_eq!(
                        ansi_c::undo($input.as_bytes().as_bstr()).expect("valid input"),
                        std::borrow::Cow::Borrowed($expected.as_bytes().as_bstr())
                    );
                }
            };
        }

        test!(unquoted_remains_unchanged, "hello", "hello");
        test!(empty_surrounded_by_quotes, "\"\"", "");
        test!(surrounded_only_by_quotes, "\"hello\"", "hello");
        test!(typical_escapes, r#""\n\r\t""#, b"\n\r\t");
        test!(untypical_escapes, r#""\a\b\f\v""#, b"\x07\x08\x0c\x0b");
        test!(literal_escape_and_double_quote, r#""\"\\""#, br#""\"#);
        test!(
            unicode_byte_escapes_by_number,
            r#""\346\277\261\351\207\216\t\347\264\224""#,
            "濱野\t純"
        );
        test!(
            exclamation_and_tilde_survive_an_escape_with_double_escaping,
            r#""\\!\\#hello there/file.ext""#,
            r"\!\#hello there/file.ext"
        );

        #[test]
        fn out_of_quote_characters_can_be_passed_and_will_not_be_consumed() {
            assert_eq!(
                ansi_c::undo(br#""hello there" out of quote"#.as_bstr()).expect("valid input"),
                std::borrow::Cow::Borrowed(b"hello there".as_bstr())
            );
        }
    }
}
