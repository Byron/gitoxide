#![allow(missing_docs)]

use crate::spec;
use bstr::BString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The opening brace in {:?} was not matched", .input)]
    UnclosedBracePair { input: BString },
    #[error("Cannot set spec kind more than once. Previous value was {:?}, now it is {:?}", .prev_kind, .kind)]
    KindSetTwice { prev_kind: spec::Kind, kind: spec::Kind },
    #[error("The @ character is either standing alone or followed by `{{<content>}}`, got {:?}", .input)]
    AtNeedsCurlyBrackets { input: BString },
    #[error("A portion of the input could not be parsed: {:?}", .input)]
    UnconsumedInput { input: BString },
    #[error("The delegate didn't indicate success - check delegate for more information")]
    Delegate,
}

///
pub mod delegate {
    use bstr::BStr;

    /// Usually the first methods to call when parsing a rev-spec to set an anchor.
    /// Methods can be called multiple time to either try input or to parse another rev-spec that is part of a range.
    /// In one case they will not be called at all: `@{[-]n}` indicates the current branch (what `HEAD` dereferences to),
    /// without ever naming it.
    pub trait Anchor {
        /// Resolve `name` as reference which might not be a valid reference name. The name may be partial like `main` or full like
        /// `refs/heads/main` solely depending on the users input.
        /// Symbolic referenced should be followed till their object, but objects must not yet be peeled.
        fn find_ref(&mut self, name: &BStr) -> Option<()>;
        /// An object prefix to disambiguate, returning `None` if it is ambiguous or wasn't found at all.
        fn disambiguate_prefix(&mut self, prefix: git_hash::Prefix) -> Option<()>;
    }

    /// Combine one or more specs into a range of multiple.
    pub trait Kind {
        /// Set the kind of the spec, which happens only once if it happens at all.
        /// In case this method isn't called, assume `Single`.
        /// Reject a kind by returning `None` to stop the parsing.
        ///
        /// Note that ranges don't necessarily assure that a second specification will be parsed.
        /// If `^rev` is given, this method is called with [`spec::Kind::Range`][crate::spec::Kind::Range]
        /// and no second specification is provided.
        fn kind(&mut self, kind: crate::spec::Kind) -> Option<()>;
    }

    /// Once an anchor is set one can adjust it using navigation methods.
    pub trait Navigation {
        fn nth_ancestor(&mut self, n: usize) -> Option<()>;
        fn nth_parent(&mut self, n: usize) -> Option<()>;
    }
}

/// A delegate to be informed about parse events, with methods split into categories.
///
/// - **Anchors** - which revision to use as starting point for…
/// - **Navigation** - where to go once from the initial revision
/// - **Range** - to learn if the specification is for a single or multiple references, and how to combine them.
pub trait Delegate: delegate::Anchor + delegate::Navigation + delegate::Kind {}

impl<T> Delegate for T where T: delegate::Anchor + delegate::Navigation + delegate::Kind {}

pub(crate) mod function {
    use crate::spec;
    use crate::spec::parse::{Delegate, Error};
    use bstr::{BStr, ByteSlice};

    fn try_set_prefix(delegate: &mut impl Delegate, hex_name: &BStr) -> Option<()> {
        git_hash::Prefix::from_hex(hex_name.to_str().expect("hexadecimal only"))
            .ok()
            .and_then(|prefix| delegate.disambiguate_prefix(prefix))
    }

    fn long_describe_prefix(name: &BStr) -> Option<&BStr> {
        let mut iter = name.rsplit(|b| *b == b'-');
        let candidate = iter.by_ref().find_map(|substr| {
            if substr.get(0)? != &b'g' {
                return None;
            };
            let rest = substr.get(1..)?;
            rest.iter().all(|b| b.is_ascii_hexdigit()).then(|| rest.as_bstr())
        });
        iter.any(|token| !token.is_empty()).then(|| candidate).flatten()
    }

    fn short_describe_prefix(name: &BStr) -> Option<&BStr> {
        let mut iter = name.split(|b| *b == b'-');
        let candidate = iter
            .next()
            .and_then(|prefix| prefix.iter().all(|b| b.is_ascii_hexdigit()).then(|| prefix.as_bstr()));
        (iter.count() == 1).then(|| candidate).flatten()
    }

