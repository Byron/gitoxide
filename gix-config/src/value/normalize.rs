use std::borrow::Cow;

use bstr::{BStr, BString, ByteSlice};

/// Removes quotes, if any, from the provided inputs, and transforms
/// the 3 escape sequences `\n`, `\t` and `\b` into newline and tab
/// respectively, while `\b` will remove the previous character.
///
/// It assumes the input contains a even number of unescaped quotes,
/// and will unescape escaped quotes and everything else (even though the latter
/// would have been rejected in the parsing stage).
///
/// The return values should be safe for value interpretation.
///
/// This has optimizations for fully-quoted values, where the returned value
/// will be a borrowed reference if the only mutation necessary is to unquote
/// the value.
///
/// This is the function used to normalize raw values from higher level
/// abstractions. Generally speaking these
/// high level abstractions will handle normalization for you, and you do not
/// need to call this yourself. However, if you're directly handling events
/// from the parser, you may want to use this to help with value interpretation.
///
/// Generally speaking, you'll want to use one of the variants of this function,
/// such as [`normalize_bstr`] or [`normalize_bstring`].
///
/// # Examples
///
/// Values don't need modification are returned borrowed, without allocation.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::ByteSlice;
/// # use gix_config::value::normalize_bstr;
/// assert!(matches!(normalize_bstr("hello world"), Cow::Borrowed(_)));
/// ```
///
/// Internally quoted values are turned into owned variant with quotes removed.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use gix_config::value::{normalize_bstr};
/// assert_eq!(normalize_bstr("hello \"world\""), Cow::<BStr>::Owned(BString::from("hello world")));
/// ```
///
/// Escaped quotes are unescaped.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use gix_config::value::normalize_bstr;
/// assert_eq!(normalize_bstr(r#"hello "world\"""#), Cow::<BStr>::Owned(BString::from(r#"hello world""#)));
/// ```
#[must_use]
pub fn normalize(input: Cow<'_, BStr>) -> Cow<'_, BStr> {
    if input.as_ref() == "\"\"" {
        return Cow::Borrowed("".into());
    }

    let size = input.len();
    if size >= 3 && input[0] == b'"' && input[size - 1] == b'"' && input[size - 2] != b'\\' {
        match input {
            Cow::Borrowed(input) => return normalize_bstr(&input[1..size - 1]),
            Cow::Owned(mut input) => {
                input.pop();
                input.remove(0);
                return normalize_bstring(input);
            }
        }
    }

    if input.find_byteset(b"\\\"").is_none() {
        return input;
    }

    let mut out: BString = Vec::with_capacity(input.len()).into();
    let mut bytes = input.iter().copied();
    while let Some(c) = bytes.next() {
        match c {
            b'\\' => match bytes.next() {
                Some(b'n') => out.push(b'\n'),
                Some(b't') => out.push(b'\t'),
                Some(b'b') => {
                    out.pop();
                }
                Some(c) => {
                    out.push(c);
                }
                None => break,
            },
            b'"' => {}
            _ => out.push(c),
        }
    }
    Cow::Owned(out)
}

/// `&[u8]` variant of [`normalize`].
#[must_use]
pub fn normalize_bstr<'a>(input: impl Into<&'a BStr>) -> Cow<'a, BStr> {
    normalize(Cow::Borrowed(input.into()))
}

/// `Vec[u8]` variant of [`normalize`].
#[must_use]
pub fn normalize_bstring(input: impl Into<BString>) -> Cow<'static, BStr> {
    normalize(Cow::Owned(input.into()))
}
