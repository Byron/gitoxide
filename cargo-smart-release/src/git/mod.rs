use std::{convert::TryInto, process::Command};

use anyhow::{anyhow, bail};
use cargo_metadata::Package;
use git_repository as git;
use git_repository::{
    bstr::{BStr, ByteSlice},
    easy::object,
    prelude::ReferenceAccessExt,
    refs::FullNameRef,
};

use crate::utils::{component_to_bytes, tag_name};

pub mod history;

pub fn has_changed_since_last_release(package: &Package, ctx: &crate::Context, verbose: bool) -> anyhow::Result<bool> {
    let version_tag_name = tag_name(package, &package.version.to_string(), &ctx.repo);
    let mut tag_ref = match ctx.repo.try_find_reference(&version_tag_name)? {
        None => {
            if verbose {
                log::info!(
                    "Package '{}' wasn't tagged with {} yet and thus needs a release",
                    package.name,
                    version_tag_name
                );
            }
            return Ok(true);
        }
        Some(r) => r,
    };
    let repo_relative_crate_dir = ctx.repo_relative_path(package);
    Ok(match ctx.repo.head()?.into_fully_peeled_id() {
        Some(c) => {
            let current_commit = c?;
            let released_target = tag_ref.peel_to_id_in_place()?;

            match repo_relative_crate_dir {
                None => current_commit != released_target,
                Some(dir) => {
                    let components = dir.components().map(component_to_bytes);
                    let current_dir_id = current_commit
                        .object()?
                        .peel_to_kind(object::Kind::Tree)?
                        .into_tree()
                        .lookup_path(components.clone())?
                        .expect("path must exist in current commit")
                        .oid;
                    let released_dir_id = released_target
                        .object()?
                        .peel_to_kind(object::Kind::Tree)?
                        .into_tree()
                        .lookup_path(components)?
                        .expect("path must exist as it was supposedly released there")
                        .oid;

                    released_dir_id != current_dir_id
                }
            }
        }
        None => true,
    })
}

pub fn assure_clean_working_tree() -> anyhow::Result<()> {
    let tracked_changed = !Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--exit-code")
        .arg("--name-only")
        .status()?
        .success();
    if tracked_changed {
        bail!("Detected working tree changes. Please commit beforehand as otherwise these would be committed as part of manifest changes, or use --allow-dirty to force it.")
    }

    let untracked = Command::new("git")
        .arg("ls-files")
        .arg("--exclude-standard")
        .arg("--others")
        .output()?
        .stdout;
    if !untracked.trim().is_empty() {
        let err = anyhow!(git_repository::bstr::BString::from(untracked));
        return Err(err.context("Found untracked files which would possibly be packaged when publishing."));
    }
    Ok(())
}

// TODO: actually derive this from the repository by doing a lot of git-config work, better to add that to git-repository
pub fn head_remote_symbol() -> &'static str {
    "origin"
}

// TODO: use git-repository for this
pub fn remote_url() -> anyhow::Result<Option<git::Url>> {
    let output = Command::new("git")
        .arg("config")
        .arg(format!("remote.{}.url", head_remote_symbol()))
        .output()?;

    output
        .status
        .success()
        .then(|| output.stdout.as_slice().try_into().map_err(Into::into))
        .transpose()
}

pub fn author() -> anyhow::Result<git_repository::actor::Signature> {
    Ok(git_repository::actor::SignatureRef::from_bytes::<()>(
        &Command::new("git").arg("var").arg("GIT_AUTHOR_IDENT").output()?.stdout,
    )?
    .to_owned())
}

pub fn strip_tag_path(name: FullNameRef<'_>) -> &BStr {
    try_strip_tag_path(name).expect("prefix iteration works")
}

pub fn try_strip_tag_path(name: FullNameRef<'_>) -> Option<&BStr> {
    name.as_bstr().strip_prefix(b"refs/tags/").map(|b| b.as_bstr())
}
