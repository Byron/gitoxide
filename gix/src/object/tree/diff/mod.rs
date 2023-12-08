use gix_diff::tree::recorder::Location;

use crate::{bstr::BStr, diff::Rewrites, Tree};

/// Returned by the `for_each` function to control flow.
#[derive(Default, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    /// Continue the traversal of changes.
    #[default]
    Continue,
    /// Stop the traversal of changes and stop calling this function.
    Cancel,
}

/// Represents any possible change in order to turn one tree into another.
#[derive(Debug, Clone, Copy)]
pub struct Change<'a, 'old, 'new> {
    /// The location of the file or directory described by `event`, if tracking was enabled.
    ///
    /// Otherwise this value is always an empty path.
    pub location: &'a BStr,
    /// The diff event itself to provide information about what would need to change.
    pub event: change::Event<'a, 'old, 'new>,
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
    /// By default, similar to `git diff`, rename tracking will be enabled if it is not configured.
    ///
    /// Note that if a clone with `--filter=blob=none` was created, rename tracking may fail as it might
    /// try to access blobs to compute a similarity metric. Thus, it's more compatible to turn rewrite tracking off
    /// using [`Platform::track_rewrites()`].
    #[allow(clippy::result_large_err)]
    pub fn changes<'a>(&'a self) -> Result<Platform<'a, 'repo>, crate::diff::new_rewrites::Error> {
        Ok(Platform {
            state: Default::default(),
            lhs: self,
            tracking: None,
            rewrites: self.repo.config.diff_renames()?.unwrap_or_default().into(),
        })
    }
}

/// The diffing platform returned by [`Tree::changes()`].
#[derive(Clone)]
pub struct Platform<'a, 'repo> {
    state: gix_diff::tree::State,
    lhs: &'a Tree<'repo>,
    tracking: Option<Location>,
    rewrites: Option<Rewrites>,
}

/// Configuration
impl<'a, 'repo> Platform<'a, 'repo> {
    /// Keep track of file-names, which makes the [`location`][Change::location] field usable with the filename of the changed item.
    pub fn track_filename(&mut self) -> &mut Self {
        self.tracking = Some(Location::FileName);
        self
    }

    /// Keep track of the entire path of a change, relative to the repository.
    ///
    /// This makes the [`location`][Change::location] field usable.
    pub fn track_path(&mut self) -> &mut Self {
        self.tracking = Some(Location::Path);
        self
    }

    /// Provide `None` to disable rewrite tracking entirely, or pass `Some(<configuration>)` to control to
    /// what extend rename and copy tracking is performed.
    ///
    /// Note that by default, the git configuration determines rewrite tracking and git defaults are used
    /// if nothing is configured, which turns rename tracking with 50% similarity on, while not tracking copies at all.
    pub fn track_rewrites(&mut self, renames: Option<Rewrites>) -> &mut Self {
        self.rewrites = renames;
        self
    }
}

///
pub mod for_each;
