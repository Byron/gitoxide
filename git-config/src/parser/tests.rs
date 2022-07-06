mod comments {
    use crate::parser::comment;
    use crate::parser::tests::util::{comment as parsed_comment, fully_consumed};

    #[test]
    fn semicolon() {
        assert_eq!(
            comment(b"; this is a semicolon comment").unwrap(),
            fully_consumed(parsed_comment(';', " this is a semicolon comment")),
        );
    }

    #[test]
    fn octothorpe() {
        assert_eq!(
            comment(b"# this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', " this is an octothorpe comment")),
        );
    }

    #[test]
    fn multiple_markers() {
        assert_eq!(
            comment(b"###### this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', "##### this is an octothorpe comment")),
        );
    }
}

mod section_headers {
    use crate::parser::section_header;
    use crate::parser::tests::util::{fully_consumed, section_header as parsed_section_header};

    #[test]
    fn no_subsection() {
        assert_eq!(
            section_header(b"[hello]").unwrap(),
            fully_consumed(parsed_section_header("hello", None)),
        );
    }

    #[test]
    fn modern_subsection() {
        assert_eq!(
            section_header(br#"[hello "world"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "world"))),
        );
    }

    #[test]
    fn escaped_subsection() {
        assert_eq!(
            section_header(br#"[hello "foo\\bar\""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", r#"foo\bar""#))),
        );
    }

    #[test]
    fn deprecated_subsection() {
        assert_eq!(
            section_header(br#"[hello.world]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (".", "world")))
        );
    }

    #[test]
    fn empty_legacy_subsection_name() {
        assert_eq!(
            section_header(br#"[hello.]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (".", "")))
        );
    }

    #[test]
    fn empty_modern_subsection_name() {
        assert_eq!(
            section_header(br#"[hello ""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "")))
        );
    }

    #[test]
    fn newline_in_header() {
        assert!(section_header(b"[hello\n]").is_err());
    }

    #[test]
    fn null_byte_in_header() {
        assert!(section_header(b"[hello\0]").is_err());
    }

    #[test]
    fn right_brace_in_subsection_name() {
        assert_eq!(
            section_header(br#"[hello "]"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "]")))
        );
    }
}

mod config_name {
    use crate::parser::config_name;
    use crate::parser::tests::util::fully_consumed;

    #[test]
    fn just_name() {
        assert_eq!(config_name(b"name").unwrap(), fully_consumed("name".into()));
    }

    #[test]
    fn must_start_with_alphabetic() {
        assert!(config_name(b"4aaa").is_err());
        assert!(config_name(b"-aaa").is_err());
    }

    #[test]
    fn cannot_be_empty() {
        assert!(config_name(b"").is_err());
    }
}

mod section_body {
    use crate::parser::tests::util::{name_event, value_event, whitespace_event};
    use crate::parser::{section_body, Event, ParserNode};

    #[test]
    fn whitespace_is_not_ambigious() {
        let mut node = ParserNode::SectionHeader;
        let mut vec = vec![];
        assert!(section_body(b"a =b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            vec![
                name_event("a"),
                whitespace_event(" "),
                Event::KeyValueSeparator,
                value_event("b")
            ]
        );

        let mut vec = vec![];
        assert!(section_body(b"a= b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            vec![
                name_event("a"),
                Event::KeyValueSeparator,
                whitespace_event(" "),
                value_event("b")
            ]
        );
    }
}

mod value_no_continuation {
    use crate::parser::tests::util::value_event;
    use crate::parser::value_impl;

    #[test]
    fn no_comment() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello", &mut events).unwrap().0, b"");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn no_comment_newline() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\na", &mut events).unwrap().0, b"\na");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn semicolon_comment_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello;world", &mut events).unwrap().0, b";world");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn octothorpe_comment_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello#world", &mut events).unwrap().0, b"#world");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn values_with_extraneous_whitespace_without_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello               ", &mut events).unwrap().0,
            b"               "
        );
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn values_with_extraneous_whitespace_before_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello             #world", &mut events).unwrap().0,
            b"             #world"
        );
        assert_eq!(events, vec![value_event("hello")]);

        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello             ;world", &mut events).unwrap().0,
            b"             ;world"
        );
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn trans_escaped_comment_marker_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(br##"hello"#"world; a"##, &mut events).unwrap().0, b"; a");
        assert_eq!(events, vec![value_event(r##"hello"#"world"##)]);
    }

    #[test]
    fn complex_test() {
        let mut events = vec![];
        assert_eq!(value_impl(br#"value";";ahhhh"#, &mut events).unwrap().0, b";ahhhh");
        assert_eq!(events, vec![value_event(r#"value";""#)]);
    }

    #[test]
    fn garbage_after_continution_is_err() {
        assert!(value_impl(b"hello \\afwjdls", &mut vec![]).is_err());
    }

    #[test]
    fn incomplete_quote() {
        assert!(value_impl(br#"hello "world"#, &mut vec![]).is_err());
    }

    #[test]
    fn incomplete_escape() {
        assert!(value_impl(br#"hello world\"#, &mut vec![]).is_err());
    }
}

mod value_continuation {
    use crate::parser::tests::util::{newline_event, value_done_event, value_not_done_event};
    use crate::parser::value_impl;

    #[test]
    fn simple_continuation() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\\\nworld", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("world")
            ]
        );
    }

    #[test]
    fn continuation_with_whitespace() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\\\n        world", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("        world")
            ]
        );
    }

    #[test]
    fn complex_continuation_with_leftover_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut events)
                .unwrap()
                .0,
            b" # \"b\t ; c"
        );
        assert_eq!(
            events,
            vec![
                value_not_done_event(r#"1    "\""#),
                newline_event(),
                value_not_done_event(r#"a ; e "\""#),
                newline_event(),
                value_done_event("d")
            ]
        );
    }

    #[test]
    fn quote_split_over_two_lines_with_leftover_comment() {
        let mut events = vec![];
        assert_eq!(value_impl(b"\"\\\n;\";a", &mut events).unwrap().0, b";a");
        assert_eq!(
            events,
            vec![value_not_done_event("\""), newline_event(), value_done_event(";\"")]
        );
    }
}

mod section {
    use crate::parser::tests::util::{
        comment_event, fully_consumed, name_event, newline_event, section_header as parsed_section_header,
        value_done_event, value_event, value_not_done_event, whitespace_event,
    };
    use crate::parser::{section, Event, ParsedSection, ParserNode};

    #[test]
    fn empty_section() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[test]", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("test", None),
                    events: vec![]
                },
                0
            )),
        );
    }

    #[test]
    fn simple_section() {
        let mut node = ParserNode::SectionHeader;
        let section_data = br#"[hello]
            a = b
            c
            d = "lol""#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![
                        newline_event(),
                        whitespace_event("            "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("b"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("c"),
                        value_event(""),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("d"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("\"lol\"")
                    ]
                },
                3
            ))
        );
    }

    #[test]
    fn section_with_empty_value() {
        let mut node = ParserNode::SectionHeader;
        let section_data = br#"[hello]
            a = b
            c=
            d = "lol""#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![
                        newline_event(),
                        whitespace_event("            "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("b"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("c"),
                        Event::KeyValueSeparator,
                        value_event(""),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("d"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("\"lol\"")
                    ]
                },
                2
            ))
        );
    }

    #[test]
    fn section_single_line() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[hello] c", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![whitespace_event(" "), name_event("c"), value_event("")]
                },
                0
            ))
        );
    }

    #[test]
    fn section_very_commented() {
        let mut node = ParserNode::SectionHeader;
        let section_data = br#"[hello] ; commentA
            a = b # commentB
            ; commentC
            ; commentD
            c = d"#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![
                        whitespace_event(" "),
                        comment_event(';', " commentA"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("b"),
                        whitespace_event(" "),
                        comment_event('#', " commentB"),
                        newline_event(),
                        whitespace_event("            "),
                        comment_event(';', " commentC"),
                        newline_event(),
                        whitespace_event("            "),
                        comment_event(';', " commentD"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("c"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("d"),
                    ]
                },
                4
            ))
        );
    }

    #[test]
    fn complex_continuation() {
        let mut node = ParserNode::SectionHeader;
        // This test is absolute hell. Good luck if this fails.
        assert_eq!(
            section(b"[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("section", None),
                    events: vec![
                        whitespace_event(" "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_not_done_event(r#"1    "\""#),
                        newline_event(),
                        value_not_done_event(r#"a ; e "\""#),
                        newline_event(),
                        value_done_event("d"),
                        whitespace_event(" "),
                        comment_event('#', " \"b\t ; c"),
                    ]
                },
                0
            ))
        );
    }

    #[test]
    fn quote_split_over_two_lines() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[section \"a\"] b =\"\\\n;\";a", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("section", (" ", "a")),
                    events: vec![
                        whitespace_event(" "),
                        name_event("b"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        value_not_done_event("\""),
                        newline_event(),
                        value_done_event(";\""),
                        comment_event(';', "a"),
                    ]
                },
                0
            ))
        );
    }

    #[test]
    fn section_handles_extranous_whitespace_before_comment() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[s]hello             #world", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("s", None),
                    events: vec![
                        name_event("hello"),
                        whitespace_event("             "),
                        value_event(""),
                        comment_event('#', "world"),
                    ]
                },
                0
            ))
        );
    }
}

