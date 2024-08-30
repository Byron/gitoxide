use crate::blob::BuiltinDriver;

impl BuiltinDriver {
    /// Return the name of this instance.
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinDriver::Text => "text",
            BuiltinDriver::Binary => "binary",
            BuiltinDriver::Union => "union",
        }
    }

    /// Get all available built-in drivers.
    pub fn all() -> &'static [Self] {
        &[BuiltinDriver::Text, BuiltinDriver::Binary, BuiltinDriver::Union]
    }

    /// Try to match one of our variants to `name`, case-sensitive, and return its instance.
    pub fn by_name(name: &str) -> Option<Self> {
        Self::all().iter().find(|variant| variant.as_str() == name).copied()
    }
}

///
pub mod binary {
    use crate::blob::Resolution;

    /// What to do when having to pick a side to resolve a conflict.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum ResolveWith {
        /// Chose the ancestor to resolve a conflict.
        Ancestor,
        /// Chose our side to resolve a conflict.
        Ours,
        /// Chose their side to resolve a conflict.
        Theirs,
    }

    /// Tell the caller of [`merge()`] which side was picked
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Pick {
        /// Chose the ancestor.
        Ancestor,
        /// Chose our side.
        Ours,
        /// Chose their side.
        Theirs,
    }

    /// As this algorithm doesn't look at the actual data, it returns a choice solely based on logic.
    ///
    /// It always results in a conflict with `current` being picked unless `on_conflict` is not `None`.
    pub fn merge(on_conflict: Option<ResolveWith>) -> (Pick, Resolution) {
        match on_conflict {
            None => (Pick::Ours, Resolution::Conflict),
            Some(ResolveWith::Ours) => (Pick::Ours, Resolution::Complete),
            Some(ResolveWith::Theirs) => (Pick::Theirs, Resolution::Complete),
            Some(ResolveWith::Ancestor) => (Pick::Ancestor, Resolution::Complete),
        }
    }
}

///
pub mod text {
    use crate::blob::Resolution;

    /// The way the built-in [text driver](crate::blob::BuiltinDriver::Text) will express
    /// merge conflicts in the resulting file.
    #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum ConflictStyle {
        /// Only show the zealously minified conflicting lines of the local changes and the incoming (other) changes,
        /// hiding the base version entirely.
        ///
        /// ```
        /// line1-changed-by-both
        /// <<<<<<< local
        /// line2-to-be-changed-in-incoming
        /// =======
        /// line2-changed
        /// >>>>>>> incoming
        ///```
        #[default]
        Merge,
        /// Show non-minimized hunks of local changes, the base, and the incoming (other) changes.
        ///
        /// This mode does not hide any information.
        /// ```
        /// <<<<<<< local
        /// line1-changed-by-both
        /// line2-to-be-changed-in-incoming
        /// ||||||| 9a8d80c
        /// line1-to-be-changed-by-both
        /// line2-to-be-changed-in-incoming
        /// =======
        /// line1-changed-by-both
        /// line2-changed
        /// >>>>>>> incoming
        ///```
        Diff3,
        /// Like [`Diff3](Self::Diff3), but will show *minimized* hunks of local change and the incoming (other) changes,
        /// as well as non-minimized hunks of the base.
        ///
        /// ```
        /// line1-changed-by-both
        /// <<<<<<< local
        /// line2-to-be-changed-in-incoming
        /// ||||||| 9a8d80c
        /// line1-to-be-changed-by-both
        /// line2-to-be-changed-in-incoming
        /// =======
        /// line2-changed
        /// >>>>>>> incoming
        /// ```
        ZealousDiff3,
    }

    /// Options for the builtin [text driver](crate::blob::BuiltinDriver::Text).
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Options {
        /// How to visualize conflicts in merged files.
        pub conflict_style: ConflictStyle,
        /// The amount of markers to draw, defaults to 7, i.e. `<<<<<<<`
        pub marker_size: usize,
        /// Decide what to do to automatically resolve conflicts.
        /// If `None`, add conflict markers according to `conflict_style` and `marker_size`.
        pub on_conflict: Option<ResolveWith>,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                conflict_style: Default::default(),
                marker_size: 7,
                on_conflict: None,
            }
        }
    }

    /// What to do to resolve a conflict.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum ResolveWith {
        /// Chose our side to resolve a conflict.
        Ours,
        /// Chose their side to resolve a conflict.
        Theirs,
        /// Place our and their lines one after another, in any order
        Union,
    }

    /// Merge `current` and `other` with `ancestor` as base according to `opts`.
    ///
    /// Place the merged result in `out` and return the resolution.
    pub fn merge(_out: &mut Vec<u8>, _current: &[u8], _ancestor: &[u8], _other: &[u8], _opts: Options) -> Resolution {
        todo!("text merge");
    }
}
