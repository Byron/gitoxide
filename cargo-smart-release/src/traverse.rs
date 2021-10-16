use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package};

use crate::{
    git,
    utils::{is_pre_release_version, is_workspace_member, package_by_name},
};

pub mod dependencies {
    pub struct Outcome {
        pub crates_to_be_published: Vec<String>,
        pub unchanged_crates_to_skip: Vec<String>,
    }
}

pub fn dependencies(
    ctx: &crate::Context,
    verbose: bool,
    add_production_crates: bool,
) -> anyhow::Result<dependencies::Outcome> {
    let mut seen = BTreeSet::new();
    let mut changed_crate_names_to_publish = Vec::new();
    let mut skipped = Vec::new();
    for crate_name in &ctx.crate_names {
        if seen.contains(crate_name) {
            continue;
        }
        if dependency_tree_has_link_to_existing_crate_names(&ctx.meta, crate_name, &changed_crate_names_to_publish)? {
            // redo all work which includes the previous tree. Could be more efficient but that would be more complicated.
            seen.clear();
            changed_crate_names_to_publish.clear();
        }
        let num_crates_for_publishing_without_dependencies = changed_crate_names_to_publish.len();
        let package = package_by_name(&ctx.meta, crate_name)?;
        let current_skipped = depth_first_traversal(
            ctx,
            add_production_crates,
            &mut seen,
            &mut changed_crate_names_to_publish,
            package,
            verbose,
        )?;
        if !verbose && current_skipped.len() > 0 {
            log::info!(
                "Skipped {} dependent crates as they didn't change since their last release. Use --verbose/-v to see much more.",
                current_skipped.len()
            );
        }
        skipped.extend(current_skipped);
        if num_crates_for_publishing_without_dependencies == changed_crate_names_to_publish.len() {
            let crate_package = package_by_name(&ctx.meta, crate_name)?;
            if !git::has_changed_since_last_release(crate_package, ctx, verbose)? {
                log::info!(
                    "Skipping provided {} v{} hasn't changed since last released",
                    crate_package.name,
                    crate_package.version
                );
                continue;
            }
        }
        changed_crate_names_to_publish.push(crate_name.to_owned());
        seen.insert(crate_name.to_owned());
    }
    Ok(dependencies::Outcome {
        crates_to_be_published: changed_crate_names_to_publish,
        unchanged_crates_to_skip: skipped,
    })
}

fn depth_first_traversal(
    ctx: &crate::Context,
    add_production_crates: bool,
    seen: &mut BTreeSet<String>,
    changed_crate_names_to_publish: &mut Vec<String>,
    package: &Package,
    verbose: bool,
) -> anyhow::Result<Vec<String>> {
    let mut skipped = Vec::new();
    for dependency in package.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal) {
        if seen.contains(&dependency.name) || !is_workspace_member(&ctx.meta, &dependency.name) {
            continue;
        }
        seen.insert(dependency.name.clone());
        let dep_package = package_by_name(&ctx.meta, &dependency.name)?;
        skipped.extend(depth_first_traversal(
            ctx,
            add_production_crates,
            seen,
            changed_crate_names_to_publish,
            dep_package,
            verbose,
        )?);
        if git::has_changed_since_last_release(dep_package, ctx, verbose)? {
            if is_pre_release_version(&dep_package.version) || add_production_crates {
                if verbose {
                    log::info!(
                        "Adding '{}' v{} to set of published crates as it changed since last release",
                        dep_package.name,
                        dep_package.version
                    );
                }
                changed_crate_names_to_publish.push(dependency.name.clone());
            } else {
                log::warn!(
                    "'{}' v{} changed since last release - consider releasing it beforehand.",
                    dep_package.name,
                    dep_package.version
                );
            }
        } else {
            if verbose {
                log::info!(
                    "'{}' v{}  - skipped release as it didn't change",
                    dep_package.name,
                    dep_package.version
                );
            }
            skipped.push(dep_package.name.clone());
        }
    }
    Ok(skipped)
}

fn dependency_tree_has_link_to_existing_crate_names(
    meta: &Metadata,
    root_name: &str,
    existing_names: &[String],
) -> anyhow::Result<bool> {
    let mut dependency_names = vec![root_name];
    let mut seen = BTreeSet::new();
    while let Some(crate_name) = dependency_names.pop() {
        if !seen.insert(crate_name) {
            continue;
        }
        if existing_names.iter().any(|n| n == crate_name) {
            return Ok(true);
        }
        dependency_names.extend(
            package_by_name(meta, crate_name)?
                .dependencies
                .iter()
                .filter(|dep| is_workspace_member(meta, &dep.name))
                .map(|dep| dep.name.as_str()),
        )
    }
    Ok(false)
}
