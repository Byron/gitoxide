use crate::values::b;
use git_config::values::{Integer, IntegerSuffix};
use std::convert::TryFrom;

#[test]
fn from_str_no_suffix() {
    assert_eq!(Integer::try_from(b("1")).unwrap(), Integer { value: 1, suffix: None });

    assert_eq!(
        Integer::try_from(b("-1")).unwrap(),
        Integer {
            value: -1,
            suffix: None
        }
    );
}

#[test]
fn from_str_with_suffix() {
    assert_eq!(
        Integer::try_from(b("1k")).unwrap(),
        Integer {
            value: 1,
            suffix: Some(IntegerSuffix::Kibi),
        }
    );

    assert_eq!(
        Integer::try_from(b("1m")).unwrap(),
        Integer {
            value: 1,
            suffix: Some(IntegerSuffix::Mebi),
        }
    );

    assert_eq!(
        Integer::try_from(b("1g")).unwrap(),
        Integer {
            value: 1,
            suffix: Some(IntegerSuffix::Gibi),
        }
    );
}

#[test]
fn invalid_from_str() {
    assert!(Integer::try_from(b("")).is_err());
    assert!(Integer::try_from(b("-")).is_err());
    assert!(Integer::try_from(b("k")).is_err());
    assert!(Integer::try_from(b("m")).is_err());
    assert!(Integer::try_from(b("g")).is_err());
    assert!(Integer::try_from(b("123123123123123123123123")).is_err());
    assert!(Integer::try_from(b("gg")).is_err());
}

#[test]
fn as_decimal() {
    assert_eq!(
        Integer::try_from(b("12")).unwrap().as_decimal().unwrap(),
        12,
        "works without suffix"
    );
    assert_eq!(
        Integer::try_from(b("13k")).unwrap().as_decimal().unwrap(),
        13 * 1024,
        "works with kilobyte suffix"
    );
    assert_eq!(
        Integer::try_from(b("14m")).unwrap().as_decimal().unwrap(),
        14 * 1048576,
        "works with megabyte suffix"
    );
    assert_eq!(
        Integer::try_from(b("15g")).unwrap().as_decimal().unwrap(),
        15 * 1073741824,
        "works with gigabyte suffix"
    );

    let max_i64 = format!("{}g", i64::MAX);
    assert!(
        Integer::try_from(b(&max_i64)).unwrap().as_decimal().is_none(),
        "overflow results in None"
    );
}
