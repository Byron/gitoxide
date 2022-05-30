/// How to interpret a revision specification, or `revspec`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A single revision specification, pointing at one reference.
    Single,
    /// Two revision specifications `a` and `b` where we want all commits from `b` that are not also in `a`.
    Range,
    /// Everything in `a` and `b` but no commit from any of their merge bases.
    MergeBase,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Single
    }
}

pub mod parse {
    #![allow(missing_docs)]
    use bstr::{BStr, BString};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("The @ character is either standing alone or followed by `{{<content>}}`, got {:?}", .input)]
        AtNeedsCurlyBrackets { input: BString },
        #[error("A portion of the input could not be parsed: {:?}", .input)]
        UnconsumedInput { input: BString },
        #[error("The delegate didn't indicate success - check delegate for more information")]
        Delegate,
    }

    /// A delegate to be informed about parse events, with methods split into three categories.
    ///
    /// - **Revisions** - which revision to use as starting point for…
    /// - **Navigation** - where to go once from the initial revision.
    /// - **range** - to learn if the specification is for a single or multiple references.
    pub trait Delegate {
        /// Resolve `name` as reference which might not be a valid reference name. The name may be partial like `main` or full like
        /// `refs/heads/main` solely depending on the users input.
        /// Symbolic referenced should be followed till their object, but objects must not yet be peeled.
        fn set_ref(&mut self, name: &BStr) -> Option<()>;
        fn set_prefix(&mut self, prefix: git_hash::Prefix) -> Option<()>;

        fn nth_ancestor(&mut self, n: usize) -> Option<()>;
        fn nth_parent(&mut self, n: usize) -> Option<()>;

        /// Set the kind of the specification, which happens only once if it happens at all.
        /// In case this method isn't called, assume `Single`.
        /// Reject a kind by returning `None` to stop the parsing.
        ///
        /// Note that ranges don't necessarily assure that a second specification will be parsed.
        /// If `^rev` is given, this method is called with [`spec::Kind::Range`][crate::spec::Kind::Range]
        /// and no second specification is provided.
        fn kind(&mut self, kind: crate::spec::Kind) -> Option<()>;
    }

    pub(crate) mod function {
        use crate::spec;
        use crate::spec::parse::{Delegate, Error};
        use bstr::{BStr, ByteSlice};

        fn try_set_prefix(delegate: &mut impl Delegate, hex_name: &BStr) -> Option<()> {
            git_hash::Prefix::from_hex(hex_name.to_str().expect("hexadecimal only"))
                .ok()
                .and_then(|prefix| delegate.set_prefix(prefix))
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

        // TODO: impl
        fn parens(_input: &[u8]) -> Option<(&BStr, &BStr)> {
            None
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
                delegate.set_ref("HEAD".into()).ok_or(Error::Delegate)?;
            } else {
                (consecutive_hex_chars.unwrap_or(0) >= git_hash::Prefix::MIN_HEX_LEN)
                    .then(|| try_set_prefix(delegate, name))
                    .flatten()
                    .or_else(|| {
                        let prefix = long_describe_prefix(name).or_else(|| short_describe_prefix(name))?;
                        try_set_prefix(delegate, prefix)
                    })
                    .or_else(|| name.is_empty().then(|| ()).or_else(|| delegate.set_ref(name)))
                    .ok_or(Error::Delegate)?;
            }

            let past_sep = input[sep_pos.map(|pos| pos + 1).unwrap_or(input.len())..].as_bstr();
            input = match sep {
                Some(b'@') => {
                    match parens(past_sep).ok_or_else(|| Error::AtNeedsCurlyBrackets { input: past_sep.into() }) {
                        Ok((_spec, rest)) => rest,
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
            if let Some(b'^') = input.get(0) {
                input = next(input).1;
                delegate.kind(spec::Kind::Range).ok_or(Error::Delegate)?;
            }

            input = revision(input, delegate)?;
            if let Some((rest, kind)) = try_range(input) {
                // TODO: protect against double-kind calls, invalid for git
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
}
pub use parse::function::parse;
