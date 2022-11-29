use std::{borrow::Cow, convert::TryInto};

use git_odb::Find;
use git_ref::{
    transaction::{LogChange, RefLog},
    FullNameRef,
};

use super::Error;
use crate::{
    bstr::{BStr, BString, ByteSlice},
    Repository,
};

pub fn write_remote_to_local_config_file(
    remote: &mut crate::Remote<'_>,
    remote_name: BString,
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
    crate::config::overrides::append(
        &mut config,
        &repo.options.api_config_overrides,
        git_config::Source::Api,
        |_| None,
    )
    .expect("applied once and can be applied again");
    repo_config.append(config);
    repo.reread_values_and_clear_caches()
        .expect("values could be read once and can be read again");
}

/// HEAD cannot be written by means of refspec by design, so we have to do it manually here. Also create the pointed-to ref
/// if we have to, as it might not have been naturally included in the ref-specs.
pub fn update_head(
    repo: &mut Repository,
    remote_refs: &[git_protocol::handshake::Ref],
    reflog_message: &BStr,
    remote_name: &BStr,
) -> Result<(), Error> {
    use git_ref::{
        transaction::{PreviousValue, RefEdit},
        Target,
    };
    let (head_peeled_id, head_ref) = match remote_refs.iter().find_map(|r| {
        Some(match r {
            git_protocol::handshake::Ref::Symbolic {
                full_ref_name,
                target,
                object,
            } if full_ref_name == "HEAD" => (Some(object.as_ref()), Some(target)),
            git_protocol::handshake::Ref::Direct { full_ref_name, object } if full_ref_name == "HEAD" => {
                (Some(object.as_ref()), None)
            }
            git_protocol::handshake::Ref::Unborn { full_ref_name, target } if full_ref_name == "HEAD" => {
                (None, Some(target))
            }
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
                                name: referent.clone(),
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
                        new: Target::Peeled(head_peeled_id.to_owned()),
                    },
                    name: head,
                    deref: false,
                })?;
            }

            setup_branch_config(repo, referent.as_ref(), head_peeled_id, remote_name)?;
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

/// Setup the remote configuration for `branch` so that it points to itself, but on the remote, if an only if currently saved refspec
/// is able to match it.
/// For that we reload the remote of `remote_name` and use its ref_specs for match.
fn setup_branch_config(
    repo: &mut Repository,
    branch: &FullNameRef,
    branch_id: Option<&git_hash::oid>,
    remote_name: &BStr,
) -> Result<(), Error> {
    let short_name = match branch.category_and_short_name() {
        Some((cat, shortened)) if cat == git_ref::Category::LocalBranch => match shortened.to_str() {
            Ok(s) => s,
            Err(_) => return Ok(()),
        },
        _ => return Ok(()),
    };
    let remote = repo
        .find_remote(remote_name)
        .expect("remote was just created and must be visible in config");
    let group = git_refspec::MatchGroup::from_fetch_specs(remote.fetch_specs.iter().map(|s| s.to_ref()));
    let null = git_hash::ObjectId::null(repo.object_hash());
    let res = group.match_remotes(
        Some(git_refspec::match_group::Item {
            full_ref_name: branch.as_bstr(),
            target: branch_id.unwrap_or(&null),
            object: None,
        })
        .into_iter(),
    );
    if !res.mappings.is_empty() {
        let mut metadata = git_config::file::Metadata::from(git_config::Source::Local);
        let config_path = remote.repo.git_dir().join("config");
        metadata.path = Some(config_path.clone());
        let mut config =
            git_config::File::from_paths_metadata(Some(metadata), Default::default())?.expect("one file to load");

        let mut section = config
            .new_section("branch", Some(Cow::Owned(short_name.into())))
            .expect("section header name is always valid per naming rules, our input branch name is valid");
        section.push("remote".try_into().expect("valid at compile time"), Some(remote_name));
        section.push(
            "merge".try_into().expect("valid at compile time"),
            Some(branch.as_bstr()),
        );
        std::fs::write(config_path, config.to_bstring())?;
        replace_changed_local_config_file(repo, config);
    }
    Ok(())
}
