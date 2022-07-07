use std::str::FromStr;

use git_config::value::ColorAttribute;

#[test]
fn non_inverted() {
    assert_eq!(ColorAttribute::from_str("bold"), Ok(ColorAttribute::Bold));
    assert_eq!(ColorAttribute::from_str("dim"), Ok(ColorAttribute::Dim));
    assert_eq!(ColorAttribute::from_str("ul"), Ok(ColorAttribute::Ul));
    assert_eq!(ColorAttribute::from_str("blink"), Ok(ColorAttribute::Blink));
    assert_eq!(ColorAttribute::from_str("reverse"), Ok(ColorAttribute::Reverse));
    assert_eq!(ColorAttribute::from_str("italic"), Ok(ColorAttribute::Italic));
    assert_eq!(ColorAttribute::from_str("strike"), Ok(ColorAttribute::Strike));
}

#[test]
fn inverted_no_dash() {
    assert_eq!(ColorAttribute::from_str("nobold"), Ok(ColorAttribute::NoBold));
    assert_eq!(ColorAttribute::from_str("nodim"), Ok(ColorAttribute::NoDim));
    assert_eq!(ColorAttribute::from_str("noul"), Ok(ColorAttribute::NoUl));
    assert_eq!(ColorAttribute::from_str("noblink"), Ok(ColorAttribute::NoBlink));
    assert_eq!(ColorAttribute::from_str("noreverse"), Ok(ColorAttribute::NoReverse));
    assert_eq!(ColorAttribute::from_str("noitalic"), Ok(ColorAttribute::NoItalic));
    assert_eq!(ColorAttribute::from_str("nostrike"), Ok(ColorAttribute::NoStrike));
}

#[test]
fn inverted_dashed() {
    assert_eq!(ColorAttribute::from_str("no-bold"), Ok(ColorAttribute::NoBold));
    assert_eq!(ColorAttribute::from_str("no-dim"), Ok(ColorAttribute::NoDim));
    assert_eq!(ColorAttribute::from_str("no-ul"), Ok(ColorAttribute::NoUl));
    assert_eq!(ColorAttribute::from_str("no-blink"), Ok(ColorAttribute::NoBlink));
    assert_eq!(ColorAttribute::from_str("no-reverse"), Ok(ColorAttribute::NoReverse));
    assert_eq!(ColorAttribute::from_str("no-italic"), Ok(ColorAttribute::NoItalic));
    assert_eq!(ColorAttribute::from_str("no-strike"), Ok(ColorAttribute::NoStrike));
}

#[test]
fn invalid() {
    assert!(ColorAttribute::from_str("a").is_err());
    assert!(ColorAttribute::from_str("no bold").is_err());
    assert!(ColorAttribute::from_str("").is_err());
    assert!(ColorAttribute::from_str("no").is_err());
    assert!(ColorAttribute::from_str("no-").is_err());
}
