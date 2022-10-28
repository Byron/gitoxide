use git_object::tree::EntryMode;

use crate::{bstr::ByteSlice, Id, Repository};

/// An event emitted when finding differences between two trees.
#[derive(Debug, Clone, Copy)]
pub enum Event<'old, 'new> {
    /// An entry was added, like the addition of a file or directory.
    Addition {
        /// The mode of the added entry.
        entry_mode: git_object::tree::EntryMode,
        /// The object id of the added entry.
        id: Id<'new>,
    },
    /// An entry was deleted, like the deletion of a file or directory.
    Deletion {
        /// The mode of the deleted entry.
        entry_mode: git_object::tree::EntryMode,
        /// The object id of the deleted entry.
        id: Id<'old>,
    },
    /// An entry was modified, e.g. changing the contents of a file adjusts its object id and turning
    /// a file into a symbolic link adjusts its mode.
    Modification {
        /// The mode of the entry before the modification.
        previous_entry_mode: git_object::tree::EntryMode,
        /// The object id of the entry before the modification.
        previous_id: Id<'old>,

        /// The mode of the entry after the modification.
        entry_mode: git_object::tree::EntryMode,
        /// The object id after the modification.
        id: Id<'new>,
    },
}

/// A platform to keep temporary information to perform line diffs.
pub struct DiffPlatform<'old, 'new> {
    old: crate::Object<'old>,
    new: crate::Object<'new>,
    algo: git_diff::text::Algorithm,
}

impl<'old, 'new> Event<'old, 'new> {
    fn repo(&self) -> &Repository {
        match self {
            Event::Addition { id, .. } => id.repo,
            Event::Deletion { id, .. } => id.repo,
            Event::Modification { id, .. } => id.repo,
        }
    }
}

///
pub mod event {
    ///
    pub mod diff {
        /// The error returned by [`Event::diff()`][super::Event::diff()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Could not find the previous object to diff against")]
            FindPrevious(#[from] crate::object::find::existing::Error),
            #[error("Could not obtain diff algorithm from configuration")]
            DiffAlgorithm(#[from] crate::config::diff::algorithm::Error),
        }
    }
}

impl<'old, 'new> Event<'old, 'new> {
    /// Produce a platform for performing a line-diff, or `None` if this is not a [`Modification`][Event::Modification]
    /// or one of the entries to compare is not a blob.
    pub fn diff(&self) -> Option<Result<DiffPlatform<'old, 'new>, event::diff::Error>> {
        match self {
            Event::Modification {
                previous_entry_mode: EntryMode::BlobExecutable | EntryMode::Blob,
                previous_id,
                entry_mode: EntryMode::BlobExecutable | EntryMode::Blob,
                id,
            } => match previous_id.object().and_then(|old| id.object().map(|new| (old, new))) {
                Ok((old, new)) => {
                    let algo = match self.repo().config.diff_algorithm() {
                        Ok(algo) => algo,
                        Err(err) => return Some(Err(err.into())),
                    };
                    Some(Ok(DiffPlatform { old, new, algo }))
                }
                Err(err) => Some(Err(err.into())),
            },
            _ => None,
        }
    }
}

impl<'old, 'new> DiffPlatform<'old, 'new> {
    /// Perform a diff on lines between the old and the new version of a blob.
    /// The algorithm is determined by the `diff.algorithm` configuration.
    /// Note that the [`Sink`][git_diff::text::imara::Sink] implementation is
    /// what makes the diff usable and relies heavily on what the caller requires, as created by `make_sink`.
    pub fn lines<FnS, S>(&self, new_sink: FnS) -> S::Out
    where
        FnS: for<'a> FnOnce(&git_diff::text::imara::intern::InternedInput<&'a [u8]>) -> S,
        S: git_diff::text::imara::Sink,
    {
        git_diff::text::with(
            self.old.data.as_bstr(),
            self.new.data.as_bstr(),
            self.algo,
            // TODO: make use of `core.eol` and/or filters to do line-counting correctly. It's probably
            //       OK to just know how these objects are saved to know what constitutes a line.
            git_diff::text::imara::intern::InternedInput::new,
            new_sink,
        )
        .1
    }
}
