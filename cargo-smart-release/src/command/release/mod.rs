use crate::command::release::Options;
use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Dependency, DependencyKind, Metadata, Package};
use git_repository::{refs::packed, Repository};
use std::{collections::BTreeSet, convert::TryInto, path::PathBuf};

mod utils;
use utils::{
    bump_spec_may_cause_empty_commits, bump_version, is_dependency_with_version_requirement, is_workspace_member,
    names_and_versions, package_by_id, package_by_name, package_eq_dependency, package_for_dependency, tag_name_for,
    will, workspace_package_by_id,
};

mod cargo;
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
        if dependency_tree_has_link_to_existing_crate_names(&meta, &crate_name, &changed_crate_names_to_publish) {
            // redo all work which includes the previous tree. Could be more efficient but that would be more complicated.
            state.seen.clear();
            changed_crate_names_to_publish.clear();
            index = 0;
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
    changed_crate_names_to_publish.reverse();

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
        reorder_according_to_existing_order(&changed_crate_names_to_publish, &publish_group)
    };

    for publishee_name in changed_crate_names_to_publish
        .iter()
        .filter(|n| !crates_to_publish_together.contains(n))
    {
        let publishee = package_by_name(&meta, publishee_name).expect("exists");

        let (new_version, commit_id) = cargo::perform_single_release(&meta, publishee, options, bump_spec, &state)?;
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

        let commit_id = cargo::edit_manifest_and_fixup_dependent_crates(
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
            cargo::publish_crate(publishee, &unpublished_crates, options)?;
            git::create_version_tag(publishee, &new_version, commit_id, &state.repo, options)?;
        }
    }

    Ok(())
}

fn dependency_tree_has_link_to_existing_crate_names(
    meta: &Metadata,
    root_name: &str,
    existing_names: &[String],
) -> bool {
    let mut dependency_names = vec![root_name];
    let mut seen = BTreeSet::new();
    while let Some(crate_name) = dependency_names.pop() {
        if !seen.insert(crate_name) {
            continue;
        }
        if existing_names.iter().any(|n| n == crate_name) {
            return true;
        }
        dependency_names.extend(
            package_by_name(meta, crate_name)
                .expect("exists")
                .dependencies
                .iter()
                .filter(|dep| is_workspace_member(meta, &dep.name))
                .map(|dep| dep.name.as_str()),
        )
    }
    false
}

fn reorder_according_to_existing_order(reference_order: &[String], workspace_members: &[String]) -> Vec<String> {
    reference_order
        .iter()
        .filter(|name| workspace_members.contains(name))
        .fold(Vec::new(), |mut acc, name| {
            acc.push(name.clone());
            acc
        })
}

struct Cycle<'a> {
    from: &'a Package,
    hops: usize,
}

fn workspace_members_referring_to_publishee<'a>(meta: &'a Metadata, publishee: &Package) -> Vec<Cycle<'a>> {
    publishee
        .dependencies
        .iter()
        .filter(|dep| is_dependency_with_version_requirement(dep)) // unspecified versions don't matter for publishing
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
