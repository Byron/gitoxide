use std::process::Command;

use anyhow::{anyhow, bail};
use bstr::ByteSlice;
use cargo_metadata::Package;
use git_repository::{easy::object, prelude::ReferenceAccessExt};

use crate::utils::{component_to_bytes, tag_name};

pub fn has_changed_since_last_release(package: &Package, ctx: &crate::Context, verbose: bool) -> anyhow::Result<bool> {
    let version_tag_name = tag_name(package, &package.version.to_string(), &ctx.repo);
    let mut tag_ref = match ctx.repo.try_find_reference(&version_tag_name)? {
        None => {
            if verbose {
                log::info!(
                    "Package {} wasn't tagged with {} yet and thus needs a release",
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
        let err = anyhow!(bstr::BString::from(untracked));
        return Err(err.context("Found untracked files which would possibly be packaged when publishing."));
    }
    Ok(())
}
