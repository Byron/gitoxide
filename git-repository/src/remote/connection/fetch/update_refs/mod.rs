use std::{collections::BTreeMap, convert::TryInto, path::PathBuf};

use git_odb::{Find, FindExt};
use git_ref::{
    transaction::{Change, LogChange, PreviousValue, RefEdit, RefLog},
    Target, TargetRef,
};

use crate::{
    ext::ObjectIdExt,
    remote::{
        fetch,
        fetch::{refs::update::Mode, RefLogMessage, Source},
    },
    Repository,
};

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

impl From<update::Mode> for Update {
    fn from(mode: Mode) -> Self {
        Update { mode, edit_index: None }
    }
}

/// Update all refs as derived from `refmap.mappings` and produce an `Outcome` informing about all applied changes in detail, with each
/// [`update`][Update] corresponding to the [`fetch::Mapping`] of at the same index.
/// If `dry_run` is true, ref transactions won't actually be applied, but are assumed to work without error so the underlying
/// `repo` is not actually changed. Also it won't perform an 'object exists' check as these are likely not to exist as the pack
/// wasn't fetched either.
/// `action` is the prefix used for reflog entries, and is typically "fetch".
///
/// It can be used to produce typical information that one is used to from `git fetch`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update(
    repo: &Repository,
    message: RefLogMessage,
    mappings: &[fetch::Mapping],
    refspecs: &[git_refspec::RefSpec],
    extra_refspecs: &[git_refspec::RefSpec],
    fetch_tags: fetch::Tags,
    dry_run: fetch::DryRun,
    write_packed_refs: fetch::WritePackedRefs,
) -> Result<update::Outcome, update::Error> {
    let mut edits = Vec::new();
    let mut updates = Vec::new();

    let implicit_tag_refspec = fetch_tags
        .to_refspec()
        .filter(|_| matches!(fetch_tags, crate::remote::fetch::Tags::Included));
    for (remote, local, spec, is_implicit_tag) in mappings.iter().filter_map(
        |fetch::Mapping {
             remote,
             local,
             spec_index,
         }| {
            spec_index.get(refspecs, extra_refspecs).map(|spec| {
                (
                    remote,
                    local,
                    spec,
                    implicit_tag_refspec.map_or(false, |tag_spec| spec.to_ref() == tag_spec),
                )
            })
        },
    ) {
        let remote_id = match remote.as_id() {
            Some(id) => id,
            None => continue,
        };
        if dry_run == fetch::DryRun::No && !repo.objects.contains(remote_id) {
            let update = if is_implicit_tag {
                update::Mode::ImplicitTagNotSentByRemote.into()
            } else {
                update::Mode::RejectedSourceObjectNotFound { id: remote_id.into() }.into()
            };
            updates.push(update);
            continue;
        }
        let checked_out_branches = worktree_branches(repo)?;
        let (mode, edit_index) = match local {
            Some(name) => {
                let (mode, reflog_message, name, previous_value) = match repo.try_find_reference(name)? {
                    Some(existing) => {
                        if let Some(wt_dir) = checked_out_branches.get(existing.name()) {
                            let mode = update::Mode::RejectedCurrentlyCheckedOut {
                                worktree_dir: wt_dir.to_owned(),
                            };
                            updates.push(mode.into());
                            continue;
                        }
                        match existing.target() {
                            TargetRef::Symbolic(_) => {
                                updates.push(update::Mode::RejectedSymbolic.into());
                                continue;
                            }
                            TargetRef::Peeled(local_id) => {
                                let previous_value =
                                    PreviousValue::MustExistAndMatch(Target::Peeled(local_id.to_owned()));
                                let (mode, reflog_message) = if local_id == remote_id {
                                    (update::Mode::NoChangeNeeded, "no update will be performed")
                                } else if let Some(git_ref::Category::Tag) = existing.name().category() {
                                    if spec.allow_non_fast_forward() {
                                        (update::Mode::Forced, "updating tag")
                                    } else {
                                        updates.push(update::Mode::RejectedTagUpdate.into());
                                        continue;
                                    }
                                } else {
                                    let mut force = spec.allow_non_fast_forward();
                                    let is_fast_forward = match dry_run {
                                        fetch::DryRun::No => {
                                            let ancestors = repo
                                                .find_object(local_id)?
                                                .try_into_commit()
                                                .map_err(|_| ())
                                                .and_then(|c| {
                                                    c.committer().map(|a| a.time.seconds_since_unix_epoch).map_err(|_| ())
                                                }).and_then(|local_commit_time|
                                                        remote_id
                                                            .to_owned()
                                                            .ancestors(|id, buf| repo.objects.find_commit_iter(id, buf))
                                                            .sorting(
                                                                git_traverse::commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan {
                                                                    time_in_seconds_since_epoch: local_commit_time
                                                                },
                                                            )
                                                            .map_err(|_| ())
                                                );
                                            match ancestors {
                                                Ok(mut ancestors) => {
                                                    ancestors.any(|cid| cid.map_or(false, |cid| cid == local_id))
                                                }
                                                Err(_) => {
                                                    force = true;
                                                    false
                                                }
                                            }
                                        }
                                        fetch::DryRun::Yes => true,
                                    };
                                    if is_fast_forward {
                                        (
                                            update::Mode::FastForward,
                                            matches!(dry_run, fetch::DryRun::Yes)
                                                .then(|| "fast-forward (guessed in dry-run)")
                                                .unwrap_or("fast-forward"),
                                        )
                                    } else if force {
                                        (update::Mode::Forced, "forced-update")
                                    } else {
                                        updates.push(update::Mode::RejectedNonFastForward.into());
                                        continue;
                                    }
                                };
                                (mode, reflog_message, existing.name().to_owned(), previous_value)
                            }
                        }
                    }
                    None => {
                        let name: git_ref::FullName = name.try_into()?;
                        let reflog_msg = match name.category() {
                            Some(git_ref::Category::Tag) => "storing tag",
                            Some(git_ref::Category::LocalBranch) => "storing head",
                            _ => "storing ref",
                        };
                        (
                            update::Mode::New,
                            reflog_msg,
                            name,
                            PreviousValue::ExistingMustMatch(Target::Peeled(remote_id.to_owned())),
                        )
                    }
                };
                let edit = RefEdit {
                    change: Change::Update {
                        log: LogChange {
                            mode: RefLog::AndReference,
                            force_create_reflog: false,
                            message: message.compose(reflog_message),
                        },
                        expected: previous_value,
                        new: if let Source::Ref(git_protocol::handshake::Ref::Symbolic { target, .. }) = &remote {
                            match mappings.iter().find_map(|m| {
                                m.remote.as_name().and_then(|name| {
                                    (name == target)
                                        .then(|| m.local.as_ref().and_then(|local| local.try_into().ok()))
                                        .flatten()
                                })
                            }) {
                                Some(local_branch) => {
                                    // This is always safe because…
                                    // - the reference may exist already
                                    // - if it doesn't exist it will be created - we are here because it's in the list of mappings after all
                                    // - if it exists and is updated, and the update is rejected due to non-fastforward for instance, the
                                    //   target reference still exists and we can point to it.
                                    Target::Symbolic(local_branch)
                                }
                                None => Target::Peeled(remote_id.into()),
                            }
                        } else {
                            Target::Peeled(remote_id.into())
                        },
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
        fetch::DryRun::No => {
            let (file_lock_fail, packed_refs_lock_fail) = repo
                .config
                .lock_timeout()
                .map_err(crate::reference::edit::Error::from)?;
            repo.refs
                .transaction()
                .packed_refs(
                    match write_packed_refs {
                        fetch::WritePackedRefs::Only => {
                            git_ref::file::transaction::PackedRefs::DeletionsAndNonSymbolicUpdatesRemoveLooseSourceReference(Box::new(|oid, buf| {
                                repo.objects
                                    .try_find(oid, buf)
                                    .map(|obj| obj.map(|obj| obj.kind))
                                    .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                            }))},
                        fetch::WritePackedRefs::Never => git_ref::file::transaction::PackedRefs::DeletionsOnly
                    }
                )
                .prepare(edits, file_lock_fail, packed_refs_lock_fail)
                .map_err(crate::reference::edit::Error::from)?
                .commit(repo.committer_or_default())
                .map_err(crate::reference::edit::Error::from)?
        }
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
