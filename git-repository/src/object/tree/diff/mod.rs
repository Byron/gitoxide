use crate::{bstr::BStr, Tree};

/// Returned by the `for_each` function to control flow.
#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    /// Continue the traversal of changes.
    Continue,
    /// Stop the traversal of changes and stop calling this function.
    Cancel,
}

impl Default for Action {
    fn default() -> Self {
        Action::Continue
    }
}

/// Represents any possible change in order to turn one tree into another.
#[derive(Debug, Clone, Copy)]
pub struct Change<'a, 'old, 'new> {
    /// The location of the file or directory described by `event`, if tracking was enabled.
    ///
    /// Otherwise this value is always an empty path.
    pub location: &'a BStr,
    /// The diff event itself to provide information about what would need to change.
    pub event: change::Event<'old, 'new>,
}

///
pub mod change;

/// Diffing
impl<'repo> Tree<'repo> {
    /// Return a platform to see the changes needed to create other trees, for instance.
    ///
    /// # Performance
    ///
    /// It's highly recommended to set an object cache to avoid extracting the same object multiple times.
    pub fn changes<'a>(&'a self) -> Platform<'a, 'repo> {
        Platform {
            state: Default::default(),
            lhs: self,
            tracking: None,
        }
    }
}

/// The diffing platform returned by [`Tree::changes()`].
#[derive(Clone)]
pub struct Platform<'a, 'repo> {
    state: git_diff::tree::State,
    lhs: &'a Tree<'repo>,
    tracking: Option<Tracking>,
}

#[derive(Clone, Copy)]
enum Tracking {
    FileName,
    Path,
}

/// Configuration
impl<'a, 'repo> Platform<'a, 'repo> {
    /// Keep track of file-names, which makes the [`location`][Change::location] field usable with the filename of the changed item.
    pub fn track_filename(&mut self) -> &mut Self {
        self.tracking = Some(Tracking::FileName);
        self
    }

    /// Keep track of the entire path of a change, relative to the repository.
    ///
    /// This makes the [`location`][Change::location] field usable.
    pub fn track_path(&mut self) -> &mut Self {
        self.tracking = Some(Tracking::Path);
        self
    }
}

///
pub mod for_each;
