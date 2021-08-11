use anyhow::{anyhow, bail};
use cargo_metadata::{
    camino::{Utf8Component, Utf8Path, Utf8PathBuf},
    DependencyKind, Metadata, Package,
};
use dia_semver::Semver;
use git_repository::{
    hash::ObjectId,
    object,
    odb::{pack, Find, FindExt},
    refs::{file, packed},
    Repository,
};
use std::{collections::BTreeSet, convert::TryInto, path::PathBuf, str::FromStr};

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

/// In order to try dealing with https://github.com/sunng87/cargo-release/issues/224 and also to make workspace
/// releases more selective.
pub fn release(dry_run: bool, version_bump_spec: String, crates: Vec<String>) -> anyhow::Result<()> {
    if crates.is_empty() {
        bail!("Please provide at least one crate name which also is a workspace member");
    }
    let meta = cargo_metadata::MetadataCommand::new().exec()?;
    let mut state = State::new(std::env::current_dir()?)?;
    for crate_name in crates {
        if !is_workspace_member(&meta, &crate_name) {
            bail!("Package to release must be a workspace member: '{}'", crate_name);
        }
        release_depth_first(dry_run, &meta, &crate_name, &version_bump_spec, &mut state)?;
    }
    Ok(())
}

fn is_workspace_member(meta: &Metadata, crate_name: &str) -> bool {
    meta.packages
        .iter()
        .find(|p| p.name == crate_name)
        .map_or(false, |p| meta.workspace_members.iter().any(|m| m == &p.id))
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
    for dependency in package.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal) {
        if state.seen.contains(&dependency.name) || !is_workspace_member(meta, &dependency.name) {
            continue;
        }
        state.seen.insert(dependency.name.clone());
        release_depth_first(dry_run, meta, &dependency.name, bump_spec, state)?;
    }

    if needs_release(package, state)? {
        perform_release(meta, package, dry_run, bump_spec)?;
    } else {
        log::info!(
            "{} v{}  - skipped release as it didn't change",
            package.name,
            package.version
        );
    }
    Ok(())
}

fn perform_release(meta: &Metadata, package: &Package, dry_run: bool, bump_spec: &str) -> anyhow::Result<()> {
    let new_version = bump_version(&package.version.to_string(), bump_spec)?;
    log::info!("{} v{} will be released", package.name, new_version);
    edit_manifest_and_fixup_dependent_crates(meta, package, &new_version, dry_run)?;
    Ok(())
}

fn edit_manifest_and_fixup_dependent_crates(
    meta: &Metadata,
    package: &Package,
    new_version: &Semver,
    dry_run: bool,
) -> anyhow::Result<()> {
    log::trace!("Preparing {} for version update", package.manifest_path);
    let mut package_manifest_lock =
        git_lock::File::acquire_to_update_resource(&package.manifest_path, git_lock::acquire::Fail::Immediately, None)?;
    let mut packages_to_fix = meta
        .workspace_members
        .iter()
        .filter(|id| *id != &package.id)
        .map(|id| {
            meta.packages
                .iter()
                .find(|p| &p.id == id)
                .expect("workspace members are in packages")
        })
        .filter(|p| p.dependencies.iter().any(|dep| dep.name == package.name))
        .map(|p| {
            log::trace!("Preparing {} for dependency version update", p.manifest_path);
            git_lock::File::acquire_to_update_resource(&p.manifest_path, git_lock::acquire::Fail::Immediately, None)
                .map(|l| (p, l))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let new_version = new_version.to_string();
    set_manifest_version(package, &new_version, &mut package_manifest_lock)?;
    for (package_to_update, out) in packages_to_fix.iter_mut() {
        update_package_dependency(package_to_update, &package.name, &new_version, out)?;
    }

    if dry_run {
        log::info!("Won't write changed manifests in dry-run mode")
    } else {
        log::info!("Committing chnages to manifests");
        for (_, lock) in packages_to_fix {
            lock.commit()?;
        }
    }
    // Run cargo manifest to assure everything is in order
    Ok(())
}

fn update_package_dependency(
    package_to_update: &Package,
    name_to_find: &str,
    new_version: &str,
    mut out: impl std::io::Write,
) -> anyhow::Result<()> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;
    for dep_type in &["dependencies", "dev-dependencies", "build-dependencies"] {
        doc.as_table_mut()
            .get_mut(dep_type)
            .and_then(|deps| deps.as_table_mut())
            .and_then(|deps| deps.get_mut(name_to_find))
            .map(|version| {
                log::info!(
                    "Updated {} dependency in {} crate to version {}",
                    name_to_find,
                    package_to_update.name,
                    new_version
                );
                *version = toml_edit::value(new_version)
            });
    }
    out.write_all(doc.to_string_in_original_order().as_bytes())?;

    Ok(())
}

fn set_manifest_version(package: &Package, new_version: &str, mut out: impl std::io::Write) -> anyhow::Result<()> {
    let manifest = std::fs::read_to_string(&package.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;
    doc["package"]["version"] = toml_edit::value(new_version);
    log::info!("Updated {} to version {}", package.name, new_version);
    out.write_all(doc.to_string_in_original_order().as_bytes())?;
    Ok(())
}

fn bump_version(version: &str, bump_spec: &str) -> anyhow::Result<Semver> {
    let v = Semver::parse(version).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    Ok(match bump_spec {
        "major" => v.new_major(),
        "minor" => v.new_minor(),
        "patch" => v.new_patch(),
        _ => bail!("Invalid version specification: '{}'", bump_spec),
    }
    .expect("no overflow"))
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

    let target = peel_ref_fully(&mut state.repo.refs.find_existing("HEAD", None)?, state)?;
    let released_target = peel_ref_fully(&mut tag_ref, state)?;

    let mut buf = Vec::new();

    let current_dir_id = find_directory_id_in_tree(
        repo_relative_crate_dir,
        resolve_tree_id_from_ref_target(target, &state.repo, &mut buf)?,
        &state.repo,
        &mut buf,
    )?;
    let released_dir_id = find_directory_id_in_tree(
        repo_relative_crate_dir,
        resolve_tree_id_from_ref_target(released_target, &state.repo, &mut buf)?,
        &state.repo,
        &mut buf,
    )?;

    Ok(released_dir_id != current_dir_id)
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
