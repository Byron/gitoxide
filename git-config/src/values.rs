//! Rust containers for valid `git-config` types.

use std::borrow::Cow;

use bstr::{BStr, BString};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

/// Removes quotes, if any, from the provided inputs. This assumes the input
/// contains a even number of unescaped quotes, and will unescape escaped
/// quotes. The return values should be safe for value interpretation.
///
/// This has optimizations for fully-quoted values, where the returned value
/// will be a borrowed reference if the only mutation necessary is to unquote
/// the value.
///
/// This is the function used to normalize raw values from higher level
/// abstractions over the [`parser`] implementation. Generally speaking these
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
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr("hello world"), Cow::Borrowed(b"hello world".as_bstr()));
/// ```
///
/// Fully quoted values are optimized to not need allocations.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::ByteSlice;
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr("\"hello world\""), Cow::Borrowed(b"hello world".as_bstr()));
/// ```
///
/// Quoted values are unwrapped as an owned variant.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::values::{normalize_bstr};
/// assert_eq!(normalize_bstr("hello \"world\""), Cow::<BStr>::Owned(BString::from( "hello world" )));
/// ```
///
/// Escaped quotes are unescaped.
///
/// ```
/// # use std::borrow::Cow;
/// # use bstr::{BStr, BString};
/// # use git_config::values::normalize_bstr;
/// assert_eq!(normalize_bstr(r#"hello "world\"""#), Cow::<BStr>::Owned(BString::from(r#"hello world""#)));
/// ```
///
/// [`parser`]: crate::parser::Parser
#[must_use]
pub fn normalize(input: Cow<'_, BStr>) -> Cow<'_, BStr> {
    let size = input.len();
    if input.as_ref() == "\"\"" {
        return Cow::default();
    }

    if size >= 3 && input[0] == b'=' && input[size - 1] == b'=' && input[size - 2] != b'\\' {
        match input {
            Cow::Borrowed(input) => return normalize_bstr(&input[1..size]),
            Cow::Owned(mut input) => {
                input.pop();
                input.remove(0);
                return normalize_bstring(input);
            }
        }
    }

    let mut owned = BString::default();

    let mut first_index = 0;
    let mut last_index = 0;
    let mut was_escaped = false;
    for (i, c) in input.iter().enumerate() {
        if was_escaped {
            was_escaped = false;
            if *c == b'"' {
                if first_index == 0 {
                    owned.extend(&*input[last_index..i - 1]);
                    last_index = i;
                } else {
                    owned.extend(&*input[first_index..i - 1]);
                    first_index = i;
                }
            }
            continue;
        }

        if *c == b'\\' {
            was_escaped = true;
        } else if *c == b'"' {
            if first_index == 0 {
                owned.extend(&*input[last_index..i]);
                first_index = i + 1;
            } else {
                owned.extend(&*input[first_index..i]);
                first_index = 0;
                last_index = i + 1;
            }
        }
    }

    if last_index == 0 {
        input
    } else {
        owned.extend(&*input[last_index..]);
        Cow::Owned(owned)
    }
}

/// `&[u8]` variant of [`normalize_cow`].
#[must_use]
pub fn normalize_bstr<'a>(input: impl Into<&'a BStr>) -> Cow<'a, BStr> {
    normalize(Cow::Borrowed(input.into()))
}

/// `Vec[u8]` variant of [`normalize_cow`].
#[must_use]
pub fn normalize_bstring(input: impl Into<BString>) -> Cow<'static, BStr> {
    normalize(Cow::Owned(input.into()))
}