mod parse {
    use crate::parser::{parse_from_bytes, parse_from_bytes_owned};

    #[test]
    fn parser_skips_bom() {
        let bytes = b"
        [core]
            a = 1
    ";
        let bytes_with_gb18030_bom = "\u{feff}
        [core]
            a = 1
    ";

        assert_eq!(
            parse_from_bytes(bytes),
            parse_from_bytes(bytes_with_gb18030_bom.as_bytes())
        );
        assert_eq!(
            parse_from_bytes_owned(bytes),
            parse_from_bytes_owned(bytes_with_gb18030_bom.as_bytes())
        );
    }
}

#[cfg(test)]
mod error {
    use crate::parser::parse_from_str;

    #[test]
    fn line_no_is_one_indexed() {
        assert_eq!(parse_from_str("[hello").unwrap_err().line_number(), 1);
    }

    #[test]
    fn remaining_data_contains_bad_tokens() {
        assert_eq!(parse_from_str("[hello").unwrap_err().remaining_data(), b"[hello");
    }

    #[test]
    fn to_string_truncates_extra_values() {
        assert_eq!(
            parse_from_str("[1234567890").unwrap_err().to_string(),
            "Got an unexpected token on line 1 while trying to parse a section header: '[123456789' ... (1 characters omitted)"
        );
    }
}

