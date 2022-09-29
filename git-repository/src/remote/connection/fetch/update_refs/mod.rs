use crate::remote::fetch;
use crate::Repository;
use git_ref::transaction::{Change, LogChange, PreviousValue, RefEdit, RefLog};
use git_ref::{Target, TargetRef};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::path::PathBuf;

///
pub mod update;

/// Information about the update of a single reference, corresponding the respective entry in [`RefMap::mappings`][crate::remote::fetch::RefMap::mappings].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    /// The way the update was performed.
    pub mode: update::Mode,
    /// The index to the edit that was created from the corresponding mapping, or `None` if there was no local ref.
    pub edit_index: Option<usize>,
}

/// Update all refs as derived from `mappings` and produce an `Outcome` informing about all applied changes in detail, with each
/// [`update`][Update] corresponding to the [`fetch::Mapping`] of at the same index.
/// If `dry_run` is true, ref transactions won't actually be applied, but are assumed to work without error so the underlying
/// `repo` is not actually changed.
///
/// It can be used to produce typical information that one is used to from `git fetch`.
pub(crate) fn update(
    repo: &Repository,
    mappings: &[fetch::Mapping],
    refspecs: &[git_refspec::RefSpec],
    dry_run: fetch::DryRun,
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
        let checked_out_branches = worktree_branches(repo)?;
        let (mode, edit_index) = match local {
            Some(name) => {
                let (mode, reflog_message, name) = match repo.try_find_reference(name)? {
                    Some(existing) => {
                        if let Some(wt_dir) = checked_out_branches.get(existing.name()) {
                            updates.push(Update {
                                mode: update::Mode::RejectedCurrentlyCheckedOut {
                                    worktree_dir: wt_dir.to_owned(),
                                },
                                edit_index: None,
                            });
                            continue;
                        }
                        match existing.target() {
                            TargetRef::Symbolic(_) => {
                                updates.push(Update {
                                    mode: update::Mode::RejectedSymbolic,
                                    edit_index: None,
                                });
                                continue;
                            }
                            TargetRef::Peeled(local_id) => {
                                let (mode, reflog_message) = if local_id == remote_id {
                                    (update::Mode::NoChangeNeeded, "TBD no change")
                                } else if refspecs[*spec_index].allow_non_fast_forward() {
                                    (update::Mode::Forced, "TBD force")
                                } else {
                                    todo!("check for fast-forward (is local an ancestor of remote?)")
                                };
                                (mode, reflog_message, existing.name().to_owned())
                            }
                        }
                    }
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
        updates.push(Update { mode, edit_index })
    }

    let edits = match dry_run {
        fetch::DryRun::No => repo.edit_references(edits)?,
        fetch::DryRun::Yes => edits,
    };

    Ok(update::Outcome { edits, updates })
}

fn worktree_branches(repo: &Repository) -> Result<BTreeMap<git_ref::FullName, PathBuf>, update::Error> {
    let mut map = BTreeMap::new();
    if let Some((wt_dir, head_ref)) = repo.work_dir().zip(repo.head_ref().ok().flatten()) {
        map.insert(head_ref.inner.name, wt_dir.to_owned());
    }
    for proxy in repo.worktrees()? {
        let repo = proxy.into_repo_with_possibly_inaccessible_worktree()?;
        if let Some((wt_dir, head_ref)) = repo.work_dir().zip(repo.head_ref().ok().flatten()) {
            map.insert(head_ref.inner.name, wt_dir.to_owned());
        }
    }
    Ok(map)
}

#[cfg(test)]
mod tests;
