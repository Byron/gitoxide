use std::convert::TryFrom;

use crate::file::cow_str;
use git_config::{boolean::True, Boolean};

use crate::value::b;

#[test]
fn from_str_false() {
    assert_eq!(Boolean::try_from(b("no")), Ok(Boolean::False(cow_str("no"))));
    assert_eq!(Boolean::try_from(b("off")), Ok(Boolean::False(cow_str("off"))));
    assert_eq!(Boolean::try_from(b("false")), Ok(Boolean::False(cow_str("false"))));
    assert_eq!(Boolean::try_from(b("zero")), Ok(Boolean::False(cow_str("zero"))));
    assert_eq!(Boolean::try_from(b("\"\"")), Ok(Boolean::False(cow_str("\"\""))));
}

#[test]
fn from_str_true() {
    assert_eq!(
        Boolean::try_from(b("yes")),
        Ok(Boolean::True(True::Explicit(cow_str("yes"))))
    );
    assert_eq!(
        Boolean::try_from(b("on")),
        Ok(Boolean::True(True::Explicit(cow_str("on"))))
    );
    assert_eq!(
        Boolean::try_from(b("true")),
        Ok(Boolean::True(True::Explicit(cow_str("true"))))
    );
    assert_eq!(
        Boolean::try_from(b("one")),
        Ok(Boolean::True(True::Explicit(cow_str("one"))))
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
