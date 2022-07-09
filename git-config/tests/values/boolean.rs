use std::convert::TryFrom;

use git_config::Boolean;

use crate::value::b;

#[test]
fn from_str_false() -> crate::Result {
    assert!(!Boolean::try_from(b("no"))?.0);
    assert!(!Boolean::try_from(b("off"))?.0);
    assert!(!Boolean::try_from(b("false"))?.0);
    assert!(!Boolean::try_from(b("zero"))?.0);
    assert!(!Boolean::try_from(b("\"\""))?.0);
    Ok(())
}

#[test]
fn from_str_true() {
    assert_eq!(Boolean::try_from(b("yes")).map(Into::into), Ok(true));
    assert_eq!(Boolean::try_from(b("on")), Ok(Boolean(true)));
    assert_eq!(Boolean::try_from(b("true")), Ok(Boolean(true)));
    assert_eq!(Boolean::try_from(b("one")), Ok(Boolean(true)));
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
