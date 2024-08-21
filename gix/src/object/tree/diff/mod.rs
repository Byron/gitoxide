use gix_diff::tree::recorder::Location;

use crate::bstr::BString;
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
    /// Otherwise, this value is always an empty path.
    pub location: &'a BStr,
    /// The diff event itself to provide information about what would need to change.
    pub event: change::Event<'a, 'old, 'new>,
}

/// Represents any possible change in order to turn one tree into another, the fully owned version
/// of [`Change`].
#[derive(Debug, Clone)]
pub struct ChangeDetached {
    /// The location of the file or directory described by `event`, if tracking was enabled.
    ///
    /// Otherwise, this value is always an empty path.
    pub location: BString,
    /// The diff event itself to provide information about what would need to change.
    pub event: change::EventDetached,
}

///
#[allow(clippy::empty_docs)]
pub mod change;

/// Diffing
impl<'repo> Tree<'repo> {
    /// Return a platform to see the changes needed to create other trees, for instance.
    ///
    /// # Performance
    ///
    /// It's highly recommended to [set an object cache](crate::Repository::compute_object_cache_size_for_tree_diffs)
    /// to avoid extracting the same object multiple times.
    /// By default, similar to `git diff`, rename tracking will be enabled if it is not configured.
    ///
    /// Note that if a clone with `--filter=blob=none` was created, rename tracking may fail as it might
    /// try to access blobs to compute a similarity metric. Thus, it's more compatible to turn rewrite tracking off
    /// using [`Platform::track_rewrites()`].
    #[allow(clippy::result_large_err)]
    #[doc(alias = "diff_tree_to_tree", alias = "git2")]
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
    /// what extent rename and copy tracking is performed.
    ///
    /// Note that by default, the git configuration determines rewrite tracking and git defaults are used
    /// if nothing is configured, which turns rename tracking with 50% similarity on, while not tracking copies at all.
    pub fn track_rewrites(&mut self, renames: Option<Rewrites>) -> &mut Self {
        self.rewrites = renames;
        self
    }
}

/// Provide aggregated information of a diff between two trees.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[doc(alias = "DiffStats", alias = "git2")]
pub struct Stats {
    /// The total amount of lines added in the between blobs of the two trees.
    #[doc(alias = "insertions", alias = "git2")]
    pub lines_added: u64,
    /// The total amount of lines removed in the between blobs of the two trees.
    #[doc(alias = "deletions", alias = "git2")]
    pub lines_removed: u64,
    /// The number of files that contributed to these statistics as they were added, removed or modified.
    pub files_changed: u64,
}

///
#[allow(clippy::empty_docs)]
pub mod stats {
    /// The error returned by [`stats()`](super::Platform::stats()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        CreateResourceCache(#[from] crate::repository::diff::resource_cache::Error),
        #[error(transparent)]
        ForEachChange(#[from] crate::object::tree::diff::for_each::Error),
    }
}

/// Convenience
impl<'a, 'repo> Platform<'a, 'repo> {
    /// Calculate statistics about the lines of the diff between our current and the `other` tree.
    ///
    /// ### Performance Notes
    ///
    /// Be sure to forcefully disable [`track_rewrites(None)`](Self::track_rewrites) to avoid
    /// rename tracking, an operation that doesn't affect the statistics currently.
    /// As diffed resources aren't cached, if highly repetitive blobs are expected, performance
    /// may be diminished. In real-world scenarios where blobs are mostly unique, that's not an issue though.
    pub fn stats(&mut self, other: &Tree<'_>) -> Result<Stats, stats::Error> {
        // let (mut number_of_files, mut lines_added, mut lines_removed) = (0, 0, 0);
        let mut resource_cache = self.lhs.repo.diff_resource_cache_for_tree_diff()?;

        let (mut files_changed, mut lines_added, mut lines_removed) = (0, 0, 0);
        self.for_each_to_obtain_tree(other, |change| {
            if let Some(counts) = change
                .diff(&mut resource_cache)
                .ok()
                .and_then(|mut platform| platform.line_counts().ok())
                .flatten()
            {
                files_changed += 1;
                lines_added += counts.insertions as u64;
                lines_removed += counts.removals as u64;
            }

            resource_cache.clear_resource_cache_keep_allocation();
            Ok::<_, std::convert::Infallible>(Action::Continue)
        })?;

        Ok(Stats {
            files_changed,
            lines_added,
            lines_removed,
        })
    }
}

///
#[allow(clippy::empty_docs)]
pub mod for_each;
