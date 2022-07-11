use std::borrow::Cow;

use bstr::{BStr, BString, ByteSlice};

/// Removes quotes, if any, from the provided inputs. This assumes the input
/// contains a even number of unescaped quotes, and will unescape escaped
/// quotes. The return values should be safe for value interpretation.
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
/// # use git_config::value::normalize_bstr;
/// assert!(matches!(normalize_bstr("hello world"), Cow::Borrowed(_)));
/// ```
///
/// Internally quoted values are turned into owned variant with quotes removed.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::value::{normalize_bstr};
/// assert_eq!(normalize_bstr("hello \"world\""), Cow::<BStr>::Owned(BString::from("hello world")));
/// ```
///
/// Escaped quotes are unescaped.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::value::normalize_bstr;
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

    let mut prev_was_backslash = false;
    for c in input.iter().copied() {
        if prev_was_backslash {
            prev_was_backslash = false;
            match c {
                b'n' => out.push(b'\n'),
                b't' => out.push(b'\t'),
                b'b' => {
                    out.pop();
                }
                _ => out.push(c),
            };
        } else {
            match c {
                b'\\' => {
                    prev_was_backslash = true;
                }
                b'"' => {}
                _ => out.push(c),
            }
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
