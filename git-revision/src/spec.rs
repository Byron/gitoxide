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
    use bstr::BStr;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
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
        fn resolve_ref(&mut self, name: &BStr) -> Option<()>;
        fn find_by_prefix(&mut self, input: &BStr) -> Option<()>;

        fn nth_ancestor(&mut self, n: usize) -> Option<()>;
        fn nth_parent(&mut self, n: usize) -> Option<()>;

        /// Set the kind of the specification, which happens only once if it happens at all.
        /// In case this method isn't called, assume `Single`.
        ///
        /// Note that ranges don't necessarily assure that a second specification will be parsed.
        /// If `^rev` is given, this method is called with [`spec::Kind::Range`][crate::spec::Kind::Range]
        /// and no second specification is provided.
        fn kind(&mut self, kind: crate::spec::Kind);
    }

    pub(crate) mod function {
        use crate::spec;
        use crate::spec::parse::{Delegate, Error};
        use bstr::{BStr, ByteSlice};

        pub fn revision<'a>(mut input: &'a BStr, delegate: &mut impl Delegate) -> Result<&'a BStr, Error> {
            if let Some(rest) = input.strip_prefix(b"@").or_else(|| input.strip_prefix(b"HEAD")) {
                delegate.resolve_ref("HEAD".into()).ok_or(Error::Delegate)?;
                input = rest.as_bstr();
            }
            Ok(input)
        }

        pub fn parse(mut input: &BStr, delegate: &mut impl Delegate) -> Result<(), Error> {
            if let Some(b'^') = input.get(0) {
                input = next(input).1;
                delegate.kind(spec::Kind::Range);
            }

            input = revision(input, delegate)?;
            if let Some((rest, kind)) = try_range(input) {
                // TODO: protect against double-kind calls, invalid for git
                delegate.kind(kind);
                input = rest.as_bstr();
            }

            assert!(
                input.is_empty(),
                "BUG: we must parse all of our input or fail gracefully: {:?}",
                input
            );
            Ok(())
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
