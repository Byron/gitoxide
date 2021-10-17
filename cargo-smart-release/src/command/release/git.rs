use std::{convert::TryInto, process::Command};

use anyhow::bail;
use cargo_metadata::Package;
use git_repository::{bstr::ByteSlice, prelude::ReferenceAccessExt, refs, refs::transaction::PreviousValue};

use super::{tag_name, Oid, Options};
use crate::utils::will;

pub(in crate::command::release_impl) fn commit_changes(
    message: impl AsRef<str>,
    dry_run: bool,
    empty_commit_possible: bool,
    ctx: &crate::Context,
) -> anyhow::Result<Option<Oid<'_>>> {
    // TODO: replace with gitoxide one day
    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("-am").arg(message.as_ref());
    if empty_commit_possible {
        cmd.arg("--allow-empty");
    }
    log::trace!("{} run {:?}", will(dry_run), cmd);
    if dry_run {
        return Ok(None);
    }

    if !cmd.status()?.success() {
        bail!("Failed to commit changed manifests");
    }
    Ok(Some(ctx.repo.find_reference("HEAD")?.peel_to_id_in_place()?))
}

pub(in crate::command::release_impl) fn create_version_tag<'repo>(
    publishee: &Package,
    new_version: &str,
    commit_id: Option<Oid<'repo>>,
    tag_message: Option<String>,
    ctx: &'repo crate::Context,
    Options { dry_run, skip_tag, .. }: Options,
) -> anyhow::Result<Option<refs::FullName>> {
    if skip_tag {
        return Ok(None);
    }
    let tag_name = tag_name(publishee, new_version, &ctx.repo);
    if dry_run {
        match tag_message {
            Some(message) => {
                log::trace!(
                    "WOULD create tag object {} with changelog message, first line is: '{}'",
                    tag_name,
                    message.lines().next().unwrap_or("")
                );
            }
            None => {
                log::trace!("WOULD create tag {}", tag_name);
            }
        }
        Ok(Some(format!("refs/tags/{}", tag_name).try_into()?))
    } else {
        let target = commit_id.expect("set in --execute mode");
        let constraint = PreviousValue::Any;
        let tag = match tag_message {
            Some(message) => {
                let tag = git_repository::prelude::ObjectAccessExt::tag(
                    &ctx.repo,
                    tag_name,
                    target,
                    git_repository::objs::Kind::Commit,
                    Some(&crate::git::author()?.to_ref()),
                    message,
                    constraint,
                )?;
                log::info!("Created tag object {} with release notes.", tag.name().as_bstr());
                tag
            }
            None => {
                let tag = ctx.repo.tag_reference(tag_name, target, constraint)?;
                log::info!("Created tag {}", tag.name().as_bstr());
                tag
            }
        };
        Ok(Some(tag.inner.name))
    }
}

// TODO: Use gitoxide here
pub fn push_tags_and_head<'a>(
    tag_names: impl IntoIterator<Item = &'a refs::FullName>,
    Options { dry_run, skip_push, .. }: Options,
) -> anyhow::Result<()> {
    if skip_push {
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.arg("push").arg(crate::git::head_remote_symbol()).arg("HEAD");
    for tag_name in tag_names {
        cmd.arg(tag_name.as_bstr().to_str()?);
    }

    log::trace!("{} run {:?}", will(dry_run), cmd);
    if dry_run || cmd.status()?.success() {
        Ok(())
    } else {
        bail!("'git push' invocation failed. Try to push manually and repeat the smart-release invocation to resume, possibly with --skip-push.");
    }
}
