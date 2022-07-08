use super::*;

mod section_headers {
    use super::section_header;
    use crate::parse::tests::util::{fully_consumed, section_header as parsed_section_header};

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
        assert_eq!(
            section_header(br#"[Hello.World]"#).unwrap(),
            fully_consumed(parsed_section_header("Hello", (".", "World")))
        );
    }

    #[test]
    fn empty_legacy_subsection_name() {
        assert_eq!(
            section_header(br#"[hello-world.]"#).unwrap(),
            fully_consumed(parsed_section_header("hello-world", (".", "")))
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
    fn backslashes_in_subsections_do_not_escape_newlines_or_tabs() {
        assert_eq!(
            section_header(br#"[hello "single \ \\ \t \n \0"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", r#"single  \ t n 0"#)))
        );
    }

    #[test]
    fn newline_in_header() {
        assert!(section_header(b"[hello\n]").is_err());
    }

    #[test]
    fn newline_in_sub_section() {
        assert!(section_header(b"[hello \"hello\n\"]").is_err());
    }

    #[test]
    fn null_byt_in_sub_section() {
        assert!(section_header(b"[hello \"hello\0\"]").is_err());
    }

    #[test]
    fn escaped_newline_in_sub_section() {
        assert!(section_header(b"[hello \"hello\\\n\"]").is_err());
    }

    #[test]
    fn eof_after_escape_in_sub_section() {
        assert!(section_header(b"[hello \"hello\\").is_err());
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
    use super::config_name;
    use crate::parse::tests::util::fully_consumed;

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

mod section {
    use crate::parse::tests::util::{
        comment_event, fully_consumed, name_event, newline_event, section_header as parsed_section_header,
        value_done_event, value_event, value_not_done_event, whitespace_event,
    };
    use crate::parse::{error::ParseNode, section, Event, Section};

    fn section<'a>(i: &'a [u8], node: &mut ParseNode) -> nom::IResult<&'a [u8], (Section<'a>, usize)> {
        let mut header = None;
        let mut events = section::Events::default();
        super::section(i, node, &mut |e| match &header {
            None => {
                header = Some(e);
            }
            Some(_) => events.push(e),
        })
        .map(|(i, o)| {
            (
                i,
                (
                    Section {
                        section_header: match header.expect("header set") {
                            Event::SectionHeader(header) => header,
                            _ => unreachable!("unexpected"),
                        },
                        events,
                    },
                    o,
                ),
            )
        })
    }

    #[test]
    fn empty_section() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[test]", &mut node).unwrap(),
            fully_consumed((
                Section {
                    section_header: parsed_section_header("test", None),
                    events: Default::default()
                },
                0
            )),
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
            fully_consumed((
                Section {
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
                    .into()
                },
                3
            ))
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
            fully_consumed((
                Section {
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
                    .into()
                },
                2
            ))
        );
    }

    #[test]
    fn section_single_line() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[hello] c", &mut node).unwrap(),
            fully_consumed((
                Section {
                    section_header: parsed_section_header("hello", None),
                    events: vec![whitespace_event(" "), name_event("c"), value_event("")].into()
                },
                0
            ))
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
            fully_consumed((
                Section {
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
                    .into()
                },
                4
            ))
        );
    }

    #[test]
    fn complex_continuation() {
        let mut node = ParseNode::SectionHeader;
        // This test is absolute hell. Good luck if this fails.
        assert_eq!(
            section(b"[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut node).unwrap(),
            fully_consumed((
                Section {
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
                    .into()
                },
                0
            ))
        );
    }

    #[test]
    fn quote_split_over_two_lines() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[section \"a\"] b =\"\\\n;\";a", &mut node).unwrap(),
            fully_consumed((
                Section {
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
                    .into()
                },
                0
            ))
        );
    }

    #[test]
    fn section_handles_extranous_whitespace_before_comment() {
        let mut node = ParseNode::SectionHeader;
        assert_eq!(
            section(b"[s]hello             #world", &mut node).unwrap(),
            fully_consumed((
                Section {
                    section_header: parsed_section_header("s", None),
                    events: vec![
                        name_event("hello"),
                        whitespace_event("             "),
                        value_event(""),
                        comment_event('#', "world"),
                    ]
                    .into()
                },
                0
            ))
        );
    }
}

mod value_continuation {
    use crate::parse::section;
    use crate::parse::tests::util::{into_events, newline_event, value_done_event, value_not_done_event};

    pub fn value_impl<'a>(i: &'a [u8], events: &mut section::Events<'a>) -> nom::IResult<&'a [u8], ()> {
        super::value_impl(i, &mut |e| events.push(e))
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
    }
}

mod value_no_continuation {
    use crate::parse::section;
    use crate::parse::tests::util::{into_events, value_event};

    use super::value_continuation::value_impl;

    #[test]
    fn no_comment() {
        let mut events = section::Events::default();
        assert_eq!(value_impl(b"hello", &mut events).unwrap().0, b"");
        assert_eq!(events, into_events(vec![value_event("hello")]));
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
    fn garbage_after_continution_is_err() {
        assert!(value_impl(b"hello \\afwjdls", &mut Default::default()).is_err());
    }

    #[test]
    fn incomplete_quote() {
        assert!(value_impl(br#"hello "world"#, &mut Default::default()).is_err());
    }

    #[test]
    fn incomplete_escape() {
        assert!(value_impl(br#"hello world\"#, &mut Default::default()).is_err());
    }
}

mod section_body {
    use crate::parse::tests::util::{into_events, name_event, value_event, whitespace_event};
    use crate::parse::{error::ParseNode, section, Event};

    fn section_body<'a>(
        i: &'a [u8],
        node: &mut ParseNode,
        events: &mut section::Events<'a>,
    ) -> nom::IResult<&'a [u8], ()> {
        super::section_body(i, node, &mut |e| events.push(e))
    }

    #[test]
    fn whitespace_is_not_ambigious() {
        let mut node = ParseNode::SectionHeader;
        let mut vec = Default::default();
        assert!(section_body(b"a =b", &mut node, &mut vec).is_ok());
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
        assert!(section_body(b"a= b", &mut node, &mut vec).is_ok());
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
    use super::comment;
    use crate::parse::tests::util::{comment as parsed_comment, fully_consumed};

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
