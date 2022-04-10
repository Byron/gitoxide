use git_config::values::normalize_str;
use std::borrow::Cow;

#[test]
fn not_modified_is_borrowed() {
    assert_eq!(normalize_str("hello world"), Cow::Borrowed(b"hello world"));
}

#[test]
fn modified_is_owned() {
    assert_eq!(
        normalize_str("hello \"world\""),
        Cow::<[u8]>::Owned(b"hello world".to_vec())
    );
}

#[test]
fn all_quoted_is_optimized() {
    assert_eq!(normalize_str("\"hello world\""), Cow::Borrowed(b"hello world"));
}

#[test]
fn all_quote_optimization_is_correct() {
    assert_eq!(normalize_str(r#""hello" world\""#), Cow::Borrowed(b"hello world\""));
}

#[test]
fn quotes_right_next_to_each_other() {
    assert_eq!(
        normalize_str("\"hello\"\" world\""),
        Cow::<[u8]>::Owned(b"hello world".to_vec())
    );
}

#[test]
fn escaped_quotes_are_kept() {
    assert_eq!(
        normalize_str(r#""hello \"\" world""#),
        Cow::<[u8]>::Owned(b"hello \"\" world".to_vec())
    );
}

#[test]
fn empty_string() {
    assert_eq!(normalize_str(""), Cow::Borrowed(b""));
}

#[test]
fn empty_normalized_string_is_optimized() {
    assert_eq!(normalize_str("\"\""), Cow::Borrowed(b""));
}
