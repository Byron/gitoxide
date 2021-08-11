#![allow(unused)]
use anyhow::{anyhow, bail};
use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package, PackageId};
use git_repository::{
    hash::ObjectId,
    object,
    object::immutable,
    odb::{pack, Find, FindExt},
    Repository,
};
use std::{collections::BTreeSet, convert::TryInto, path::PathBuf};

struct State {
    root: Utf8PathBuf,
    seen: BTreeSet<String>,
    repo: Repository,
}

impl State {
    fn new(repo_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = repo_path.into();
        let repo = git_repository::discover(&root)?;
        Ok(State {
            root: root.try_into()?,
            seen: BTreeSet::new(),
            repo,
        })
    }
}

pub fn release(dry_run: bool, version_bump_spec: String, crates: Vec<String>) -> anyhow::Result<()> {
    let meta = cargo_metadata::MetadataCommand::new().exec()?;
    let mut state = State::new(std::env::current_dir()?)?;
    for crate_name in crates {
        if !meta.workspace_members.iter().any(|p| to_name(p) == crate_name) {
            bail!("Package to release must be a workspace member: '{}'", crate_name);
        }
        release_depth_first(dry_run, &meta, &crate_name, &version_bump_spec, &mut state)?;
    }
    Ok(())
}

fn release_depth_first(
    dry_run: bool,
    meta: &Metadata,
    crate_name: &str,
    bump_spec: &str,
    state: &mut State,
) -> anyhow::Result<()> {
    let package = meta
        .packages
        .iter()
        .find(|p| p.name == crate_name)
        .ok_or_else(|| anyhow!("workspace member must be a listed package: '{}'", crate_name))?;
    for dependency in &package.dependencies {
        if state.seen.contains(&dependency.name)
            || !meta.workspace_members.iter().any(|p| to_name(p) == dependency.name)
        {
            continue;
        }
        state.seen.insert(dependency.name.clone());
        release_depth_first(dry_run, meta, &dependency.name, bump_spec, state)?;
    }

    if needs_release(package, state)? {
        log::info!("{} needs a release", crate_name);
    }
    Ok(())
}

fn needs_release(package: &Package, state: &State) -> anyhow::Result<bool> {
    let repo_relative_crate_dir = package
        .manifest_path
        .parent()
        .expect("parent of a file is always present")
        .strip_prefix(&state.root)
        .expect("workspace members are releative to the root directory");
    let target = state.repo.refs.find_existing("HEAD", None)?.peel_to_id_in_place(
        &state.repo.refs,
        state.repo.refs.packed()?.as_ref(),
        |oid, buf| {
            state
                .repo
                .odb
                .find(oid, buf, &mut pack::cache::Never)
                .map(|r| r.map(|obj| (obj.kind, obj.data)))
        },
    )?;
    let mut buf = Vec::new();
    log::info!("{}", repo_relative_crate_dir);
    let tree_id = resolve_tree_id_from_id(target, &state.repo, &mut buf)?;
    Ok(true)
}

/// Note that borrowchk doesn't like us to return an immutable, decoded tree which we would otherwise do. Chalk/polonius could allow that,
/// preventing a duplicate lookup.
fn resolve_tree_id_from_id(mut id: ObjectId, repo: &Repository, buf: &mut Vec<u8>) -> anyhow::Result<ObjectId> {
    let mut cursor = repo.odb.find_existing(id, buf, &mut pack::cache::Never)?;
    loop {
        match cursor.kind {
            object::Kind::Tree => return Ok(id),
            object::Kind::Commit => {
                id = cursor.into_commit_iter().expect("commit").tree_id().expect("id");
                cursor = repo.odb.find_existing(id, buf, &mut pack::cache::Never)?;
                continue;
            }
            _ => todo!("other cases"),
        }
    }
}

fn to_name(p: &PackageId) -> &str {
    p.repr
        .splitn(2, ' ')
        .next()
        .expect("crate-name <additional data we don't need>")
}
