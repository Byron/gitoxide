use std::{borrow::Cow, convert::TryFrom, error::Error};

use git_config::{
    file::GitConfig,
    values::{Boolean, Bytes, TrueVariant},
};

#[test]
fn single_section() -> Result<(), Box<dyn Error>> {
    let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
    let first_value: Bytes = config.value("core", None, "a")?;
    let second_value: Boolean = config.value("core", None, "c")?;

    assert_eq!(
        first_value,
        Bytes {
            value: Cow::Borrowed(b"b")
        }
    );
    assert_eq!(second_value, Boolean::True(TrueVariant::Implicit));

    Ok(())
}

#[test]
fn sections_by_name() {
    let config = r#"
    [core]
        repositoryformatversion = 0
        filemode = true
        bare = false
        logallrefupdates = true
    [remote "origin"]
        url = git@github.com:Byron/gitoxide.git
        fetch = +refs/heads/*:refs/remotes/origin/*
    "#;

    let config = GitConfig::try_from(config).unwrap();
    let value = config.value::<Bytes>("remote", Some("origin"), "url").unwrap();
    assert_eq!(
        value,
        Bytes {
            value: Cow::Borrowed(b"git@github.com:Byron/gitoxide.git")
        }
    );
}
