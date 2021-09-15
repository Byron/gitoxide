use std::{convert::TryInto, process::Command};

use anyhow::bail;
use bstr::ByteSlice;
use cargo_metadata::Package;
use git_repository::{prelude::ReferenceAccessExt, refs, refs::transaction::PreviousValue};

use super::{tag_name, Oid, Options};
use crate::utils::will;

pub(in crate::command::release_impl) fn commit_changes(
    message: impl AsRef<str>,
    verbose: bool,
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
    if verbose {
        log::info!("{} run {:?}", will(dry_run), cmd);
    }
    if dry_run {
        return Ok(None);
    }

    if !cmd.status()?.success() {
        bail!("Failed to commit changed manifests");
    }
    Ok(Some(ctx.git_easy.find_reference("HEAD")?.peel_to_id_in_place()?))
}

pub(in crate::command::release_impl) fn create_version_tag<'repo>(
    publishee: &Package,
    new_version: &str,
    commit_id: Option<Oid<'repo>>,
    ctx: &'repo crate::Context,
    Options {
        verbose,
        dry_run,
        skip_tag,
        ..
    }: Options,
) -> anyhow::Result<Option<refs::FullName>> {
    if skip_tag {
        return Ok(None);
    }
    let tag_name = tag_name(&publishee, new_version, &ctx.git_easy);
    if dry_run {
        if verbose {
            log::info!("WOULD create tag {}", tag_name);
        }
        Ok(Some(format!("refs/tags/{}", tag_name).try_into()?))
    } else {
        let tag = ctx
            .git_easy
            .tag(tag_name, commit_id.expect("set in --execute mode"), PreviousValue::Any)?;
        log::info!("Created tag {}", tag.name().as_bstr());
        Ok(Some(tag.inner.name))
    }
}

// TODO: Make this gitoxide
pub fn push_tags_and_head(tag_names: impl IntoIterator<Item = refs::FullName>, options: Options) -> anyhow::Result<()> {
    if options.skip_push {
        return Ok(());
    }

    let mut cmd = Command::new("git");
    cmd.arg("push").arg("origin").arg("HEAD");
    for tag_name in tag_names {
        cmd.arg(tag_name.as_bstr().to_str()?);
    }

    if options.verbose {
        log::info!("{} run {:?}", will(options.dry_run), cmd);
    }
    if options.dry_run || cmd.status()?.success() {
        Ok(())
    } else {
        bail!("'git push' invocation failed. Try to push manually and repeat the smart-release invocation to resume, possibly with --skip-push.");
    }
}
