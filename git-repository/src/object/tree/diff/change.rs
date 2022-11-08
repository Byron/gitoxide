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

/// A platform to keep temporary information to perform line diffs on modified blobs.
///
pub struct DiffPlatform<'old, 'new> {
    /// The previous version of the blob.
    pub old: crate::Object<'old>,
    /// The new version of the blob.
    pub new: crate::Object<'new>,
    /// The algorithm to use when calling [imara_diff::diff()][git_diff::blob::diff()].
    /// This value is determined by the `diff.algorithm` configuration.
    pub algo: git_diff::blob::Algorithm,
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
        /// The error returned by [`Event::diff()`][super::super::Event::diff()].
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
    /// Perform a diff on lines between the old and the new version of a blob, passing each hunk of lines to `process_hunk`.
    /// The diffing algorithm is determined by the `diff.algorithm` configuration.
    ///
    /// Note that you can invoke the diff more flexibly as well.
    pub fn lines<FnH, S>(&self, _process_hunk: FnH)
    where
        FnH: for<'a> FnOnce(&git_diff::blob::intern::InternedInput<&'a [u8]>) -> S,
    {
        let _intern = self.line_tokens();
        // git_diff::blob::diff(self.algo, &intern);
        todo!()
    }

    /// Count the amount of removed and inserted lines efficiently.
    pub fn line_counts(&self) -> git_diff::blob::sink::Counter<()> {
        let tokens = self.line_tokens();
        git_diff::blob::diff(self.algo, &tokens, git_diff::blob::sink::Counter::default())
    }

    /// Return a tokenizer which treats lines as smallest unit.
    ///
    /// The line separator is determined according to normal git rules and filters.
    pub fn line_tokens(&self) -> git_diff::blob::intern::InternedInput<&[u8]> {
        // TODO: make use of `core.eol` and/or filters to do line-counting correctly. It's probably
        //       OK to just know how these objects are saved to know what constitutes a line.
        git_diff::blob::intern::InternedInput::new(self.old.data.as_bytes(), self.new.data.as_bytes())
    }
}
