use super::Error;
use crate::bstr::BStr;
use crate::Repository;
use git_odb::Find;
use git_ref::transaction::{LogChange, RefLog};

pub fn write_remote_to_local_config_file(
    remote: &mut crate::Remote<'_>,
    remote_name: String,
) -> Result<git_config::File<'static>, Error> {
    let mut metadata = git_config::file::Metadata::from(git_config::Source::Local);
    let config_path = remote.repo.git_dir().join("config");
    metadata.path = Some(config_path.clone());
    let mut config =
        git_config::File::from_paths_metadata(Some(metadata), Default::default())?.expect("one file to load");
    remote.save_as_to(remote_name, &mut config)?;
    std::fs::write(config_path, config.to_bstring())?;
    Ok(config)
}

pub fn replace_changed_local_config_file(repo: &mut Repository, mut config: git_config::File<'static>) {
    let repo_config = git_features::threading::OwnShared::make_mut(&mut repo.config.resolved);
    let ids_to_remove: Vec<_> = repo_config
        .sections_and_ids()
        .filter_map(|(s, id)| {
            matches!(s.meta().source, git_config::Source::Local | git_config::Source::Api).then(|| id)
        })
        .collect();
    for id in ids_to_remove {
        repo_config.remove_section_by_id(id);
    }
    crate::config::overrides::apply(&mut config, &repo.options.config_overrides, git_config::Source::Api)
        .expect("applied once and can be applied again");
    repo_config.append(config);
    repo.reread_values_and_clear_caches()
        .expect("values could be read once and can be read again");
}

/// HEAD cannot be written by means of refspec by design, so we have to do it manually here. Also create the pointed-to ref
/// if we have to, as it might not have been naturally included in the ref-specs.
pub fn update_head(
    repo: &Repository,
    remote_refs: &[git_protocol::fetch::Ref],
    reflog_message: &BStr,
) -> Result<(), Error> {
    use git_ref::transaction::{PreviousValue, RefEdit};
    use git_ref::Target;
    use std::convert::TryInto;
    let (head_peeled_id, head_ref) = match remote_refs.iter().find_map(|r| {
        Some(match r {
            git_protocol::fetch::Ref::Symbolic {
                full_ref_name,
                target,
                object,
            } if full_ref_name == "HEAD" => (Some(object), Some(target)),
            git_protocol::fetch::Ref::Direct { full_ref_name, object } if full_ref_name == "HEAD" => {
                (Some(object), None)
            }
            git_protocol::fetch::Ref::Unborn { target } => (None, Some(target)),
            _ => return None,
        })
    }) {
        Some(t) => t,
        None => return Ok(()),
    };

    let head: git_ref::FullName = "HEAD".try_into().expect("valid");
    let reflog_message = || LogChange {
        mode: RefLog::AndReference,
        force_create_reflog: false,
        message: reflog_message.to_owned(),
    };
    match head_ref {
        Some(referent) => {
            let referent: git_ref::FullName = referent.try_into().map_err(|err| Error::InvalidHeadRef {
                head_ref_name: referent.to_owned(),
                source: err,
            })?;
            repo.refs
                .transaction()
                .packed_refs(git_ref::file::transaction::PackedRefs::DeletionsAndNonSymbolicUpdates(
                    Box::new(|oid, buf| {
                        repo.objects
                            .try_find(oid, buf)
                            .map(|obj| obj.map(|obj| obj.kind))
                            .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync + 'static>)
                    }),
                ))
                .prepare(
                    {
                        let mut edits = vec![RefEdit {
                            change: git_ref::transaction::Change::Update {
                                log: reflog_message(),
                                expected: PreviousValue::Any,
                                new: Target::Symbolic(referent.clone()),
                            },
                            name: head.clone(),
                            deref: false,
                        }];
                        if let Some(head_peeled_id) = head_peeled_id {
                            edits.push(RefEdit {
                                change: git_ref::transaction::Change::Update {
                                    log: reflog_message(),
                                    expected: PreviousValue::Any,
                                    new: Target::Peeled(head_peeled_id.to_owned()),
                                },
                                name: referent,
                                deref: false,
                            });
                        };
                        edits
                    },
                    git_lock::acquire::Fail::Immediately,
                    git_lock::acquire::Fail::Immediately,
                )
                .map_err(crate::reference::edit::Error::from)?
                .commit(repo.committer_or_default())
                .map_err(crate::reference::edit::Error::from)?;

            if let Some(head_peeled_id) = head_peeled_id {
                let mut log = reflog_message();
                log.mode = RefLog::Only;
                repo.edit_reference(RefEdit {
                    change: git_ref::transaction::Change::Update {
                        log,
                        expected: PreviousValue::Any,
                        new: Target::Peeled(*head_peeled_id),
                    },
                    name: head,
                    deref: false,
                })?;
            }
        }
        None => {
            repo.edit_reference(RefEdit {
                change: git_ref::transaction::Change::Update {
                    log: reflog_message(),
                    expected: PreviousValue::Any,
                    new: Target::Peeled(
                        head_peeled_id
                            .expect("detached heads always point to something")
                            .to_owned(),
                    ),
                },
                name: head,
                deref: false,
            })?;
        }
    };
    Ok(())
}
