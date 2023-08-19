//! Pathspec plumbing and abstractions
use crate::bstr::BStr;
use crate::{Pathspec, Repository};
use gix_odb::FindExt;
pub use gix_pathspec::*;

///
pub mod init {
    /// The error returned by [`Pathspec::new()`](super::Pathspec::new()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Filesystem configuration could not be obtained")]
        FilesystemConfig(#[from] crate::config::boolean::Error),
        #[error(transparent)]
        MakeAttributes(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
        #[error(transparent)]
        Defaults(#[from] gix_pathspec::defaults::from_environment::Error),
        #[error(transparent)]
        ParseSpec(#[from] gix_pathspec::parse::Error),
        #[error(
            "Could not obtain the repository prefix as the relative path of the CWD as seen from the working tree"
        )]
        NormalizeSpec(#[from] gix_pathspec::normalize::Error),
        #[error(transparent)]
        RepoPrefix(#[from] gix_path::realpath::Error),
    }
}

/// Lifecycle
impl<'repo> Pathspec<'repo> {
    /// Create a new instance by parsing `patterns` into [`Pathspecs`](Pattern) to make them usable for searches.
    /// `make_attribute` may be called if one of the patterns has a `(attr:a)` element which requires attribute matching. It should
    /// be used to control where attributes are coming from.
    /// If `inherit_ignore_case` is `true`, the pathspecs may have their ignore-case default overridden to be case-insensitive by default.
    /// This only works towards turning ignore-case for pathspecs on, but won't ever turn that setting off if.
    ///
    /// ### Deviation
    ///
    /// Pathspecs can declare to be case-insensitive as part of their elements, which is a setting that is now respected for attribute
    /// queries as well.
    pub fn new(
        repo: &'repo Repository,
        patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
        inherit_ignore_case: bool,
        make_attributes: impl FnOnce() -> Result<gix_worktree::Stack, Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Result<Self, init::Error> {
        let mut defaults = repo.pathspec_defaults()?;
        if inherit_ignore_case && repo.config.fs_capabilities()?.ignore_case {
            defaults.signature |= MagicSignature::ICASE;
        }
        let patterns = patterns
            .into_iter()
            .map(move |p| parse(p.as_ref(), defaults))
            .collect::<Result<Vec<_>, _>>()?;
        let needs_cache = patterns.iter().any(|p| !p.attributes.is_empty());
        let search = Search::from_specs(
            patterns,
            repo.prefix()?,
            &gix_path::realpath_opts(
                repo.work_dir().unwrap_or_else(|| repo.git_dir()),
                repo.options.current_dir_or_empty(),
                gix_path::realpath::MAX_SYMLINKS,
            )?,
        )?;
        let cache = needs_cache.then(make_attributes).transpose()?;
        Ok(Self { repo, search, cache })
    }
    /// Turn ourselves into the functional parts for direct usage.
    /// Note that the [`cache`](gix_worktree::Stack) is only set if one of the [`search` patterns](Search)
    /// is specifying attributes to match for.
    pub fn into_parts(self) -> (Search, Option<gix_worktree::Stack>) {
        (self.search, self.cache)
    }
}

/// Access
impl<'repo> Pathspec<'repo> {
    /// Return the attributes cache which is used when matching attributes in pathspecs, or `None` if none of the pathspecs require that.
    pub fn attributes(&self) -> Option<&gix_worktree::Stack> {
        self.cache.as_ref()
    }

    /// Return the search itself which can be used for matching paths or accessing the actual patterns that will be used.
    pub fn search(&self) -> &gix_pathspec::Search {
        &self.search
    }

    /// Return the first [`Match`](search::Match) of `relative_path`, or `None`.
    /// Note that the match might [be excluded](search::Match::is_excluded()).
    /// `is_dir` is true if `relative_path` is a directory.
    pub fn pattern_matching_relative_path<'a>(
        &mut self,
        relative_path: impl Into<&'a BStr>,
        is_dir: Option<bool>,
    ) -> Option<gix_pathspec::search::Match<'_>> {
        self.search
            .pattern_matching_relative_path(relative_path, is_dir, |relative_path, case, is_dir, out| {
                let cache = self.cache.as_mut().expect("initialized in advance");
                cache
                    .set_case(case)
                    .at_entry(relative_path, Some(is_dir), |id, buf| {
                        self.repo.objects.find_blob(id, buf)
                    })
                    .map_or(false, |platform| platform.matching_attributes(out))
            })
    }

    /// The simplified version of [`pattern_matching_relative_path()`](Self::pattern_matching_relative_path()) which returns
    /// `true` if `relative_path` is included in the set of positive pathspecs, while not being excluded.
    pub fn is_included<'a>(&mut self, relative_path: impl Into<&'a BStr>, is_dir: Option<bool>) -> bool {
        self.pattern_matching_relative_path(relative_path, is_dir)
            .map_or(false, |m| !m.is_excluded())
    }

    /// Return an iterator over all entries along with their path if the path matches the pathspec, or `None` if the pathspec is
    /// known to match no entry.
    // TODO: tests
    pub fn index_entries_with_paths<'s: 'repo, 'a: 'repo>(
        &'s mut self,
        index: &'a gix_index::State,
    ) -> Option<impl Iterator<Item = (&'a BStr, &'a gix_index::Entry)> + 'repo + 's> {
        index.prefixed_entries(self.search.common_prefix()).map(|entries| {
            entries.iter().filter_map(move |entry| {
                let path = entry.path(index);
                self.is_included(path, Some(false)).then_some((path, entry))
            })
        })
    }
}
