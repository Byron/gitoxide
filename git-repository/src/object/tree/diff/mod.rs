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
    #[allow(clippy::result_large_err)]
    pub fn changes<'a>(&'a self) -> Result<Platform<'a, 'repo>, renames::Error> {
        Ok(Platform {
            state: Default::default(),
            lhs: self,
            tracking: None,
            renames: self.repo.config.diff_renames()?.unwrap_or_default().into(),
        })
    }
}

/// The diffing platform returned by [`Tree::changes()`].
#[derive(Clone)]
pub struct Platform<'a, 'repo> {
    state: git_diff::tree::State,
    lhs: &'a Tree<'repo>,
    tracking: Option<Tracking>,
    renames: Option<Renames>,
}

#[derive(Clone, Copy)]
enum Tracking {
    FileName,
    Path,
}

/// A structure to capture how to perform rename tracking
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Renames {
    /// If `Some(…)`, do also find copies. `None` is the default which does not try to detect copies at all.
    ///
    /// Note that this is an even more expensive operation than detecting renames as files.
    pub copies: Option<renames::Copies>,
    /// The percentage of similarity needed for files to be considered renamed or copied, defaulting to `Some(0.5)`.
    /// This field is similar to `git diff -M50%`.
    ///
    /// If `None`, files are only considered equal if their content matches 100%.
    /// Note that values greater than 1.0 have no different effect than 1.0.
    pub percentage: Option<f32>,
    /// The amount of files to consider for rename or copy tracking. Defaults to 1000.
    /// If 0, there is no limit.
    pub limit: usize,
}

///
pub mod renames;

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

    /// Provide `None` to disable rename tracking entirely, or pass `Some(<configuration>)` to control to
    /// what extend rename tracking is performed.
    ///
    /// Note that by default, the configuration determines rename tracking and standard git defaults are used
    /// if nothing is configured, which turns on rename tracking with `-M50%`.
    pub fn track_renames(&mut self, renames: Option<Renames>) -> &mut Self {
        self.renames = renames;
        self
    }
}

///
pub mod for_each;