    fn parens(input: &[u8]) -> Result<Option<(&BStr, &BStr)>, Error> {
        if input.get(0) != Some(&b'{') {
            return Ok(None);
        }
        let pos = input
            .find_byte(b'}')
            .ok_or_else(|| Error::UnclosedBracePair { input: input.into() })?;
        Ok(Some((input[1..pos].as_bstr(), input[pos + 1..].as_bstr())))
    }

    fn revision<'a>(mut input: &'a BStr, delegate: &mut impl Delegate) -> Result<&'a BStr, Error> {
        let mut cursor = input;
        let mut sep_pos = None;
        let mut consecutive_hex_chars = Some(0);
        while let Some((pos, b)) = cursor.iter().enumerate().find(|(_, b)| {
            if b"@~^:.".contains(b) {
                true
            } else {
                if let Some(num) = consecutive_hex_chars.as_mut() {
                    if b.is_ascii_hexdigit() {
                        *num += 1;
                    } else {
                        consecutive_hex_chars = None;
                    }
                }
                false
            }
        }) {
            if *b != b'.' || cursor.get(pos + 1) == Some(&b'.') {
                sep_pos = Some(pos);
                break;
            }
            cursor = &cursor[pos + 1..];
        }

        let name = &input[..sep_pos.unwrap_or(input.len())].as_bstr();
        let sep = sep_pos.map(|pos| cursor[pos]);
        if name.is_empty() && sep == Some(b'@') {
            delegate.find_ref("HEAD".into()).ok_or(Error::Delegate)?;
        } else {
            (consecutive_hex_chars.unwrap_or(0) >= git_hash::Prefix::MIN_HEX_LEN)
                .then(|| try_set_prefix(delegate, name))
                .flatten()
                .or_else(|| {
                    let prefix = long_describe_prefix(name).or_else(|| short_describe_prefix(name))?;
                    try_set_prefix(delegate, prefix)
                })
                .or_else(|| name.is_empty().then(|| ()).or_else(|| delegate.find_ref(name)))
                .ok_or(Error::Delegate)?;
        }

        let past_sep = input[sep_pos.map(|pos| pos + 1).unwrap_or(input.len())..].as_bstr();
        input = match sep {
            Some(b'@') => {
                match parens(past_sep)?.ok_or_else(|| Error::AtNeedsCurlyBrackets { input: past_sep.into() }) {
                    Ok((_nav, rest)) => rest,
                    Err(_) if name.is_empty() => past_sep,
                    Err(err) => return Err(err),
                }
            }
            Some(b'~') => todo!("~"),
            Some(b'^') => todo!("^"),
            Some(b':') => todo!(":"),
            Some(b'.') => input[sep_pos.unwrap_or(input.len())..].as_bstr(),
            None => past_sep,
            Some(unknown) => unreachable!("BUG: found unknown separation character {:?}", unknown),
        };
        Ok(input)
    }

    pub fn parse(mut input: &BStr, delegate: &mut impl Delegate) -> Result<(), Error> {
        let mut prev_kind = None;
        if let Some(b'^') = input.get(0) {
            input = next(input).1;
            delegate.kind(spec::Kind::Range).ok_or(Error::Delegate)?;
            prev_kind = spec::Kind::Range.into();
        }

        input = revision(input, delegate)?;
        if let Some((rest, kind)) = try_range(input) {
            if let Some(prev_kind) = prev_kind {
                return Err(Error::KindSetTwice { prev_kind, kind });
            }
            delegate.kind(kind).ok_or(Error::Delegate)?;
            input = revision(rest.as_bstr(), delegate)?;
        }

        if input.is_empty() {
            Ok(())
        } else {
            Err(Error::UnconsumedInput { input: input.into() })
        }
    }

    fn try_range(input: &BStr) -> Option<(&[u8], spec::Kind)> {
        input
            .strip_prefix(b"...")
            .map(|rest| (rest, spec::Kind::MergeBase))
            .or_else(|| input.strip_prefix(b"..").map(|rest| (rest, spec::Kind::Range)))
    }

    fn next(i: &BStr) -> (u8, &BStr) {
        let b = i[0];
        (b, i[1..].as_bstr())
    }
}
