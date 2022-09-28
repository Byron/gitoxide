use crate::remote::fetch;
use std::path::PathBuf;

mod error {
    /// The error returned when updating references.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        FindReference(#[from] crate::reference::find::Error),
        #[error("A remote reference had a name that wasn't considered valid. Corrupt remote repo or insufficient checks on remote?")]
        InvalidRefName(#[from] git_validate::refname::Error),
        #[error("Failed to update references to their new position to match their remote locations")]
        EditReferences(#[from] crate::reference::edit::Error),
    }
}

pub use error::Error;

/// The outcome of the refs-update operation at the end of a fetch.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// All edits that were performed to update local refs.
    pub edits: Vec<git_ref::transaction::RefEdit>,
    /// Each update provides more information about what happened to the corresponding mapping.
    /// Use [`iter_mapping_updates()`][Self::iter_mapping_updates()] to recombine the update information with ref-edits and their
    /// mapping.
    pub updates: Vec<super::Update>,
}

/// Describe the way a ref was updated
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// The old ref's commit was an ancestor of the new one, allowing for a fast-forward without a merge.
    FastForward,
    /// The ref was set to point to the new commit from the remote without taking into consideration its ancestry.
    Forced,
    /// A new ref has been created as there was none before.
    New,
    /// The reference update would not have been a fast-forward, and force is not specified in the ref-spec.
    RejectedNonFastForward,
    /// The update of a local symbolic reference was rejected.
    RejectedSymbolic,
    /// The update was rejected because the branch is checked out in the given worktree_dir.
    ///
    /// Note that the check applies to any known worktree, whether it's present on disk or not.
    RejectedCurrentlyCheckedOut {
        /// The path to the worktree directory where the branch is checked out.
        worktree_dir: PathBuf,
    },
    /// No change was attempted as the remote ref didn't change compared to the current ref, or because no remote ref was specified
    /// in the ref-spec.
    NoChangeNeeded,
}

impl Outcome {
    /// Produce an iterator over all information used to produce the this outcome, ref-update by ref-update, using the `mappings`
    /// used when producing the ref update.
    pub fn iter_mapping_updates<'a, 'b>(
        &self,
        mappings: &'a [fetch::Mapping],
        refspecs: &'b [git_refspec::RefSpec],
    ) -> impl Iterator<
        Item = (
            &super::Update,
            &'a fetch::Mapping,
            &'b git_refspec::RefSpec,
            Option<&git_ref::transaction::RefEdit>,
        ),
    > {
        self.updates.iter().zip(mappings.iter()).map(move |(update, mapping)| {
            (
                update,
                mapping,
                &refspecs[mapping.spec_index],
                update.edit_index.and_then(|idx| self.edits.get(idx)),
            )
        })
    }
}