pub(crate) mod util {
    //! This module is only included for tests, and contains common unit test helper
    //! functions.

    use std::borrow::Cow;

    use crate::parser::{Event, Key, ParsedComment, ParsedSectionHeader};

    pub fn section_header(
        name: &str,
        subsection: impl Into<Option<(&'static str, &'static str)>>,
    ) -> ParsedSectionHeader<'_> {
        let name = name.into();
        if let Some((separator, subsection_name)) = subsection.into() {
            ParsedSectionHeader {
                name,
                separator: Some(Cow::Borrowed(separator.into())),
                subsection_name: Some(Cow::Borrowed(subsection_name.into())),
            }
        } else {
            ParsedSectionHeader {
                name,
                separator: None,
                subsection_name: None,
            }
        }
    }

    pub(crate) fn name_event(name: &'static str) -> Event<'static> {
        Event::Key(Key(Cow::Borrowed(name.into())))
    }

    pub(crate) fn value_event(value: &'static str) -> Event<'static> {
        Event::Value(Cow::Borrowed(value.into()))
    }

    pub(crate) fn value_not_done_event(value: &'static str) -> Event<'static> {
        Event::ValueNotDone(Cow::Borrowed(value.into()))
    }

    pub(crate) fn value_done_event(value: &'static str) -> Event<'static> {
        Event::ValueDone(Cow::Borrowed(value.into()))
    }

    pub(crate) fn newline_event() -> Event<'static> {
        newline_custom_event("\n")
    }

    pub(crate) fn newline_custom_event(value: &'static str) -> Event<'static> {
        Event::Newline(Cow::Borrowed(value.into()))
    }

    pub(crate) fn whitespace_event(value: &'static str) -> Event<'static> {
        Event::Whitespace(Cow::Borrowed(value.into()))
    }

    pub(crate) fn comment_event(tag: char, msg: &'static str) -> Event<'static> {
        Event::Comment(comment(tag, msg))
    }

    pub(crate) fn comment(comment_tag: char, comment: &'static str) -> ParsedComment<'static> {
        ParsedComment {
            comment_tag: comment_tag as u8,
            comment: Cow::Borrowed(comment.into()),
        }
    }

    pub(crate) const fn fully_consumed<T>(t: T) -> (&'static [u8], T) {
        (&[], t)
    }
}
