use crate::command::release::Options;
use anyhow::bail;
use bstr::ByteSlice;
use cargo_metadata::{camino::Utf8PathBuf, Dependency, DependencyKind, Metadata, Package};
use dia_semver::Semver;
use git_repository::{hash::ObjectId, refs::packed, Repository};
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryInto,
    path::PathBuf,
    process::Command,
    str::FromStr,
};

mod utils;
pub use utils::{
    is_workspace_member, names_and_versions, package_by_id, package_by_name, package_eq_dependency,
    package_for_dependency, tag_name_for, will, workspace_package_by_id,
};
mod git;

pub(in crate::command::release_impl) struct State {
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
pub fn release(options: Options, version_bump_spec: String, crates: Vec<String>) -> anyhow::Result<()> {
    if crates.is_empty() {
        bail!("Please provide at least one crate name which also is a workspace member");
    }
    release_depth_first(options, crates, &version_bump_spec)?;
    Ok(())
}

fn release_depth_first(options: Options, crate_names: Vec<String>, bump_spec: &str) -> anyhow::Result<()> {
    let meta = cargo_metadata::MetadataCommand::new().exec()?;
    let mut state = State::new(std::env::current_dir()?)?;
    let mut changed_crate_names_to_publish = Vec::new();
    let mut index = 0;
    for crate_name in crate_names {
        if state.seen.contains(&crate_name) {
            continue;
        }
        changed_crate_names_to_publish.push(crate_name.clone());
        while let Some(crate_name) = changed_crate_names_to_publish.get(index) {
            let package = package_by_name(&meta, crate_name)?;
            for dependency in package.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal) {
                if state.seen.contains(&dependency.name) || !is_workspace_member(&meta, &dependency.name) {
                    continue;
                }
                state.seen.insert(dependency.name.clone());
                let dep_package = package_by_name(&meta, &dependency.name).expect("exists");
                if git::has_changed_since_last_release(dep_package, &state)? {
                    changed_crate_names_to_publish.push(dependency.name.clone());
                } else {
                    log::info!(
                        "{} v{}  - skipped release as it didn't change",
                        dep_package.name,
                        dep_package.version
                    );
                }
            }
            index += 1;
        }
        state.seen.insert(crate_name);
    }
    changed_crate_names_to_publish = reorder_according_to_resolution_order(&meta, &changed_crate_names_to_publish);

    let crates_to_publish_together = {
        let mut crates_to_publish_additionally_to_avoid_instability = Vec::new();
        let mut publish_group = Vec::<String>::new();
        for publishee_name in changed_crate_names_to_publish.iter() {
            let publishee = package_by_name(&meta, publishee_name).expect("exists");
            let cycles = workspace_members_referring_to_publishee(&meta, publishee);
            if cycles.is_empty() {
                log::debug!("'{}' is cycle-free", publishee.name);
            } else {
                for Cycle { from, hops } in cycles {
                    log::warn!(
                        "'{}' links to '{}' {} causing publishes to never settle.",
                        publishee.name,
                        from.name,
                        if hops == 1 {
                            "directly".to_string()
                        } else {
                            format!("via {} hops", hops)
                        }
                    );
                    if !changed_crate_names_to_publish.contains(&from.name) {
                        crates_to_publish_additionally_to_avoid_instability.push(from.name.clone());
                    } else {
                        for name in &[&from.name, &publishee.name] {
                            if !publish_group.contains(name) {
                                publish_group.push(name.to_string())
                            }
                        }
                    }
                }
            }
        }
        if !crates_to_publish_additionally_to_avoid_instability.is_empty() && !options.ignore_instability {
            bail!(
                "Refusing to publish unless --ignore-instability is provided or crate(s) {} is/are included in the publish",
                crates_to_publish_additionally_to_avoid_instability.join(", ")
            )
        }
        reorder_according_to_resolution_order(&meta, &publish_group)
    };

    for publishee_name in changed_crate_names_to_publish
        .iter()
        .filter(|n| !crates_to_publish_together.contains(n))
    {
        let publishee = package_by_name(&meta, publishee_name).expect("exists");

        let (new_version, commit_id) = perform_single_release(&meta, publishee, options, bump_spec, &state)?;
        git::create_version_tag(publishee, &new_version, commit_id, &state.repo, options)?;
    }

