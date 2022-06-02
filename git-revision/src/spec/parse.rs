#![allow(missing_docs)]

use crate::spec;
use bstr::BString;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse time {:?} for revlog lookup.", .input)]
    Time { input: BString },
    #[error("Sibling branches like 'upstream' or 'push' require a branch name with remote configuration, got {:?}", .name)]
    SiblingBranchNeedsBranchName { name: BString },
    #[error("Reflog entries require a ref name, got {:?}", .name)]
    ReflogLookupNeedsRefName { name: BString },
    #[error("A reference name must be followed by positive numbers in '@{{n}}', got {:?}", .nav)]
    RefnameNeedsPositiveReflogEntries { nav: BString },
    #[error("Negative or explicitly positive numbers are invalid here: {:?}", .input)]
    SignedNumber { input: BString },
    #[error("Negative zeroes are invalid: {:?} - remove the '-'", .input)]
    NegativeZero { input: BString },
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

    /// Usually the first methods to call when parsing a rev-spec to set an anchoring revision (which is typically a `Commit` object).
    /// Methods can be called multiple time to either try input or to parse another rev-spec that is part of a range.
    ///
    /// In one case they will not be called at all, e.g. `@{[-]n}` indicates the current branch (what `HEAD` dereferences to),
    /// without ever naming it, and so does `@{upstream}` or `@{<date>}`.
    ///
    /// Note that when dereferencing `HEAD` implicitly, a revision must be set for later navigation.
    pub trait Revision {
        /// Resolve `name` as reference which might not be a valid reference name. The name may be partial like `main` or full like
        /// `refs/heads/main` solely depending on the users input.
        /// Symbolic referenced should be followed till their object, but objects **must not yet** be peeled.
        fn find_ref(&mut self, name: &BStr) -> Option<()>;

        /// An object prefix to disambiguate, returning `None` if it is ambiguous or wasn't found at all.
        fn disambiguate_prefix(&mut self, prefix: git_hash::Prefix) -> Option<()>;

        /// Lookup the reflog of the previously set reference, or dereference `HEAD` to its reference
        /// to obtain the ref name (as opposed to `HEAD` itself).
        /// If there is no such reflog entry, return `None`.
        fn reflog(&mut self, query: ReflogLookup) -> Option<()>;

        /// When looking at `HEAD`, `branch_no` is the non-null checkout in the path, e.g. `1` means the last branch checked out,
        /// `2` is the one before that.
        /// Return `None` if there is no branch as the checkout history (via the reflog) isn't long enough.
        fn nth_checked_out_branch(&mut self, branch_no: usize) -> Option<()>;

        /// Lookup the previously set branch or dereference `HEAD` to its reference to use its name to lookup the sibling branch of `kind`
        /// in the configuration (typically in `refs/remotes/…`). The sibling branches are always local tracking branches.
        /// Return `None` of no such configuration exists and no sibling could be found, which is also the case for all reference outside
        /// of `refs/heads/`.
        /// Note that the caller isn't aware if the previously set reference is a branch or not and might call this method even though no reference
        /// is known.
        fn sibling_branch(&mut self, kind: SiblingBranch) -> Option<()>;
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

    /// Once an anchor is set one can adjust it using traversal methods.
    pub trait Navigate {
        /// Adjust the current revision to traverse the graph according to `kind`.
        fn traverse(&mut self, kind: Traversal) -> Option<()>;

        /// Peel the current object until it reached `kind` or `None` if the chain does not contain such object.
        fn peel_until(&mut self, kind: PeelTo) -> Option<()>;
    }

    /// A lookup into the reflog of a reference.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    pub enum ReflogLookup {
        /// Lookup by entry, where `0` is the most recent entry, and `1` is the older one behind `0`.
        Entry(usize),
        Date(git_date::Time),
    }

    /// Define how to traverse the commit graph.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    pub enum Traversal {
        /// Select the given parent commit of the currently selected commit, start at `1` for the first parent.
        /// The value will never be `0`.
        NthParent(usize),
    }

    /// Define where a tag object should be peeled to.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    pub enum PeelTo {
        /// An object of the given kind.
        ObjectKind(git_object::Kind),
    }

    /// The kind of sibling branch to obtain.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    pub enum SiblingBranch {
        /// The upstream branch as configured in `branch.<name>.remote` or `branch.<name>.merge`.
        Upstream,
        Push,
    }

    impl SiblingBranch {
        /// Parse `input` as branch representation, if possible.
        pub fn parse(input: &BStr) -> Option<Self> {
            if input.eq_ignore_ascii_case(b"u") || input.eq_ignore_ascii_case(b"upstream") {
                SiblingBranch::Upstream.into()
            } else if input.eq_ignore_ascii_case(b"push") {
                SiblingBranch::Push.into()
            } else {
                None
            }
        }
    }
}

/// A delegate to be informed about parse events, with methods split into categories.
///
/// - **Anchors** - which revision to use as starting point for…
/// - **Navigation** - where to go once from the initial revision
/// - **Range** - to learn if the specification is for a single or multiple references, and how to combine them.
pub trait Delegate: delegate::Revision + delegate::Navigate + delegate::Kind {}

impl<T> Delegate for T where T: delegate::Revision + delegate::Navigate + delegate::Kind {}

pub(crate) mod function {
    use crate::spec;
    use crate::spec::parse::delegate::SiblingBranch;
    use crate::spec::parse::{delegate, Delegate, Error};
    use bstr::{BStr, ByteSlice};
    use std::convert::TryInto;
    use std::str::FromStr;

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

