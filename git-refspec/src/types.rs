use bstr::BStr;

/// The way to interpret a refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Mode {
    /// Apply standard rules for refspecs which are including refs with specific rules related to allowing fast forwards of destinations.
    Normal,
    /// Even though according to normal rules a non-fastforward would be denied, override this and reset a ref forcefully in the destination.
    Force,
    /// Instead of considering matching refs included, we consider them excluded. This applies only to the source side of a refspec.
    Negative,
}

/// What operation to perform with the refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Operation {
    /// The `src` side is local and the `dst` side is remote.
    Push,
    /// The `src` side is remote and the `dst` side is local.
    Fetch,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Instruction<'a> {
    Push(Push<'a>),
    Fetch(Fetch<'a>),
}

/// Note that all sources can either be a ref-name, partial or full, or a rev-spec, unless specified otherwise, on the local side.
/// Destinations can only be a partial or full ref names on the remote side.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Push<'a> {
    /// Push all local branches to the matching destination on the remote, which has to exist to be updated.
    AllMatchingBranches {
        /// If true, allow non-fast-forward updates of the matched destination branch.
        allow_non_fast_forward: bool,
    },
    /// Delete the destination ref or glob pattern, with only a single `*` allowed.
    Delete {
        /// The reference or pattern to delete on the remote.
        ref_or_pattern: &'a BStr,
    },
    /// Push a single ref or refspec to a known destination ref.
    Matching {
        /// The source ref or refspec to push. If pattern, it contains a single `*`.
        src: &'a BStr,
        /// The ref to update with the object from `src`. If `src`  is a pattern, this is a pattern too.
        dst: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
}

/// Note that any source can either be a ref name (full or partial) or a fully spelled out hex-sha for an object, on the remote side.
///
/// Destinations can only be a partial or full ref-names on the local side.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Fetch<'a> {
    Only {
        /// The ref name to fetch on the remote side, without updating the local side. This will write the result into `FETCH_HEAD`.
        src: &'a BStr,
    },
    /// Exclude a single ref.
    Exclude {
        /// A single partial or full ref name to exclude on the remote, or a pattern with a single `*`. It cannot be a spelled out object hash.
        src: &'a BStr,
    },
    AndUpdate {
        /// The ref name to fetch on the remote side, or a pattern with a single `*` to match against.
        src: &'a BStr,
        /// The local destination to update with what was fetched, or a pattern whose single `*` will be replaced with the matching portion
        /// of the `*` from `src`.
        dst: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
}

impl Instruction<'_> {
    pub fn operation(&self) -> Operation {
        match self {
            Instruction::Push(_) => Operation::Push,
            Instruction::Fetch(_) => Operation::Fetch,
        }
    }
}
