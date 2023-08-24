use super::*;

mod section_headers {
    use winnow::prelude::*;

    use super::section_header;
    use crate::parse::tests::util::{fully_consumed, section_header as parsed_section_header};

    #[test]
    fn no_subsection() {
        assert_eq!(
            section_header.parse_peek(b"[hello]").unwrap(),
            fully_consumed(parsed_section_header("hello", None)),
        );
    }

    #[test]
    fn modern_subsection() {
        assert_eq!(
            section_header.parse_peek(br#"[hello "world"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "world"))),
        );
    }

    #[test]
    fn escaped_subsection() {
        assert_eq!(
            section_header.parse_peek(br#"[hello "foo\\bar\""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", r#"foo\bar""#))),
        );
    }

    #[test]
    fn deprecated_subsection() {
        assert_eq!(
            section_header.parse_peek(br#"[hello.world]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (".", "world")))
        );
        assert_eq!(
            section_header.parse_peek(br#"[Hello.World]"#).unwrap(),
            fully_consumed(parsed_section_header("Hello", (".", "World")))
        );
    }

    #[test]
    fn empty_legacy_subsection_name() {
        assert_eq!(
            section_header.parse_peek(br#"[hello-world.]"#).unwrap(),
            fully_consumed(parsed_section_header("hello-world", (".", "")))
        );
    }

    #[test]
    fn empty_modern_subsection_name() {
        assert_eq!(
            section_header.parse_peek(br#"[hello ""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "")))
        );
    }

    #[test]
    fn backslashes_in_subsections_do_not_escape_newlines_or_tabs() {
        assert_eq!(
            section_header.parse_peek(br#"[hello "single \ \\ \t \n \0"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", r"single  \ t n 0")))
        );
    }

    #[test]
    fn newline_in_header() {
        assert!(section_header.parse_peek(b"[hello\n]").is_err());
    }

    #[test]
    fn newline_in_sub_section() {
        assert!(section_header.parse_peek(b"[hello \"hello\n\"]").is_err());
    }

    #[test]
    fn null_byt_in_sub_section() {
        assert!(section_header.parse_peek(b"[hello \"hello\0\"]").is_err());
    }

    #[test]
    fn escaped_newline_in_sub_section() {
        assert!(section_header.parse_peek(b"[hello \"hello\\\n\"]").is_err());
    }

    #[test]
    fn eof_after_escape_in_sub_section() {
        assert!(section_header.parse_peek(b"[hello \"hello\\").is_err());
    }

    #[test]
    fn null_byte_in_header() {
        assert!(section_header.parse_peek(b"[hello\0]").is_err());
    }

    #[test]
    fn invalid_characters_in_section() {
        assert!(section_header.parse_peek(b"[$]").is_err());
    }
    #[test]
    fn invalid_characters_in_legacy_sub_section() {
        assert!(section_header.parse_peek(b"[hello.$]").is_err());
        assert!(section_header.parse_peek(b"[hello. world]").is_err());
    }

    #[test]
    fn right_brace_in_subsection_name() {
        assert_eq!(
            section_header.parse_peek(br#"[hello "]"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "]")))
        );
    }
}

mod sub_section {
    use std::borrow::Cow;

    use winnow::prelude::*;

    use super::sub_section;

    #[test]
    fn zero_copy_simple() {
        let actual = sub_section.parse_peek(b"name\"").unwrap().1;
        assert_eq!(actual.as_ref(), "name");
        assert!(matches!(actual, Cow::Borrowed(_)));
    }

    #[test]
    fn escapes_need_allocation() {
        let actual = sub_section.parse_peek(br#"\x\t\n\0\\\"""#).unwrap().1;
        assert_eq!(actual.as_ref(), r#"xtn0\""#);
        assert!(matches!(actual, Cow::Owned(_)));
    }
}

mod config_name {
    use winnow::prelude::*;

    use super::config_name;
    use crate::parse::tests::util::fully_consumed;

    #[test]
    fn just_name() {
        assert_eq!(config_name.parse_peek(b"name").unwrap(), fully_consumed("name".into()));
    }

    #[test]
    fn must_start_with_alphabetic() {
        assert!(config_name.parse_peek(b"4aaa").is_err());
        assert!(config_name.parse_peek(b"-aaa").is_err());
    }

    #[test]
    fn only_a_subset_of_characters_is_allowed() {
        assert!(config_name.parse(b"Name$_").is_err());
        assert!(config_name.parse(b"other#").is_err());
    }

    #[test]
    fn cannot_be_empty() {
        assert!(config_name.parse_peek(b"").is_err());
    }
}

mod section {
    use crate::parse::{
        error::ParseNode,
        section,
        tests::util::{
            comment_event, fully_consumed, name_event, newline_custom_event, newline_event,
            section_header as parsed_section_header, value_done_event, value_event, value_not_done_event,
            whitespace_event,
        },
        Event, Section,
    };

    fn section<'a>(mut i: &'a [u8], node: &mut ParseNode) -> winnow::IResult<&'a [u8], Section<'a>> {
        let mut header = None;
        let mut events = section::Events::default();
        super::section(&mut i, node, &mut |e| match &header {
            None => {
                header = Some(e);
            }
            Some(_) => events.push(e),
        })
        .map(|_| {
            (
                i,
                Section {
                    header: match header.expect("header set") {
                        Event::SectionHeader(header) => header,
                        _ => unreachable!("unexpected"),
                    },
                    events,
                },
            )
        })
    }

    #[test]
    fn empty_value_with_windows_newlines() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[a] k = \r\n", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("a", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("k"),
                    whitespace_event(" "),
                    Event::KeyValueSeparator,
                    whitespace_event(" "),
                    value_event(""),
                    newline_custom_event("\r\n")
                ]
                .into(),
            }),
        );
    }

    #[test]
    fn simple_value_with_windows_newlines() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[a] k = v\r\n", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("a", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("k"),
                    whitespace_event(" "),
                    Event::KeyValueSeparator,
                    whitespace_event(" "),
                    value_event("v"),
                    newline_custom_event("\r\n")
                ]
                .into(),
            }),
        );
        assert_eq!(
            section(b"[a] k = \r\n", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("a", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("k"),
                    whitespace_event(" "),
                    Event::KeyValueSeparator,
                    whitespace_event(" "),
                    value_event(""),
                    newline_custom_event("\r\n")
                ]
                .into(),
            }),
        );
    }

    #[test]
    fn empty_section() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[test]", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("test", None),
                events: Default::default()
            }),
        );
    }

    #[test]
    fn simple_section() {
        let mut node = ParseNode::SectionHeader;
        let section_data = br#"[hello]
            a = b
            c
            d = "lol""#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("hello", None),
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
                .into()
            })
        );
    }

    #[test]
    fn section_with_empty_value_simplified() {
        let mut node = ParseNode::SectionHeader;
        let section_data = b"[a] k=";
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("a", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("k"),
                    Event::KeyValueSeparator,
                    value_event(""),
                ]
                .into()
            })
        );

        let section_data = b"[a] k=\n";
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("a", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("k"),
                    Event::KeyValueSeparator,
                    value_event(""),
                    newline_event(),
                ]
                .into()
            })
        );
    }

    #[test]
    fn section_with_empty_value() {
        let mut node = ParseNode::SectionHeader;
        let section_data = br#"[hello]
            a = b
            c=
            d = "lol""#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("hello", None),
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
                .into()
            })
        );
    }

    #[test]
    fn section_implicit_value() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[hello] c", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("hello", None),
                events: vec![whitespace_event(" "), name_event("c"), value_event("")].into()
            })
        );

        assert_eq!(
            section(b"[hello] c\nd", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("hello", None),
                events: vec![
                    whitespace_event(" "),
                    name_event("c"),
                    value_event(""),
                    newline_event(),
                    name_event("d"),
                    value_event("")
                ]
                .into()
            })
        );
    }

    #[test]
    fn section_very_commented() {
        let mut node = ParseNode::SectionHeader;
        let section_data = br#"[hello] ; commentA
            a = b # commentB
            ; commentC
            ; commentD
            c = d"#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("hello", None),
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
                .into()
            })
        );
    }

    #[test]
    fn complex_continuation() {
        let mut node = ParseNode::SectionHeader;
        // This test is absolute hell. Good luck if this fails.
        assert_eq!(
            section(b"[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("section", None),
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
                .into()
            })
        );
    }

    #[test]
    fn quote_split_over_two_lines() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[section \"a\"] b =\"\\\n;\";a", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("section", (" ", "a")),
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
                .into()
            })
        );
    }

    #[test]
    fn section_handles_extraneous_whitespace_before_comment() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[s]hello             #world", &mut node).unwrap(),
            fully_consumed(Section {
                header: parsed_section_header("s", None),
                events: vec![
                    name_event("hello"),
                    whitespace_event("             "),
                    value_event(""),
                    comment_event('#', "world"),
                ]
                .into()
            })
        );
    }
}

