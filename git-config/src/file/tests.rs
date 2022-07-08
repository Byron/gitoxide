mod try_from {
    use crate::file::{SectionBodyId, SectionBodyIds};
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::{
        file::SectionBody,
        parse::{
            section,
            tests::util::{name_event, newline_event, section_header, value_event},
            Event,
        },
        File,
    };

    #[test]
    fn empty() {
        let config = File::try_from("").unwrap();
        assert!(config.section_headers.is_empty());
        assert_eq!(config.section_id_counter, 0);
        assert!(config.section_lookup_tree.is_empty());
        assert!(config.sections.is_empty());
        assert!(config.section_order.is_empty());
    }

    #[test]
    fn single_section() {
        let mut config = File::try_from("[core]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionBodyId(0), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                section::Name(Cow::Borrowed("core".into())),
                vec![SectionBodyIds::Terminal(vec![SectionBodyId(0)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionBodyId(0),
                SectionBody(
                    vec![
                        newline_event(),
                        name_event("a"),
                        Event::KeyValueSeparator,
                        value_event("b"),
                        newline_event(),
                        name_event("c"),
                        Event::KeyValueSeparator,
                        value_event("d"),
                    ]
                    .into(),
                ),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionBodyId(0)]);
    }

    #[test]
    fn single_subsection() {
        let mut config = File::try_from("[core.sub]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionBodyId(0), section_header("core", (".", "sub")));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            let mut inner_tree = HashMap::new();
            inner_tree.insert(Cow::Borrowed("sub".into()), vec![SectionBodyId(0)]);
            tree.insert(
                section::Name(Cow::Borrowed("core".into())),
                vec![SectionBodyIds::NonTerminal(inner_tree)],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionBodyId(0),
                SectionBody(
                    vec![
                        newline_event(),
                        name_event("a"),
                        Event::KeyValueSeparator,
                        value_event("b"),
                        newline_event(),
                        name_event("c"),
                        Event::KeyValueSeparator,
                        value_event("d"),
                    ]
                    .into(),
                ),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionBodyId(0)]);
    }

    #[test]
    fn multiple_sections() {
        let mut config = File::try_from("[core]\na=b\nc=d\n[other]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionBodyId(0), section_header("core", None));
            map.insert(SectionBodyId(1), section_header("other", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                section::Name(Cow::Borrowed("core".into())),
                vec![SectionBodyIds::Terminal(vec![SectionBodyId(0)])],
            );
            tree.insert(
                section::Name(Cow::Borrowed("other".into())),
                vec![SectionBodyIds::Terminal(vec![SectionBodyId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionBodyId(0),
                SectionBody(
                    vec![
                        newline_event(),
                        name_event("a"),
                        Event::KeyValueSeparator,
                        value_event("b"),
                        newline_event(),
                        name_event("c"),
                        Event::KeyValueSeparator,
                        value_event("d"),
                        newline_event(),
                    ]
                    .into(),
                ),
            );
            sections.insert(
                SectionBodyId(1),
                SectionBody(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")].into()),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(
            config.section_order.make_contiguous(),
            &[SectionBodyId(0), SectionBodyId(1)]
        );
    }

    #[test]
    fn multiple_duplicate_sections() {
        let mut config = File::try_from("[core]\na=b\nc=d\n[core]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionBodyId(0), section_header("core", None));
            map.insert(SectionBodyId(1), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                section::Name(Cow::Borrowed("core".into())),
                vec![SectionBodyIds::Terminal(vec![SectionBodyId(0), SectionBodyId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionBodyId(0),
                SectionBody(
                    vec![
                        newline_event(),
                        name_event("a"),
                        Event::KeyValueSeparator,
                        value_event("b"),
                        newline_event(),
                        name_event("c"),
                        Event::KeyValueSeparator,
                        value_event("d"),
                        newline_event(),
                    ]
                    .into(),
                ),
            );
            sections.insert(
                SectionBodyId(1),
                SectionBody(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")].into()),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(
            config.section_order.make_contiguous(),
            &[SectionBodyId(0), SectionBodyId(1)]
        );
    }
}
