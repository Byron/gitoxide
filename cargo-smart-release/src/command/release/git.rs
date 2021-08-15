use super::{Context, Options};
use crate::command::release_impl::tag_name_for;
use anyhow::{anyhow, bail};
use bstr::ByteSlice;
use cargo_metadata::{
    camino::{Utf8Component, Utf8Path},
    Package,
};
use git_repository::{
    actor,
    hash::ObjectId,
    object,
    odb::{pack, Find, FindExt},
    refs::{
        file,
        file::loose::reference::peel,
        mutable::Target,
        transaction::{Change, Create, RefEdit},
    },
    Repository,
};
use std::{convert::TryInto, process::Command};

pub(in crate::command::release_impl) fn has_changed_since_last_release(
    package: &Package,
    ctx: &Context,
) -> anyhow::Result<bool> {
    let version_tag_name = tag_name_for(&package.name, &package.version.to_string());
    let mut tag_ref = match ctx.repo.refs.find(&version_tag_name, ctx.packed_refs.as_ref())? {
        None => {
            log::info!(
                "Package {} wasn't tagged with {} yet and thus needs a release",
                package.name,
                version_tag_name
            );
            return Ok(true);
        }
        Some(r) => r,
    };
    let repo_relative_crate_dir = package
        .manifest_path
        .parent()
        .expect("parent of a file is always present")
        .strip_prefix(&ctx.root)
        .expect("workspace members are releative to the root directory");

    let target = peel_ref_fully(&mut ctx.repo.refs.find_existing("HEAD", None)?, ctx)?;
    let released_target = peel_ref_fully(&mut tag_ref, ctx)?;

    if repo_relative_crate_dir.as_os_str().is_empty() {
        Ok(target != released_target)
    } else {
        let mut buf = Vec::new();

        let current_dir_id = find_directory_id_in_tree(
            repo_relative_crate_dir,
            resolve_tree_id_from_ref_target(target, &ctx.repo, &mut buf)?,
            &ctx.repo,
            &mut buf,
        )?;
        let released_dir_id = find_directory_id_in_tree(
            repo_relative_crate_dir,
            resolve_tree_id_from_ref_target(released_target, &ctx.repo, &mut buf)?,
            &ctx.repo,
            &mut buf,
        )?;

        Ok(released_dir_id != current_dir_id)
    }
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

    let has_untracked = !Command::new("git")
        .arg("ls-files")
        .arg("--exclude-standard")
        .arg("--others")
        .output()?
        .stdout
        .as_slice()
        .trim()
        .is_empty();

    if has_untracked {
        bail!("Found untracked files which would possibly be packaged when publishing.")
    }
    Ok(())
}

fn find_directory_id_in_tree(
    path: &Utf8Path,
    id: ObjectId,
    repo: &Repository,
    buf: &mut Vec<u8>,
) -> anyhow::Result<ObjectId> {
    let mut tree_id = None::<ObjectId>;

    for component in path.components() {
        match component {
            Utf8Component::Normal(c) => {
                let mut tree_iter = repo
                    .odb
                    .find_existing(tree_id.take().unwrap_or(id), buf, &mut pack::cache::Never)?
                    .into_tree_iter()
                    .expect("tree");
                tree_id = tree_iter
                    .find_map(|e| {
                        let e = e.expect("tree parseable");
                        (e.filename == c).then(|| e.oid)
                    })
                    .map(ToOwned::to_owned);
                if tree_id.is_none() {
                    break;
                }
            }
            _ => panic!(
                "only normal components are expected in relative manifest paths: '{}'",
                path
            ),
        }
    }

    tree_id.ok_or_else(|| anyhow!("path '{}' didn't exist in tree {}", path, id))
}

fn peel_ref_fully(reference: &mut file::Reference<'_>, ctx: &Context) -> anyhow::Result<ObjectId> {
    reference
        .peel_to_id_in_place(&ctx.repo.refs, ctx.packed_refs.as_ref(), |oid, buf| {
            ctx.repo
                .odb
                .find(oid, buf, &mut pack::cache::Never)
                .map(|r| r.map(|obj| (obj.kind, obj.data)))
        })
        .map_err(Into::into)
}

/// Note that borrowchk doesn't like us to return an immutable, decoded tree which we would otherwise do. Chalk/polonius could allow that,
/// preventing a duplicate lookup.
fn resolve_tree_id_from_ref_target(mut id: ObjectId, repo: &Repository, buf: &mut Vec<u8>) -> anyhow::Result<ObjectId> {
    let mut cursor = repo.odb.find_existing(id, buf, &mut pack::cache::Never)?;
    loop {
        match cursor.kind {
            object::Kind::Tree => return Ok(id),
            object::Kind::Commit => {
                id = cursor.into_commit_iter().expect("commit").tree_id().expect("id");
                cursor = repo.odb.find_existing(id, buf, &mut pack::cache::Never)?;
            }
            object::Kind::Tag | object::Kind::Blob => {
                bail!(
                    "A ref ultimately points to a blob or tag {} but we need a tree, peeling takes care of tags",
                    id
                )
            }
        }
    }
}

pub(in crate::command::release_impl) fn commit_changes(
    message: impl AsRef<str>,
    empty_commit_possible: bool,
    ctx: &Context,
) -> anyhow::Result<ObjectId> {
    // TODO: replace with gitoxide one day
    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("-am").arg(message.as_ref());
    if empty_commit_possible {
        cmd.arg("--allow-empty");
    }
    if !cmd.status()?.success() {
        bail!("Failed to commit changed manifests");
    }
    Ok(ctx
        .repo
        .refs
        .loose_find_existing("HEAD")?
        .peel_to_id_in_place(&ctx.repo.refs, ctx.packed_refs.as_ref(), peel::none)?
        .to_owned())
}

pub fn create_version_tag(
    publishee: &Package,
    new_version: &str,
    commit_id: ObjectId,
    repo: &Repository,
    Options { dry_run, skip_tag, .. }: Options,
) -> anyhow::Result<()> {
    if skip_tag {
        return Ok(());
    }
    let tag_name = tag_name_for(&publishee.name, new_version);
    if dry_run {
        log::info!("WOULD create tag {}", tag_name);
    } else {
        for tag in repo
            .refs
            .transaction()
            .prepare(
                Some(RefEdit {
                    change: Change::Update {
                        log: Default::default(),
                        mode: Create::Only,
                        new: Target::Peeled(commit_id),
                    },
                    name: format!("refs/tags/{}", tag_name).try_into()?,
                    deref: false,
                }),
                git_lock::acquire::Fail::Immediately,
            )?
            .commit(&actor::Signature::empty())?
        {
            log::info!("Created tag {}", tag.name.as_bstr());
        }
    }
    Ok(())
}