    if !crates_to_publish_together.is_empty() {
        let mut crates_to_publish_together = crates_to_publish_together
            .into_iter()
            .map(|name| {
                let p = package_by_name(&meta, &name).expect("package present");
                bump_version(&p.version.to_string(), bump_spec).map(|v| (p, v.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        log::info!(
            "{} prepare releases of {}",
            will(options.dry_run),
            names_and_versions(&crates_to_publish_together)
        );

        let commit_id = edit_manifest_and_fixup_dependent_crates(
            &meta,
            &crates_to_publish_together,
            bump_spec_may_cause_empty_commits(bump_spec),
            options,
            &state,
        )?;

        crates_to_publish_together.reverse();
        while let Some((publishee, new_version)) = crates_to_publish_together.pop() {
            let unpublished_crates: Vec<_> = crates_to_publish_together
                .iter()
                .map(|(p, _)| p.name.to_owned())
                .collect();
            publish_crate(publishee, &unpublished_crates, options)?;
            git::create_version_tag(publishee, &new_version, commit_id, &state.repo, options)?;
        }
    }

    Ok(())
}

fn reorder_according_to_resolution_order(meta: &Metadata, workspace_members: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for package_in_resolve_order in meta
        .resolve
        .as_ref()
        .expect("resolve_data")
        .nodes
        .iter()
        .filter_map(|node| {
            meta.workspace_members.contains(&node.id).then(|| {
                meta.packages
                    .iter()
                    .find(|p| p.id == node.id)
                    .expect("node always present")
            })
        })
    {
        if workspace_members.contains(&package_in_resolve_order.name) {
            out.push(package_in_resolve_order.name.clone())
        }
    }
    out
}

struct Cycle<'a> {
    from: &'a Package,
    hops: usize,
}

fn workspace_members_referring_to_publishee<'a>(meta: &'a Metadata, publishee: &Package) -> Vec<Cycle<'a>> {
    publishee
        .dependencies
        .iter()
        .filter(|dep| {
            dep.kind != DependencyKind::Normal
                && meta
                    .workspace_members
                    .iter()
                    .map(|id| package_by_id(meta, id))
                    .any(|potential_cycle| package_eq_dependency(potential_cycle, dep))
        })
        .filter_map(|dep| {
            hops_for_dependency_to_link_back_to_publishee(meta, dep, publishee).map(|hops| Cycle {
                hops,
                from: package_by_name(meta, &dep.name).expect("package exists"),
            })
        })
        .collect()
}

fn hops_for_dependency_to_link_back_to_publishee<'a>(
    meta: &'a Metadata,
    source: &Dependency,
    destination: &Package,
) -> Option<usize> {
    let source = package_for_dependency(meta, source);
    let mut package_ids = vec![(0, &source.id)];
    let mut seen = BTreeSet::new();
    while let Some((level, id)) = package_ids.pop() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(package) = workspace_package_by_id(meta, id) {
            if package
                .dependencies
                .iter()
                .any(|dep| package_eq_dependency(destination, dep))
            {
                return Some(level + 1);
            }
            package_ids.extend(
                package
                    .dependencies
                    .iter()
                    .map(|dep| (level + 1, &package_for_dependency(meta, dep).id)),
            );
        };
    }
    None
}

pub fn bump_spec_may_cause_empty_commits(bump_spec: &str) -> bool {
    bump_spec == "keep"
}

fn perform_single_release(
    meta: &Metadata,
    publishee: &Package,
    options: Options,
    bump_spec: &str,
    state: &State,
) -> anyhow::Result<(String, ObjectId)> {
    let new_version = bump_version(&publishee.version.to_string(), bump_spec)?.to_string();
    log::info!(
        "{} prepare release of {} v{}",
        will(options.dry_run),
        publishee.name,
        new_version
    );
    let commit_id = edit_manifest_and_fixup_dependent_crates(
        meta,
        &[(publishee, new_version.clone())],
        bump_spec_may_cause_empty_commits(bump_spec),
        options,
        state,
    )?;
    publish_crate(publishee, &[], options)?;
    Ok((new_version, commit_id))
}

fn publish_crate(
    publishee: &Package,
    other_publishee_names: &[String],
    Options {
        skip_publish,
        dry_run,
        allow_dirty,
        no_verify,
        ..
    }: Options,
) -> anyhow::Result<()> {
    if skip_publish {
        return Ok(());
    }
    let max_attempts = 3;
    let must_not_verify = publishee
        .dependencies
        .iter()
        .any(|dep| other_publishee_names.contains(&dep.name));
    for attempt in 1..=max_attempts {
        let mut c = Command::new("cargo");
        c.arg("publish");

        if allow_dirty {
            c.arg("--allow-dirty");
        }
        if no_verify || must_not_verify {
            c.arg("--no-verify");
        }
        c.arg("--manifest-path").arg(&publishee.manifest_path);
        log::info!("{} run {:?}", will(dry_run), c);
        if dry_run || c.status()?.success() {
            break;
        } else if attempt == max_attempts {
            bail!("Could not successfully execute 'cargo publish' even ")
        } else {
            log::warn!(
                "'cargo publish' run {} failed but we retry up to {} times to rule out flakiness",
                attempt,
                max_attempts
            );
        }
    }
    Ok(())
}