mod value_continuation {
    use bstr::ByteSlice;

    use crate::parse::{
        section,
        tests::util::{into_events, newline_custom_event, newline_event, value_done_event, value_not_done_event},
    };

    pub fn value_impl<'a>(mut i: &'a [u8], events: &mut section::Events<'a>) -> winnow::IResult<&'a [u8], ()> {
        super::value_impl(&mut i, &mut |e| events.push(e)).map(|_| (i, ()))
    }

    #[test]
    fn simple_continuation() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello\\\nworld", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("world")
            ])
        );
    }

    #[test]
    fn continuation_with_whitespace() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello\\\n        world", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("        world")
            ])
        );

        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello\\\r\n        world", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("hello"),
                newline_custom_event("\r\n"),
                value_done_event("        world")
            ])
        );

        let mut events = section::Events::default();
        assert!(
            value_impl(b"hello\\\r\r\n        world", &mut events).is_err(),
            "\\r must be followed by \\n"
        );
    }

    #[test]
    fn complex_continuation_with_leftover_comment() {
        let mut events = section::Events::default();
        assert_eq!(
            value_impl(b"1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut events)
                .unwrap()
                .0,
            b" # \"b\t ; c"
        );
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event(r#"1    "\""#),
                newline_event(),
                value_not_done_event(r#"a ; e "\""#),
                newline_event(),
                value_done_event("d")
            ])
        );
    }

    #[test]
    fn quote_split_over_two_lines_with_leftover_comment() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"\"\\\n;\";a", &mut events).unwrap().0, b";a");
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("\""),
                newline_event(),
                value_done_event(";\"")
            ])
        );

        let mut events = section::Events::default();
        assert_eq!(value_impl(b"\"a\\\r\nb;\";c", &mut events).unwrap().0, b";c");
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("\"a"),
                newline_custom_event("\r\n"),
                value_done_event("b;\"")
            ])
        );
    }

    #[test]
    fn quote_split_over_multiple_lines_without_surrounding_quotes_but_inner_quotes() {
        let mut events = section::Events::default();
        assert_eq!(
            value_impl(
                br#"1\
"2" a\
\"3 b\"\
4 ; comment "#,
                &mut events
            )
            .unwrap()
            .0
            .as_bstr(),
            b" ; comment ".as_bstr()
        );
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("1"),
                newline_event(),
                value_not_done_event("\"2\" a"),
                newline_event(),
                value_not_done_event("\\\"3 b\\\""),
                newline_event(),
                value_done_event("4")
            ])
        );
    }

    #[test]
    fn quote_split_over_multiple_lines_with_surrounding_quotes() {
        let mut events = section::Events::default();
        assert_eq!(
            value_impl(
                br#""1\
"2" a\
\"3 b\"\
4 " ; comment "#,
                &mut events
            )
            .unwrap()
            .0
            .as_bstr(),
            b" ; comment ".as_bstr()
        );
        assert_eq!(
            events,
            into_events(vec![
                value_not_done_event("\"1"),
                newline_event(),
                value_not_done_event("\"2\" a"),
                newline_event(),
                value_not_done_event("\\\"3 b\\\""),
                newline_event(),
                value_done_event("4 \"")
            ])
        );
    }
}

