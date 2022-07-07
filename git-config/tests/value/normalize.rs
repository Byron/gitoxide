use crate::file::cow_str;
use git_config::value::normalize_bstr;

#[test]
fn not_modified_is_borrowed() {
    assert_eq!(normalize_bstr("hello world"), cow_str("hello world"));
}

#[test]
fn modified_is_owned() {
    assert_eq!(normalize_bstr("hello \"world\""), cow_str("hello world").to_owned());
}

#[test]
fn all_quoted_is_optimized() {
    assert_eq!(normalize_bstr("\"hello world\""), cow_str("hello world"));
}

#[test]
fn all_quote_optimization_is_correct() {
    assert_eq!(normalize_bstr(r#""hello" world\""#), cow_str("hello world\""));
}

#[test]
fn quotes_right_next_to_each_other() {
    assert_eq!(normalize_bstr("\"hello\"\" world\""), cow_str("hello world").to_owned());
}

#[test]
fn escaped_quotes_are_kept() {
    assert_eq!(
        normalize_bstr(r#""hello \"\" world""#),
        cow_str("hello \"\" world").to_owned(),
    );
}

#[test]
fn empty_string() {
    assert_eq!(normalize_bstr(""), cow_str(""));
}

#[test]
fn empty_normalized_string_is_optimized() {
    assert_eq!(normalize_bstr("\"\""), cow_str(""));
}
