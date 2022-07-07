use crate::file::cow_str;
use git_config::value::normalize_bstr;
use std::borrow::Cow;

#[test]
fn not_modified_is_borrowed() {
    let cow = normalize_bstr("hello world");
    assert_eq!(cow, cow_str("hello world"));
    assert!(matches!(cow, Cow::Borrowed(_)));
}

#[test]
fn modified_is_owned() {
    let cow = normalize_bstr("hello \"world\"");
    assert_eq!(cow, cow_str("hello world"));
    assert!(matches!(cow, Cow::Owned(_)));
}

#[test]
fn empty_quotes_are_zero_copy() {
    let cow = normalize_bstr("\"\"");
    assert_eq!(cow, cow_str(""));
    assert!(matches!(cow, Cow::Borrowed(_)));
}

#[test]
fn all_quoted_is_optimized() {
    let cow = normalize_bstr("\"hello world\"");
    assert_eq!(cow, cow_str("hello world"));
    assert!(matches!(cow, Cow::Borrowed(_)));
}

#[test]
fn all_quote_optimization_is_correct() {
    let cow = normalize_bstr(r#""hello" world\""#);
    assert_eq!(cow, cow_str("hello world\""));
    assert!(matches!(cow, Cow::Owned(_)));
}

#[test]
fn quotes_right_next_to_each_other() {
    let cow = normalize_bstr("\"hello\"\" world\"");
    assert_eq!(cow, cow_str("hello world").to_owned());
    assert!(matches!(cow, Cow::Owned(_)));
}

#[test]
fn escaped_quotes_are_kept() {
    let cow = normalize_bstr(r#""hello \"\" world""#);
    assert_eq!(cow, cow_str("hello \"\" world").to_owned(),);
    assert!(matches!(cow, Cow::Owned(_)));
}

#[test]
fn empty_string() {
    let cow = normalize_bstr("");
    assert_eq!(cow, cow_str(""));
    assert!(matches!(cow, Cow::Borrowed(_)));
}
