use crate::remote::fetch;
use git_ref::transaction::{Change, LogChange, PreviousValue, RefEdit, RefLog};
use git_ref::{Target, TargetRef};
use std::convert::TryInto;

///
pub mod update {
    use crate::remote::fetch;
    mod error {
        /// The error returned by [`fetch::refs::update()`].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            FindReference(#[from] crate::reference::find::Error),
            #[error("A remote reference had a name that wasn't considered valid. Corrupt remote repo or insufficient checks on remote?")]
            InvalidRefName(#[from] git_validate::refname::Error),
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
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        /// No change was attempted as the remote ref didn't change compared to the current ref, or because no remote ref was specified
        /// in the ref-spec.
        NoChangeNeeded,
    }

    impl Outcome {
        /// Produce an iterator over all information used to produce the this outcome, ref-update by ref-update, using the `mappings`
        /// used when producing the ref update.
        pub fn iter_mapping_updates<'a>(
            &self,
            mappings: &'a [fetch::Mapping],
        ) -> impl Iterator<
            Item = (
                &super::Update,
                &'a fetch::Mapping,
                Option<&git_ref::transaction::RefEdit>,
            ),
        > {
            self.updates
                .iter()
                .zip(mappings.iter())
                .map(move |(update, mapping)| (update, mapping, update.edit_index.and_then(|idx| self.edits.get(idx))))
        }
    }
}

/// Information about the update of a single reference, corresponding the respective entry in [`RefMap::mappings`][crate::remote::fetch::RefMap::mappings].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Update {
    /// The way the update was performed.
    pub mode: update::Mode,
    /// The index to the edit that was created from the corresponding mapping, or `None` if there was no local ref.
    pub edit_index: Option<usize>,
    /// The index of the ref-spec from which the source mapping originated.
    pub spec_index: usize,
}

/// Update all refs as derived from `mappings` and produce an `Outcome` informing about all applied changes in detail.
/// If `dry_run` is true, ref transactions won't actually be applied, but are assumed to work without error so the underlying
/// `repo` is not actually changed.
///
/// It can be used to produce typical information that one is used to from `git fetch`.
pub fn update(
    repo: &crate::Repository,
    mappings: &[fetch::Mapping],
    _dry_run: fetch::DryRun,
) -> Result<update::Outcome, update::Error> {
    let mut edits = Vec::new();
    let mut updates = Vec::new();

    for fetch::Mapping {
        remote,
        local,
        spec_index,
    } in mappings
    {
        let remote_id = remote.as_id();
        let (mode, edit_index) = match local {
            Some(name) => {
                let (mode, reflog_message, name) = match repo.try_find_reference(name)? {
                    Some(existing) => match existing.target() {
                        TargetRef::Symbolic(_) => {
                            updates.push(Update {
                                mode: update::Mode::RejectedSymbolic,
                                spec_index: *spec_index,
                                edit_index: None,
                            });
                            continue;
                        }
                        TargetRef::Peeled(local_id) => {
                            let (mode, reflog_message) = if local_id == remote_id {
                                (update::Mode::NoChangeNeeded, "TBD no change")
                            } else {
                                todo!("determine fast forward or force")
                            };
                            (mode, reflog_message, existing.name().to_owned())
                        }
                    },
                    None => (update::Mode::New, "TBD new", name.try_into()?),
                };
                let edit = RefEdit {
                    change: Change::Update {
                        log: LogChange {
                            mode: RefLog::AndReference,
                            force_create_reflog: false,
                            message: reflog_message.into(),
                        },
                        expected: PreviousValue::ExistingMustMatch(Target::Peeled(remote_id.into())),
                        new: Target::Peeled(remote_id.into()),
                    },
                    name,
                    deref: false,
                };
                let edit_index = edits.len();
                edits.push(edit);
                (mode, Some(edit_index))
            }
            None => (update::Mode::NoChangeNeeded, None),
        };
        updates.push(Update {
            mode,
            spec_index: *spec_index,
            edit_index,
        })
    }

    Ok(update::Outcome { edits, updates })
}
