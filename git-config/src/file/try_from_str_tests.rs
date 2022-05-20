use super::{Cow, HashMap, LookupTreeNode, SectionId};
use crate::file::SectionBody;
use crate::parser::Event;
use crate::{
    parser::SectionHeaderName,
    test_util::{name_event, newline_event, section_header, value_event},
    File,
};
use std::convert::TryFrom;

#[test]
fn parse_empty() {
    let config = File::try_from("").unwrap();
    assert!(config.section_headers.is_empty());
    assert_eq!(config.section_id_counter, 0);
    assert!(config.section_lookup_tree.is_empty());
    assert!(config.sections.is_empty());
    assert!(config.section_order.is_empty());
}

#[test]
fn parse_single_section() {
    let mut config = File::try_from("[core]\na=b\nc=d").unwrap();
    let expected_separators = {
        let mut map = HashMap::new();
        map.insert(SectionId(0), section_header("core", None));
        map
    };
    assert_eq!(config.section_headers, expected_separators);
    assert_eq!(config.section_id_counter, 1);
    let expected_lookup_tree = {
        let mut tree = HashMap::new();
        tree.insert(
            SectionHeaderName(Cow::Borrowed("core")),
            vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
        );
        tree
    };
    assert_eq!(config.section_lookup_tree, expected_lookup_tree);
    let expected_sections = {
        let mut sections = HashMap::new();
        sections.insert(
            SectionId(0),
            SectionBody::from(vec![
                newline_event(),
                name_event("a"),
                Event::KeyValueSeparator,
                value_event("b"),
                newline_event(),
                name_event("c"),
                Event::KeyValueSeparator,
                value_event("d"),
            ]),
        );
        sections
    };
    assert_eq!(config.sections, expected_sections);
    assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
}

#[test]
fn parse_single_subsection() {
    let mut config = File::try_from("[core.sub]\na=b\nc=d").unwrap();
    let expected_separators = {
        let mut map = HashMap::new();
        map.insert(SectionId(0), section_header("core", (".", "sub")));
        map
    };
    assert_eq!(config.section_headers, expected_separators);
    assert_eq!(config.section_id_counter, 1);
    let expected_lookup_tree = {
        let mut tree = HashMap::new();
        let mut inner_tree = HashMap::new();
        inner_tree.insert(Cow::Borrowed("sub"), vec![SectionId(0)]);
        tree.insert(
            SectionHeaderName(Cow::Borrowed("core")),
            vec![LookupTreeNode::NonTerminal(inner_tree)],
        );
        tree
    };
    assert_eq!(config.section_lookup_tree, expected_lookup_tree);
    let expected_sections = {
        let mut sections = HashMap::new();
        sections.insert(
            SectionId(0),
            SectionBody::from(vec![
                newline_event(),
                name_event("a"),
                Event::KeyValueSeparator,
                value_event("b"),
                newline_event(),
                name_event("c"),
                Event::KeyValueSeparator,
                value_event("d"),
            ]),
        );
        sections
    };
    assert_eq!(config.sections, expected_sections);
    assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
}

#[test]
fn parse_multiple_sections() {
    let mut config = File::try_from("[core]\na=b\nc=d\n[other]e=f").unwrap();
    let expected_separators = {
        let mut map = HashMap::new();
        map.insert(SectionId(0), section_header("core", None));
        map.insert(SectionId(1), section_header("other", None));
        map
    };
    assert_eq!(config.section_headers, expected_separators);
    assert_eq!(config.section_id_counter, 2);
    let expected_lookup_tree = {
        let mut tree = HashMap::new();
        tree.insert(
            SectionHeaderName(Cow::Borrowed("core")),
            vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
        );
        tree.insert(
            SectionHeaderName(Cow::Borrowed("other")),
            vec![LookupTreeNode::Terminal(vec![SectionId(1)])],
        );
        tree
    };
    assert_eq!(config.section_lookup_tree, expected_lookup_tree);
    let expected_sections = {
        let mut sections = HashMap::new();
        sections.insert(
            SectionId(0),
            SectionBody::from(vec![
                newline_event(),
                name_event("a"),
                Event::KeyValueSeparator,
                value_event("b"),
                newline_event(),
                name_event("c"),
                Event::KeyValueSeparator,
                value_event("d"),
                newline_event(),
            ]),
        );
        sections.insert(
            SectionId(1),
            SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
        );
        sections
    };
    assert_eq!(config.sections, expected_sections);
    assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
}

#[test]
fn parse_multiple_duplicate_sections() {
    let mut config = File::try_from("[core]\na=b\nc=d\n[core]e=f").unwrap();
    let expected_separators = {
        let mut map = HashMap::new();
        map.insert(SectionId(0), section_header("core", None));
        map.insert(SectionId(1), section_header("core", None));
        map
    };
    assert_eq!(config.section_headers, expected_separators);
    assert_eq!(config.section_id_counter, 2);
    let expected_lookup_tree = {
        let mut tree = HashMap::new();
        tree.insert(
            SectionHeaderName(Cow::Borrowed("core")),
            vec![LookupTreeNode::Terminal(vec![SectionId(0), SectionId(1)])],
        );
        tree
    };
    assert_eq!(config.section_lookup_tree, expected_lookup_tree);
    let expected_sections = {
        let mut sections = HashMap::new();
        sections.insert(
            SectionId(0),
            SectionBody::from(vec![
                newline_event(),
                name_event("a"),
                Event::KeyValueSeparator,
                value_event("b"),
                newline_event(),
                name_event("c"),
                Event::KeyValueSeparator,
                value_event("d"),
                newline_event(),
            ]),
        );
        sections.insert(
            SectionId(1),
            SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
        );
        sections
    };
    assert_eq!(config.sections, expected_sections);
    assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
}
