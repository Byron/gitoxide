use crate::file::cow_str;
use bstr::BStr;
use git_config::{
    values::{Boolean, Bytes, TrueVariant, *},
    File,
};
use std::{borrow::Cow, convert::TryFrom, error::Error};

/// Asserts we can cast into all variants of our type
#[test]
fn get_value_for_all_provided_values() -> crate::Result {
    let config = r#"
        [core]
            bool-explicit = false
            bool-implicit
            integer-no-prefix = 10
            integer-prefix = 10g
            color = brightgreen red \
            bold
            other = hello world
            other-quoted = "hello world"
            location = ~/tmp
    "#;

    let file = File::try_from(config)?;

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-explicit")?,
        Boolean::False(Cow::Borrowed("false".into()))
    );
    assert!(!file.boolean("core", None, "bool-explicit").expect("exists")?);

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit")?,
        Boolean::True(TrueVariant::Implicit)
    );
    assert_eq!(
        file.try_value::<Boolean>("core", None, "bool-implicit")
            .expect("exists")?,
        Boolean::True(TrueVariant::Implicit)
    );

    assert!(file.boolean("core", None, "bool-implicit").expect("present")?);
    assert_eq!(file.try_value::<String>("doesnt", None, "exist"), None);

    assert_eq!(
        file.value::<Integer>("core", None, "integer-no-prefix")?,
        Integer {
            value: 10,
            suffix: None
        }
    );

    assert_eq!(
        file.value::<Integer>("core", None, "integer-no-prefix")?,
        Integer {
            value: 10,
            suffix: None
        }
    );

    assert_eq!(
        file.value::<Integer>("core", None, "integer-prefix")?,
        Integer {
            value: 10,
            suffix: Some(IntegerSuffix::Gibi),
        }
    );

    assert_eq!(
        file.value::<Color>("core", None, "color")?,
        Color {
            foreground: Some(ColorValue::BrightGreen),
            background: Some(ColorValue::Red),
            attributes: vec![ColorAttribute::Bold]
        }
    );

    assert_eq!(
        file.value::<Cow<'_, BStr>>("core", None, "other")?,
        cow_str("hello world")
    );
    assert_eq!(
        file.value::<String>("core", None, "other-quoted")?,
        String {
            value: cow_str("hello world")
        }
    );

    assert_eq!(
        file.string("core", None, "other").expect("present").as_ref(),
        "hello world"
    );
    assert_eq!(
        file.string("core", None, "other-quoted").expect("present").as_ref(),
        "hello world"
    );

    let actual = file.value::<git_config::values::Path>("core", None, "location")?;
    assert_eq!(&*actual, "~/tmp", "no interpolation occurs when querying a path");

    let home = std::env::current_dir()?;
    let expected = home.join("tmp");
    assert_eq!(actual.interpolate(None, home.as_path().into()).unwrap(), expected);

    let actual = file.path("core", None, "location").expect("present");
    assert_eq!(&*actual, "~/tmp",);

    Ok(())
}

/// There was a regression where lookup would fail because we only checked the
/// last section entry for any given section and subsection
#[test]
fn get_value_looks_up_all_sections_before_failing() -> crate::Result {
    let config = r#"
        [core]
            bool-explicit = false
            bool-implicit = false
        [core]
            bool-implicit
    "#;

    let file = File::try_from(config)?;

    // Checks that we check the last entry first still
    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit")?,
        Boolean::True(TrueVariant::Implicit)
    );

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-explicit")?,
        Boolean::False(cow_str("false"))
    );

    Ok(())
}

#[test]
fn section_names_are_case_insensitive() -> crate::Result {
    let config = "[core] bool-implicit";
    let file = File::try_from(config)?;
    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit").unwrap(),
        file.value::<Boolean>("CORE", None, "bool-implicit").unwrap()
    );

    Ok(())
}

#[test]
fn value_names_are_case_insensitive() -> crate::Result {
    let config = "[core]
        a = true
        A = false";
    let file = File::try_from(config)?;
    assert_eq!(file.multi_value::<Boolean>("core", None, "a")?.len(), 2);
    assert_eq!(
        file.value::<Boolean>("core", None, "a").unwrap(),
        file.value::<Boolean>("core", None, "A").unwrap()
    );

    Ok(())
}

#[test]
fn single_section() -> Result<(), Box<dyn Error>> {
    let config = File::try_from("[core]\na=b\nc").unwrap();
    let first_value: Bytes = config.value("core", None, "a")?;
    let second_value: Boolean = config.value("core", None, "c")?;

    assert_eq!(first_value, Bytes { value: cow_str("b") });
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

    let config = File::try_from(config).unwrap();
    let value = config.value::<Bytes>("remote", Some("origin"), "url").unwrap();
    assert_eq!(
        value,
        Bytes {
            value: cow_str("git@github.com:Byron/gitoxide.git")
        }
    );
}
