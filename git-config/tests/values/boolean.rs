use crate::values::b;
use git_config::values::{Boolean, TrueVariant};
use std::convert::TryFrom;

#[test]
fn from_str_false() {
    assert_eq!(Boolean::try_from(b("no")), Ok(Boolean::False("no".into())));
    assert_eq!(Boolean::try_from(b("off")), Ok(Boolean::False("off".into())));
    assert_eq!(Boolean::try_from(b("false")), Ok(Boolean::False("false".into())));
    assert_eq!(Boolean::try_from(b("zero")), Ok(Boolean::False("zero".into())));
    assert_eq!(Boolean::try_from(b("\"\"")), Ok(Boolean::False("\"\"".into())));
}

#[test]
fn from_str_true() {
    assert_eq!(
        Boolean::try_from(b("yes")),
        Ok(Boolean::True(TrueVariant::Explicit("yes".into())))
    );
    assert_eq!(
        Boolean::try_from(b("on")),
        Ok(Boolean::True(TrueVariant::Explicit("on".into())))
    );
    assert_eq!(
        Boolean::try_from(b("true")),
        Ok(Boolean::True(TrueVariant::Explicit("true".into())))
    );
    assert_eq!(
        Boolean::try_from(b("one")),
        Ok(Boolean::True(TrueVariant::Explicit("one".into())))
    );
}

#[test]
fn ignores_case() {
    // Random subset
    for word in &["no", "yes", "off", "true", "zero"] {
        let first: bool = Boolean::try_from(b(word)).unwrap().into();
        let second: bool = Boolean::try_from(b(&*word.to_uppercase())).unwrap().into();
        assert_eq!(first, second);
    }
}

#[test]
fn from_str_err() {
    assert!(Boolean::try_from(b("yesn't")).is_err());
    assert!(Boolean::try_from(b("yesno")).is_err());
}
