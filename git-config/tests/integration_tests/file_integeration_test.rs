use git_config::file::GitConfig;
use git_config::values::*;
use std::borrow::Cow;
use std::convert::TryFrom;

/// Asserts we can cast into all variants of our type
#[test]
fn get_value_for_all_provided_values() -> Result<(), Box<dyn std::error::Error>> {
    let config = r#"
        [core]
            bool-explicit = false
            bool-implicit
            integer-no-prefix = 10
            integer-prefix = 10g
            color = brightgreen red \
            bold
            other = hello world
    "#;

    let file = GitConfig::try_from(config)?;

    assert_eq!(
        file.get_value::<Boolean>("core", None, "bool-explicit")?,
        Boolean::False(Cow::Borrowed("false"))
    );

    assert_eq!(
        file.get_value::<Boolean>("core", None, "bool-implicit")?,
        Boolean::True(TrueVariant::Implicit)
    );

    assert_eq!(
        file.get_value::<Integer>("core", None, "integer-no-prefix")?,
        Integer {
            value: 10,
            suffix: None
        }
    );

    assert_eq!(
        file.get_value::<Integer>("core", None, "integer-no-prefix")?,
        Integer {
            value: 10,
            suffix: None
        }
    );

    assert_eq!(
        file.get_value::<Integer>("core", None, "integer-prefix")?,
        Integer {
            value: 10,
            suffix: Some(IntegerSuffix::Gibi),
        }
    );

    assert_eq!(
        file.get_value::<Color>("core", None, "color")?,
        Color {
            foreground: Some(ColorValue::BrightGreen),
            background: Some(ColorValue::Red),
            attributes: vec![ColorAttribute::Bold]
        }
    );

    assert_eq!(
        file.get_value::<Value>("core", None, "other")?,
        Value::Other(Cow::Borrowed(b"hello world"))
    );

    Ok(())
}
