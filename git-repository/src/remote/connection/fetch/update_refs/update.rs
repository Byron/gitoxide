use std::path::PathBuf;

use crate::remote::fetch;

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
        #[error("Failed to read or iterate worktree dir")]
        WorktreeListing(#[from] std::io::Error),
        #[error("Could not open worktree repository")]
        OpenWorktreeRepo(#[from] crate::open::Error),
        #[error("Could not find local commit for fast-forward ancestor check")]
        FindCommit(#[from] crate::object::find::existing::Error),
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
    /// No change was attempted as the remote ref didn't change compared to the current ref, or because no remote ref was specified
    /// in the ref-spec.
    NoChangeNeeded,
    /// The old ref's commit was an ancestor of the new one, allowing for a fast-forward without a merge.
    FastForward,
    /// The ref was set to point to the new commit from the remote without taking into consideration its ancestry.
    Forced,
    /// A new ref has been created as there was none before.
    New,
    /// The reference belongs to a tag that was listed by the server but whose target didn't get sent as it doesn't point
    /// to the commit-graph we were fetching explicitly.
    ///
    /// This is kind of update is only happening if `remote.<name>.tagOpt` is not set explicitly to either `--tags` or `--no-tags`.
    ImplicitTagNotSentByRemote,
    /// The object id to set the target reference to could not be found.
    RejectedSourceObjectNotFound {
        /// The id of the object that didn't exist in the object database, even though it should since it should be part of the pack.
        id: git_hash::ObjectId,
    },
    /// Tags can never be overwritten (whether the new object would be a fast-forward or not, or unchanged), unless the refspec
    /// specifies force.
    RejectedTagUpdate,
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
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::NoChangeNeeded => "up-to-date",
            Mode::FastForward => "fast-forward",
            Mode::Forced => "forced-update",
            Mode::New => "new",
            Mode::ImplicitTagNotSentByRemote => "unrelated tag on remote",
            Mode::RejectedSourceObjectNotFound { id } => return write!(f, "rejected ({} not found)", id),
            Mode::RejectedTagUpdate => "rejected (would overwrite existing tag)",
            Mode::RejectedNonFastForward => "rejected (non-fast-forward)",
            Mode::RejectedSymbolic => "rejected (refusing to write symbolic refs)",
            Mode::RejectedCurrentlyCheckedOut { worktree_dir } => {
                return write!(
                    f,
                    "rejected (cannot write into checked-out branch at \"{}\")",
                    worktree_dir.display()
                )
            }
        }
        .fmt(f)
    }
}

impl Outcome {
    /// Produce an iterator over all information used to produce the this outcome, ref-update by ref-update, using the `mappings`
    /// used when producing the ref update.
    ///
    /// Note that mappings that don't have a corresponding entry in `refspecs`  these will be `None` even though that should never be the case.
    /// This can happen if the `refspecs` passed in aren't the respecs used to create the `mapping`, and it's up to the caller to sort it out.
    pub fn iter_mapping_updates<'a, 'b>(
        &self,
        mappings: &'a [fetch::Mapping],
        refspecs: &'b [git_refspec::RefSpec],
        extra_refspecs: &'b [git_refspec::RefSpec],
    ) -> impl Iterator<
        Item = (
            &super::Update,
            &'a fetch::Mapping,
            Option<&'b git_refspec::RefSpec>,
            Option<&git_ref::transaction::RefEdit>,
        ),
    > {
        self.updates.iter().zip(mappings.iter()).map(move |(update, mapping)| {
            (
                update,
                mapping,
                mapping.spec_index.get(refspecs, extra_refspecs),
                update.edit_index.and_then(|idx| self.edits.get(idx)),
            )
        })
    }
}
