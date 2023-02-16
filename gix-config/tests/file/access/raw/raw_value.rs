use std::convert::TryFrom;

use git_config::{lookup, File};

#[test]
fn single_section() -> crate::Result {
    let config = File::try_from("[core]\na=b\nc=d")?;
    assert_eq!(config.raw_value("core", None, "a")?.as_ref(), "b");
    assert_eq!(config.raw_value("core", None, "c")?.as_ref(), "d");
    Ok(())
}

#[test]
fn last_one_wins_respected_in_section() -> crate::Result {
    let config = File::try_from("[core]\na=b\na=d")?;
    assert_eq!(config.raw_value("core", None, "a")?.as_ref(), "d");
    Ok(())
}

#[test]
fn last_one_wins_respected_across_section() -> crate::Result {
    let config = File::try_from("[core]\na=b\n[core]\na=d")?;
    assert_eq!(config.raw_value("core", None, "a")?.as_ref(), "d");
    Ok(())
}

#[test]
fn section_not_found() -> crate::Result {
    let config = File::try_from("[core]\na=b\nc=d")?;
    assert!(matches!(
        config.raw_value("foo", None, "a"),
        Err(lookup::existing::Error::SectionMissing)
    ));
    Ok(())
}

#[test]
fn subsection_not_found() -> crate::Result {
    let config = File::try_from("[core]\na=b\nc=d")?;
    assert!(matches!(
        config.raw_value("core", Some("a".into()), "a"),
        Err(lookup::existing::Error::SubSectionMissing)
    ));
    Ok(())
}

#[test]
fn key_not_found() -> crate::Result {
    let config = File::try_from("[core]\na=b\nc=d")?;
    assert!(matches!(
        config.raw_value("core", None, "aaaaaa"),
        Err(lookup::existing::Error::KeyMissing)
    ));
    Ok(())
}

#[test]
fn subsection_must_be_respected() -> crate::Result {
    let config = File::try_from("[core]a=b\n[core.a]a=c")?;
    assert_eq!(config.raw_value("core", None, "a")?.as_ref(), "b");
    assert_eq!(config.raw_value("core", Some("a".into()), "a")?.as_ref(), "c");
    Ok(())
}
