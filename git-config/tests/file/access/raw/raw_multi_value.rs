use std::{borrow::Cow, convert::TryFrom};

use git_config::{lookup, File};

#[test]
fn single_value_is_identical_to_single_value_query() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert_eq!(
        vec![config.raw_value("core", None, "a").unwrap()],
        config.raw_multi_value("core", None, "a").unwrap()
    );
}

#[test]
fn multi_value_in_section() {
    let config = File::try_from("[core]\na=b\na=c").unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "a").unwrap(),
        vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
    );
}

#[test]
fn multi_value_across_sections() {
    let config = File::try_from("[core]\na=b\na=c\n[core]a=d").unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "a").unwrap(),
        vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
    );
}

#[test]
fn section_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_multi_value("foo", None, "a"),
        Err(lookup::existing::Error::SectionMissing)
    ));
}

#[test]
fn subsection_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_multi_value("core", Some("a"), "a"),
        Err(lookup::existing::Error::SubSectionMissing)
    ));
}

#[test]
fn key_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_multi_value("core", None, "aaaaaa"),
        Err(lookup::existing::Error::KeyMissing)
    ));
}

#[test]
fn subsection_must_be_respected() {
    let config = File::try_from("[core]a=b\n[core.a]a=c").unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "a").unwrap(),
        vec![Cow::Borrowed(b"b")]
    );
    assert_eq!(
        config.raw_multi_value("core", Some("a"), "a").unwrap(),
        vec![Cow::Borrowed(b"c")]
    );
}

#[test]
fn non_relevant_subsection_is_ignored() {
    let config = File::try_from("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
    assert_eq!(
        config.raw_multi_value("core", None, "a").unwrap(),
        vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
    );
}
