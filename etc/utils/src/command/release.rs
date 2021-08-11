#![allow(unused)]
use anyhow::{anyhow, bail};
use cargo_metadata::camino::Utf8Path;
use cargo_metadata::{
    camino::{Utf8Component, Utf8PathBuf},
    Metadata, Package, PackageId,
};
use git_repository::object::mutable::Object;
use git_repository::refs::file;
use git_repository::refs::packed;
use git_repository::{
    hash::ObjectId,
    object,
    odb::{pack, Find, FindExt},
    Repository,
};
use std::{collections::BTreeSet, convert::TryInto, path::PathBuf};

struct State {
    root: Utf8PathBuf,
    seen: BTreeSet<String>,
    repo: Repository,
    packed_refs: Option<packed::Buffer>,
}

impl State {
    fn new(repo_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = repo_path.into();
        let repo = git_repository::discover(&root)?;
        let packed_refs = repo.refs.packed()?;
        Ok(State {
            root: root.try_into()?,
            seen: BTreeSet::new(),
            repo,
            packed_refs,
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
    let version_tag_name = format!("{}-v{}", package.name, package.version);
    let mut tag_ref = match state.repo.refs.find(&version_tag_name, state.packed_refs.as_ref())? {
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
        .strip_prefix(&state.root)
        .expect("workspace members are releative to the root directory");

    let target = peel_ref_fully(&mut state.repo.refs.find_existing("HEAD", None)?, &state)?;
    let released_target = peel_ref_fully(&mut tag_ref, &state)?;

    let mut buf = Vec::new();
    log::info!("{}", repo_relative_crate_dir);

    let current_dir_id = find_directory_id_in_tree(
        repo_relative_crate_dir,
        resolve_tree_id_from_ref_target(target, &state.repo, &mut buf)?,
        &state.repo,
        &mut buf,
    )?;
    let released_tree_id = find_directory_id_in_tree(
        repo_relative_crate_dir,
        resolve_tree_id_from_ref_target(released_target, &state.repo, &mut buf)?,
        &state.repo,
        &mut buf,
    )?;

    Ok(true)
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
                    .find_existing(tree_id.take().unwrap_or_else(|| id), buf, &mut pack::cache::Never)?
                    .into_tree_iter()
                    .expect("tree");
                tree_id = tree_iter
                    .find_map(|e| {
                        let e = e.expect("tree parseable");
                        if e.filename == c {
                            Some(e.oid)
                        } else {
                            None
                        }
                    })
                    .map(ToOwned::to_owned)
                    .into();
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

    Ok(tree_id.ok_or_else(|| anyhow!("path '{}' didn't exist in tree {}", path, id))?)
}

fn peel_ref_fully(reference: &mut file::Reference<'_>, state: &State) -> anyhow::Result<ObjectId> {
    reference
        .peel_to_id_in_place(&state.repo.refs, state.packed_refs.as_ref(), |oid, buf| {
            state
                .repo
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

fn to_name(p: &PackageId) -> &str {
    p.repr
        .splitn(2, ' ')
        .next()
        .expect("crate-name <additional data we don't need>")
}