    fn try_parse<T: FromStr + PartialEq + Default>(input: &BStr) -> Result<Option<T>, Error> {
        input
            .to_str()
            .ok()
            .and_then(|n| {
                n.parse().ok().map(|n| {
                    if n == T::default() && input[0] == b'-' {
                        return Err(Error::NegativeZero { input: input.into() });
                    };
                    Ok(n)
                })
            })
            .transpose()
    }

    fn revision<'a>(mut input: &'a BStr, delegate: &mut impl Delegate) -> Result<&'a BStr, Error> {
        let mut sep_pos = None;
        let mut consecutive_hex_chars = Some(0);
        {
            let mut cursor = input;
            let mut ofs = 0;
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
                    sep_pos = Some(ofs + pos);
                    break;
                }
                ofs += pos + 1;
                cursor = &cursor[pos + 1..];
            }
        }

        let name = &input[..sep_pos.unwrap_or(input.len())].as_bstr();
        let mut sep = sep_pos.map(|pos| input[pos]);
        let mut has_ref_or_implied_name = name.is_empty();
        if name.is_empty() && sep == Some(b'@') && sep_pos.and_then(|pos| input.get(pos + 1)) != Some(&b'{') {
            delegate.find_ref("HEAD".into()).ok_or(Error::Delegate)?;
            sep_pos = sep_pos.map(|pos| pos + 1);
            sep = match sep_pos.and_then(|pos| input.get(pos).copied()) {
                None => return Ok("".into()),
                Some(pos) => Some(pos),
            };
        } else {
            (consecutive_hex_chars.unwrap_or(0) >= git_hash::Prefix::MIN_HEX_LEN)
                .then(|| try_set_prefix(delegate, name))
                .flatten()
                .or_else(|| {
                    let prefix = long_describe_prefix(name).or_else(|| short_describe_prefix(name))?;
                    try_set_prefix(delegate, prefix)
                })
                .or_else(|| {
                    name.is_empty().then(|| ()).or_else(|| {
                        let res = delegate.find_ref(name)?;
                        has_ref_or_implied_name = true;
                        res.into()
                    })
                })
                .ok_or(Error::Delegate)?;
        }

        let past_sep = input[sep_pos.map(|pos| pos + 1).unwrap_or(input.len())..].as_bstr();
        input = match sep {
            Some(b'@') => {
                let (nav, rest) = parens(past_sep)?.ok_or_else(|| Error::AtNeedsCurlyBrackets {
                    input: input[sep_pos.unwrap_or(input.len())..].into(),
                })?;
                if let Some(n) = try_parse::<isize>(nav)? {
                    if n < 0 {
                        if name.is_empty() {
                            delegate
                                .nth_checked_out_branch(n.abs().try_into().expect("non-negative isize fits usize"))
                                .ok_or(Error::Delegate)?;
                        } else {
                            return Err(Error::RefnameNeedsPositiveReflogEntries { nav: nav.into() });
                        }
                    } else if has_ref_or_implied_name {
                        delegate
                            .reflog(delegate::ReflogLookup::Entry(
                                n.try_into().expect("non-negative isize fits usize"),
                            ))
                            .ok_or(Error::Delegate)?;
                    } else {
                        return Err(Error::ReflogLookupNeedsRefName { name: (*name).into() });
                    }
                } else if let Some(kind) = SiblingBranch::parse(nav) {
                    if has_ref_or_implied_name {
                        delegate.sibling_branch(kind).ok_or(Error::Delegate)
                    } else {
                        Err(Error::SiblingBranchNeedsBranchName { name: (*name).into() })
                    }?
                } else if has_ref_or_implied_name {
                    let time = git_date::parse(nav).ok_or_else(|| Error::Time { input: nav.into() })?;
                    delegate
                        .reflog(delegate::ReflogLookup::Date(time))
                        .ok_or(Error::Delegate)?;
                } else {
                    return Err(Error::ReflogLookupNeedsRefName { name: (*name).into() });
                }
                rest
            }
            Some(b'~') => todo!("~"),
            Some(b'^') => {
                if let Some((number, rest)) = try_parse_usize(past_sep)? {
                    if number == 0 {
                        delegate.peel_until(delegate::PeelTo::ObjectKind(git_object::Kind::Commit))
                    } else {
                        delegate.traverse(delegate::Traversal::NthParent(number))
                    }
                    .ok_or(Error::Delegate)?;
                    rest
                } else if let Some((_kind, _rest)) = parens(past_sep)? {
                    todo!("try ^{{…}}")
                } else {
                    delegate
                        .traverse(delegate::Traversal::NthParent(1))
                        .ok_or(Error::Delegate)?;
                    past_sep
                }
            }
            Some(b':') => todo!(":"),
            Some(b'.') => input[sep_pos.unwrap_or(input.len())..].as_bstr(),
            None => past_sep,
            Some(unknown) => unreachable!("BUG: found unknown separation character {:?}", unknown as char),
        };
        Ok(input)
    }

    fn try_parse_usize(input: &BStr) -> Result<Option<(usize, &BStr)>, Error> {
        let mut bytes = input.iter().peekable();
        if bytes.peek().filter(|&&&b| b == b'-' || b == b'+').is_some() {
            return Err(Error::SignedNumber { input: input.into() });
        }
        let num_digits = bytes.take_while(|b| b.is_ascii_digit()).count();
        if num_digits == 0 {
            return Ok(None);
        }
        let number = try_parse(&input[..num_digits])?.expect("parse number if only digits");
        Ok(Some((number, input[num_digits..].as_bstr())))
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
