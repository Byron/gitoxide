use std::{borrow::Cow, convert::TryFrom};

use git_config::{lookup, File};

#[test]
fn single_section() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"b")
    );
    assert_eq!(
        config.raw_value("core", None, "c").unwrap(),
        Cow::<[u8]>::Borrowed(b"d")
    );
}

#[test]
fn last_one_wins_respected_in_section() {
    let config = File::try_from("[core]\na=b\na=d").unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"d")
    );
}

#[test]
fn last_one_wins_respected_across_section() {
    let config = File::try_from("[core]\na=b\n[core]\na=d").unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"d")
    );
}

#[test]
fn section_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_value("foo", None, "a"),
        Err(lookup::existing::Error::SectionMissing)
    ));
}

#[test]
fn subsection_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_value("core", Some("a"), "a"),
        Err(lookup::existing::Error::SubSectionMissing)
    ));
}

#[test]
fn key_not_found() {
    let config = File::try_from("[core]\na=b\nc=d").unwrap();
    assert!(matches!(
        config.raw_value("core", None, "aaaaaa"),
        Err(lookup::existing::Error::KeyMissing)
    ));
}

#[test]
fn subsection_must_be_respected() {
    let config = File::try_from("[core]a=b\n[core.a]a=c").unwrap();
    assert_eq!(
        config.raw_value("core", None, "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"b")
    );
    assert_eq!(
        config.raw_value("core", Some("a"), "a").unwrap(),
        Cow::<[u8]>::Borrowed(b"c")
    );
}