fn edit_manifest_and_fixup_dependent_crates(
    meta: &Metadata,
    publishees: &[(&Package, String)],
    empty_commit_possible: bool,
    Options {
        dry_run, allow_dirty, ..
    }: Options,
    state: &State,
) -> anyhow::Result<ObjectId> {
    if !allow_dirty {
        assure_clean_working_tree()?;
    }
    let mut locks_by_manifest_path = BTreeMap::new();
    for (publishee, _) in publishees {
        let lock = git_lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_lock::acquire::Fail::Immediately,
            None,
        )?;
        locks_by_manifest_path.insert(&publishee.manifest_path, lock);
    }
    let mut packages_to_fix = Vec::new();
    for package_to_fix in meta
        .workspace_members
        .iter()
        .map(|id| package_by_id(meta, id))
        .filter(|p| {
            p.dependencies.iter().any(|dep| {
                publishees
                    .iter()
                    .any(|(publishee, _)| package_eq_dependency(publishee, dep))
            })
        })
    {
        if locks_by_manifest_path.contains_key(&package_to_fix.manifest_path) {
            continue;
        }
        let lock = git_lock::File::acquire_to_update_resource(
            &package_to_fix.manifest_path,
            git_lock::acquire::Fail::Immediately,
            None,
        )?;
        locks_by_manifest_path.insert(&package_to_fix.manifest_path, lock);
        packages_to_fix.push(package_to_fix);
    }

    for (publishee, new_version) in publishees {
        let mut lock = locks_by_manifest_path
            .get_mut(&publishee.manifest_path)
            .expect("lock available");
        set_version_and_update_package_dependency(publishee, Some(&new_version.to_string()), publishees, &mut lock)?;
    }

    for package_to_update in packages_to_fix.iter_mut() {
        let mut lock = locks_by_manifest_path
            .get_mut(&package_to_update.manifest_path)
            .expect("lock written once");
        set_version_and_update_package_dependency(package_to_update, None, publishees, &mut lock)?;
    }

    let message = format!("Release {}", names_and_versions(publishees));
    if dry_run {
        log::info!("WOULD commit changes to manifests with {:?}", message);
        Ok(ObjectId::null_sha1())
    } else {
        log::info!("Persisting changes to manifests");
        for manifest_lock in locks_by_manifest_path.into_values() {
            manifest_lock.commit()?;
        }
        refresh_cargo_lock()?;
        git::commit_changes(message, empty_commit_possible, state)
    }
}

fn refresh_cargo_lock() -> anyhow::Result<()> {
    cargo_metadata::MetadataCommand::new().exec()?;
    Ok(())
}

fn assure_clean_working_tree() -> anyhow::Result<()> {
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

fn set_version_and_update_package_dependency(
    package_to_update: &Package,
    new_version: Option<&str>,
    publishees: &[(&Package, String)],
    mut out: impl std::io::Write,
) -> anyhow::Result<()> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;

    if let Some(new_version) = new_version {
        doc["package"]["version"] = toml_edit::value(new_version);
        log::info!(
            "Pending '{}' manifest version update: \"{}\"",
            package_to_update.name,
            new_version
        );
    }
    for dep_type in &["dependencies", "dev-dependencies", "build-dependencies"] {
        for (name_to_find, new_version) in publishees.iter().map(|(p, nv)| (&p.name, nv)) {
            if let Some(name_table) = doc
                .as_table_mut()
                .get_mut(dep_type)
                .and_then(|deps| deps.as_table_mut())
                .and_then(|deps| deps.get_mut(name_to_find).and_then(|name| name.as_inline_table_mut()))
            {
                log::info!(
                    "Pending '{}' manifest {} update: '{} = \"{}\"'",
                    package_to_update.name,
                    dep_type,
                    name_to_find,
                    new_version,
                );
                *name_table.get_or_insert("version", new_version.as_str()) =
                    toml_edit::Value::from(new_version.as_str());
            }
        }
    }
    out.write_all(doc.to_string_in_original_order().as_bytes())?;

    Ok(())
}

/// TODO: Potentially just use existing semver here to avoid conversions and reduce complexity
fn bump_version(version: &str, bump_spec: &str) -> anyhow::Result<Semver> {
    let v = Semver::parse(version).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    Ok(match bump_spec {
        "major" => v.new_major(),
        "minor" => v.new_minor(),
        "patch" => v.new_patch(),
        "keep" => v.into(),
        _ => bail!("Invalid version specification: '{}'", bump_spec),
    }
    .expect("no overflow"))
}
