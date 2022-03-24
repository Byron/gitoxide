mod ansi_c {
    mod undo {
        use bstr::ByteSlice;

        macro_rules! test {
            ($name:ident, $input:literal, $expected:literal) => {
                #[test]
                fn $name() {
                    assert_eq!(
                        git_quote::ansi_c::undo($input.as_bytes().as_bstr()).expect("valid input"),
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
    }
}
