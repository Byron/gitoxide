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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Options {
    /// Determine of the diff will be performed.
    /// Defaults to [`imara_diff::Algorithm::Myers`].
    pub diff_algorithm: imara_diff::Algorithm,
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
            diff_algorithm: imara_diff::Algorithm::Myers,
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

pub(super) mod function;
mod utils;
