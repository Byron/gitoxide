use std::{
    borrow::Cow,
    convert::TryFrom,
    path::{Path, PathBuf},
};

use git_config::{file::GitConfig, values::*};

#[test]
fn parse_config_with_windows_line_endings_successfully() -> crate::Result {
    GitConfig::open(Path::new("tests").join("fixtures").join("repo-config.crlf"))?;
    Ok(())
}

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
            location = ~/tmp
    "#;

    let file = GitConfig::try_from(config)?;

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-explicit")?,
        Boolean::False(Cow::Borrowed("false"))
    );

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit")?,
        Boolean::True(TrueVariant::Implicit)
    );

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
        file.value::<Value>("core", None, "other")?,
        Value::Other(Cow::Borrowed(b"hello world"))
    );

    let actual = file.value::<git_config::values::Path>("core", None, "location")?;
    assert_eq!(
        &*actual,
        "~/tmp".as_bytes(),
        "no interpolation occurs when querying a path due to lack of context"
    );
    let expected = PathBuf::from(format!("{}/tmp", dirs::home_dir().expect("empty home dir").display()));
    assert_eq!(actual.interpolate(None).unwrap(), expected);

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

    let file = GitConfig::try_from(config)?;

    // Checks that we check the last entry first still
    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit")?,
        Boolean::True(TrueVariant::Implicit)
    );

    assert_eq!(
        file.value::<Boolean>("core", None, "bool-explicit")?,
        Boolean::False(Cow::Borrowed("false"))
    );

    Ok(())
}

#[test]
fn section_names_are_case_insensitive() -> crate::Result {
    let config = "[core] bool-implicit";
    let file = GitConfig::try_from(config)?;
    assert!(file.value::<Boolean>("core", None, "bool-implicit").is_ok());
    assert_eq!(
        file.value::<Boolean>("core", None, "bool-implicit"),
        file.value::<Boolean>("CORE", None, "bool-implicit")
    );

    Ok(())
}

#[test]
fn value_names_are_case_insensitive() -> crate::Result {
    let config = "[core]
        a = true
        A = false";
    let file = GitConfig::try_from(config)?;
    assert_eq!(file.multi_value::<Boolean>("core", None, "a")?.len(), 2);
    assert_eq!(
        file.value::<Boolean>("core", None, "a"),
        file.value::<Boolean>("core", None, "A")
    );

    Ok(())
}