mod value_no_continuation {
    use super::value_continuation::value_impl;
    use crate::parse::{
        section,
        tests::util::{into_events, value_event},
    };

    #[test]
    fn no_comment() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello", &mut events).unwrap().0, b"");
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    fn windows_newline() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hi\r\nrest", &mut events).unwrap().0, b"\r\nrest");
        assert_eq!(events, into_events(vec![value_event("hi")]));

        events.clear();
        assert_eq!(value_impl(b"hi\r\r\r\nrest", &mut events).unwrap().0, b"\r\r\r\nrest");
        assert_eq!(events, into_events(vec![value_event("hi")]));
    }

    #[test]
    fn no_comment_newline() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello\na", &mut events).unwrap().0, b"\na");
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    fn semicolon_comment_not_consumed() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello;world", &mut events).unwrap().0, b";world");
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    fn octothorpe_comment_not_consumed() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello#world", &mut events).unwrap().0, b"#world");
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    fn values_with_extraneous_whitespace_without_comment() {
        let mut events = section::Events::default();
        assert_eq!(
            value_impl(b"hello               ", &mut events).unwrap().0,
            b"               "
        );
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    fn values_with_extraneous_whitespace_before_comment() {
        let mut events = section::Events::default();
        assert_eq!(
            value_impl(b"hello             #world", &mut events).unwrap().0,
            b"             #world"
        );
        assert_eq!(events, into_events(vec![value_event("hello")]));

        let mut events = section::Events::default();
        assert_eq!(
            value_impl(b"hello             ;world", &mut events).unwrap().0,
            b"             ;world"
        );
        assert_eq!(events, into_events(vec![value_event("hello")]));
    }

    #[test]
    #[allow(clippy::needless_raw_string_hashes)]
    fn trans_escaped_comment_marker_not_consumed() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(br##"hello"#"world; a"##, &mut events).unwrap().0, b"; a");
        assert_eq!(events, into_events(vec![value_event(r##"hello"#"world"##)]));
    }

    #[test]
    fn complex_test() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(br#"value";";ahhhh"#, &mut events).unwrap().0, b";ahhhh");
        assert_eq!(events, into_events(vec![value_event(r#"value";""#)]));
    }

    #[test]
    fn garbage_after_continuation_is_err() {
        assert!(value_impl(b"hello \\afwjdls", &mut Default::default()).is_err());
    }

    #[test]
    fn invalid_escape() {
        assert!(value_impl(br"\x", &mut Default::default()).is_err());
    }

    #[test]
    fn incomplete_quote() {
        assert!(value_impl(br#"hello "world"#, &mut Default::default()).is_err());
    }

    #[test]
    fn incomplete_escape() {
        assert!(value_impl(br"hello world\", &mut Default::default()).is_err());
    }
}

mod key_value_pair {
    use crate::parse::{
        error::ParseNode,
        section,
        tests::util::{into_events, name_event, value_event, whitespace_event},
        Event,
    };

    fn key_value<'a>(
        mut i: &'a [u8],
        node: &mut ParseNode,
        events: &mut section::Events<'a>,
    ) -> winnow::IResult<&'a [u8], ()> {
        super::key_value_pair(&mut i, node, &mut |e| events.push(e)).map(|_| (i, ()))
    }

    #[test]
    fn nonascii_is_allowed_for_values_but_not_for_keys() {
        let mut node = ParseNode::SectionHeader;
        let mut vec = Default::default();
        assert!(
            key_value("你好".as_bytes(), &mut node, &mut vec).is_ok(),
            "Verifying `is_ok` because bad keys get ignored, the caller parser handles this as error"
        );
        assert_eq!(vec, into_events(vec![]));

        let mut node = ParseNode::SectionHeader;
        let mut vec = Default::default();
        assert!(key_value("a = 你好 ".as_bytes(), &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            into_events(vec![
                name_event("a"),
                whitespace_event(" "),
                Event::KeyValueSeparator,
                whitespace_event(" "),
                value_event("你好")
            ])
        );
    }

    #[test]
    fn whitespace_is_not_ambiguous() {
        let mut node = ParseNode::SectionHeader;
        let mut vec = Default::default();
        assert!(key_value(b"a =b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            into_events(vec![
                name_event("a"),
                whitespace_event(" "),
                Event::KeyValueSeparator,
                value_event("b")
            ])
        );

        let mut vec = Default::default();
        assert!(key_value(b"a= b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            into_events(vec![
                name_event("a"),
                Event::KeyValueSeparator,
                whitespace_event(" "),
                value_event("b")
            ])
        );
    }
}

mod comment {
    use winnow::prelude::*;

    use super::comment;
    use crate::parse::tests::util::{comment as parsed_comment, fully_consumed};

    #[test]
    fn semicolon() {
        assert_eq!(
            comment.parse_peek(b"; this is a semicolon comment").unwrap(),
            fully_consumed(parsed_comment(';', " this is a semicolon comment")),
        );
    }

    #[test]
    fn octothorpe() {
        assert_eq!(
            comment.parse_peek(b"# this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', " this is an octothorpe comment")),
        );
    }

    #[test]
    fn multiple_markers() {
        assert_eq!(
            comment.parse_peek(b"###### this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', "##### this is an octothorpe comment")),
        );
    }
}
