use std::{borrow::Cow, convert::TryInto, io::Write};

use gix_odb::Find;
use gix_ref::{
    transaction::{LogChange, RefLog},
    FullNameRef,
};

use super::Error;
use crate::{
    bstr::{BStr, BString, ByteSlice},
    Repository,
};

enum WriteMode {
    Overwrite,
    Append,
}

#[allow(clippy::result_large_err)]
pub fn write_remote_to_local_config_file(
    remote: &mut crate::Remote<'_>,
    remote_name: BString,
) -> Result<gix_config::File<'static>, Error> {
    let mut config = gix_config::File::new(local_config_meta(remote.repo));
    remote.save_as_to(remote_name, &mut config)?;

    write_to_local_config(&config, WriteMode::Append)?;
    Ok(config)
}

fn local_config_meta(repo: &Repository) -> gix_config::file::Metadata {
    let meta = repo.config.resolved.meta().clone();
    assert_eq!(
        meta.source,
        gix_config::Source::Local,
        "local path is the default for new sections"
    );
    meta
}

fn write_to_local_config(config: &gix_config::File<'static>, mode: WriteMode) -> std::io::Result<()> {
    assert_eq!(
        config.meta().source,
        gix_config::Source::Local,
        "made for appending to local configuration file"
    );
    let mut local_config = std::fs::OpenOptions::new()
        .create(false)
        .write(matches!(mode, WriteMode::Overwrite))
        .append(matches!(mode, WriteMode::Append))
        .open(config.meta().path.as_deref().expect("local config with path set"))?;
    local_config.write_all(config.detect_newline_style())?;
    config.write_to_filter(&mut local_config, &mut |s| s.meta().source == gix_config::Source::Local)
}

pub fn append_config_to_repo_config(repo: &mut Repository, config: gix_config::File<'static>) {
    let repo_config = gix_features::threading::OwnShared::make_mut(&mut repo.config.resolved);
    repo_config.append(config);
}

/// HEAD cannot be written by means of refspec by design, so we have to do it manually here. Also create the pointed-to ref
/// if we have to, as it might not have been naturally included in the ref-specs.
pub fn update_head(
    repo: &mut Repository,
    remote_refs: &[gix_protocol::handshake::Ref],
    reflog_message: &BStr,
    remote_name: &BStr,
) -> Result<(), Error> {
    use gix_ref::{
        transaction::{PreviousValue, RefEdit},
        Target,
    };
    let (head_peeled_id, head_ref) = match remote_refs.iter().find_map(|r| {
        Some(match r {
            gix_protocol::handshake::Ref::Symbolic {
                full_ref_name,
                target,
                tag: _,
                object,
            } if full_ref_name == "HEAD" => (Some(object.as_ref()), Some(target)),
            gix_protocol::handshake::Ref::Direct { full_ref_name, object } if full_ref_name == "HEAD" => {
                (Some(object.as_ref()), None)
            }
            gix_protocol::handshake::Ref::Unborn { full_ref_name, target } if full_ref_name == "HEAD" => {
                (None, Some(target))
            }
            _ => return None,
        })
    }) {
        Some(t) => t,
        None => return Ok(()),
    };

    let head: gix_ref::FullName = "HEAD".try_into().expect("valid");
    let reflog_message = || LogChange {
        mode: RefLog::AndReference,
        force_create_reflog: false,
        message: reflog_message.to_owned(),
    };
    match head_ref {
        Some(referent) => {
            let referent: gix_ref::FullName = referent.try_into().map_err(|err| Error::InvalidHeadRef {
                head_ref_name: referent.to_owned(),
                source: err,
            })?;
            repo.refs
                .transaction()
                .packed_refs(gix_ref::file::transaction::PackedRefs::DeletionsAndNonSymbolicUpdates(
                    Box::new(|oid, buf| repo.objects.try_find(&oid, buf).map(|obj| obj.map(|obj| obj.kind))),
                ))
                .prepare(
                    {
                        let mut edits = vec![RefEdit {
                            change: gix_ref::transaction::Change::Update {
                                log: reflog_message(),
                                expected: PreviousValue::Any,
                                new: Target::Symbolic(referent.clone()),
                            },
                            name: head.clone(),
                            deref: false,
                        }];
                        if let Some(head_peeled_id) = head_peeled_id {
                            edits.push(RefEdit {
                                change: gix_ref::transaction::Change::Update {
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
                    gix_lock::acquire::Fail::Immediately,
                    gix_lock::acquire::Fail::Immediately,
                )
                .map_err(crate::reference::edit::Error::from)?
                .commit(
                    repo.committer()
                        .transpose()
                        .map_err(|err| Error::HeadUpdate(crate::reference::edit::Error::ParseCommitterTime(err)))?,
                )
                .map_err(crate::reference::edit::Error::from)?;

            if let Some(head_peeled_id) = head_peeled_id {
                let mut log = reflog_message();
                log.mode = RefLog::Only;
                repo.edit_reference(RefEdit {
                    change: gix_ref::transaction::Change::Update {
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
                change: gix_ref::transaction::Change::Update {
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

/// Setup the remote configuration for `branch` so that it points to itself, but on the remote, if and only if currently
/// saved refspecs are able to match it.
/// For that we reload the remote of `remote_name` and use its `ref_specs` for match.
fn setup_branch_config(
    repo: &mut Repository,
    branch: &FullNameRef,
    branch_id: Option<&gix_hash::oid>,
    remote_name: &BStr,
) -> Result<(), Error> {
    let short_name = match branch.category_and_short_name() {
        Some((gix_ref::Category::LocalBranch, shortened)) => match shortened.to_str() {
            Ok(s) => s,
            Err(_) => return Ok(()),
        },
        _ => return Ok(()),
    };
    let remote = repo
        .find_remote(remote_name)
        .expect("remote was just created and must be visible in config");
    let group = gix_refspec::MatchGroup::from_fetch_specs(remote.fetch_specs.iter().map(gix_refspec::RefSpec::to_ref));
    let null = gix_hash::ObjectId::null(repo.object_hash());
    let res = group.match_remotes(
        Some(gix_refspec::match_group::Item {
            full_ref_name: branch.as_bstr(),
            target: branch_id.unwrap_or(&null),
            object: None,
        })
        .into_iter(),
    );
    if !res.mappings.is_empty() {
        let mut config = repo.config_snapshot_mut();
        let mut section = config
            .new_section("branch", Some(Cow::Owned(short_name.into())))
            .expect("section header name is always valid per naming rules, our input branch name is valid");
        section.push("remote".try_into().expect("valid at compile time"), Some(remote_name));
        section.push(
            "merge".try_into().expect("valid at compile time"),
            Some(branch.as_bstr()),
        );
        write_to_local_config(&config, WriteMode::Overwrite)?;
        config.commit().expect("configuration we set is valid");
    }
    Ok(())
}
