use std::collections::BTreeSet;

use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Dependency, DependencyKind, Metadata, Package};

use crate::command::release::Options;

mod utils;
use crates_index::Index;
use utils::{
    is_dependency_with_version_requirement, is_workspace_member, names_and_versions, package_by_id, package_by_name,
    package_eq_dependency, package_for_dependency, tag_name_for, will, workspace_package_by_id,
};

mod cargo;
mod git;
mod manifest;
mod version;

type Oid<'repo> = git_repository::easy::Oid<'repo, git_repository::Easy>;

pub(crate) struct Context {
    root: Utf8PathBuf,
    meta: Metadata,
    git_easy: git_repository::Easy,
    crates_index: Index,
    crate_names: Vec<String>,
    bump: String,
    bump_dependencies: String,
}

impl Context {
    fn new(crate_names: Vec<String>, bump: String, bump_dependencies: String) -> anyhow::Result<Self> {
        let meta = cargo_metadata::MetadataCommand::new().exec()?;
        let root = meta.workspace_root.clone();
        let repo = git_repository::discover(&root)?;
        let index = Index::new_cargo_default();
        Ok(Context {
            root,
            git_easy: repo.into(),
            meta,
            crates_index: index,
            crate_names,
            bump,
            bump_dependencies,
        })
    }
}

/// In order to try dealing with https://github.com/sunng87/cargo-release/issues/224 and also to make workspace
/// releases more selective.
pub fn release(options: Options, crates: Vec<String>, bump: String, bump_dependencies: String) -> anyhow::Result<()> {
    if options.dry_run_cargo_publish && !options.dry_run {
        bail!("The --no-dry-run-cargo-publish flag is only effective without --execute")
    }
    let mut ctx = Context::new(crates, bump, bump_dependencies)?;
    if options.update_crates_index {
        log::info!("Updating crates-io index at '{}'", ctx.crates_index.path().display());
        ctx.crates_index.update()?;
    } else if options.bump_when_needed {
        log::warn!(
            "Consider running with --update-crates-index to assure bumping on demand uses the latest information"
        );
    }
    if !ctx.crates_index.exists() {
        log::warn!("Crates.io index doesn't exist. Consider using --update-crates-index to help determining if release versions are published already");
    }

    if ctx.crate_names.is_empty() {
        let current_dir = std::env::current_dir()?;
        let manifest = current_dir.join("Cargo.toml");
        let dir_name = current_dir
            .file_name()
            .expect("a valid directory with a name")
            .to_str()
            .expect("directory is UTF8 representable");
        let crate_name = if manifest.is_file() {
            let manifest = cargo_toml::Manifest::from_path(manifest)?;
            manifest.package.map_or(dir_name.to_owned(), |p| p.name)
        } else {
            dir_name.to_owned()
        };
        log::warn!(
            "Using '{}' as crate name as no one was provided. Specify one if this isn't correct",
            crate_name
        );
        ctx.crate_names = vec![crate_name];
    }
    release_depth_first(ctx, options)?;
    Ok(())
}

fn release_depth_first(ctx: Context, options: Options) -> anyhow::Result<()> {
    let meta = &ctx.meta;
    let changed_crate_names_to_publish = if options.skip_dependencies {
        ctx.crate_names.clone()
    } else {
        traverse_dependencies_and_find_changed_crates(meta, &ctx.crate_names, &ctx, options)?
    };

    let crates_to_publish_together = resolve_cycles_with_publish_group(meta, &changed_crate_names_to_publish, options)?;

    assure_working_tree_is_unchanged(options)?;

    if options.multi_crate_release && !changed_crate_names_to_publish.is_empty() {
        perforrm_multi_version_release(&ctx, options, meta, changed_crate_names_to_publish)?;
    } else {
        for publishee_name in changed_crate_names_to_publish
            .iter()
            .filter(|n| !crates_to_publish_together.contains(n))
        {
            let publishee = package_by_name(meta, publishee_name)?;

            let (new_version, commit_id) = perform_single_release(meta, publishee, options, &ctx)?;
            let tag_name = git::create_version_tag(publishee, &new_version, commit_id, &ctx, options)?;
            git::push_tags_and_head(tag_name, options)?;
        }
    }

    if !crates_to_publish_together.is_empty() {
        perforrm_multi_version_release(&ctx, options, meta, crates_to_publish_together)?;
    }

    Ok(())
}

fn assure_working_tree_is_unchanged(options: Options) -> anyhow::Result<()> {
    if !options.allow_dirty {
        if let Err(err) = git::assure_clean_working_tree() {
            if options.dry_run {
                log::warn!("The working tree has changes which will prevent a release with --execute unless --allow-dirty is also specified. The latter isn't recommended.")
            } else {
                return Err(err);
            }
        }
    }
    Ok(())
}

fn perforrm_multi_version_release(
    ctx: &Context,
    options: Options,
    meta: &Metadata,
    crates_to_publish_together: Vec<String>,
) -> anyhow::Result<()> {
    let mut crates_to_publish_together = crates_to_publish_together
        .into_iter()
        .map(|name| {
            let p = package_by_name(meta, &name)?;
            version::bump(
                p,
                version::select_publishee_bump_spec(&p.name, ctx),
                ctx,
                options.bump_when_needed,
            )
            .map(|v| (p, v.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    log::info!(
        "{} prepare releases of {}",
        will(options.dry_run),
        names_and_versions(&crates_to_publish_together)
    );

    let commit_id = manifest::edit_version_and_fixup_dependent_crates(meta, &crates_to_publish_together, options, ctx)?;

    crates_to_publish_together.reverse();
    let mut tag_names = Vec::new();
    while let Some((publishee, new_version)) = crates_to_publish_together.pop() {
        let unpublished_crates: Vec<_> = crates_to_publish_together
            .iter()
            .map(|(p, _)| p.name.to_owned())
            .collect();

        cargo::publish_crate(publishee, &unpublished_crates, options)?;
        if let Some(tag_name) = git::create_version_tag(publishee, &new_version, commit_id.clone(), ctx, options)? {
            tag_names.push(tag_name);
        };
    }
    git::push_tags_and_head(tag_names, options)?;
    Ok(())
}

fn resolve_cycles_with_publish_group(
    meta: &Metadata,
    changed_crate_names_to_publish: &[String],
    options: Options,
) -> anyhow::Result<Vec<String>> {
    let mut crates_to_publish_additionally_to_avoid_instability = Vec::new();
    let mut publish_group = Vec::<String>::new();
    for publishee_name in changed_crate_names_to_publish.iter() {
        let publishee = package_by_name(meta, publishee_name)?;
        let cycles = workspace_members_referring_to_publishee(meta, publishee);
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
            "Refusing to publish unless --ignore-instability is provided or crate(s) {} is/are included in the publish. To avoid this, don't specify versions in your dev dependencies.",
            crates_to_publish_additionally_to_avoid_instability.join(", ")
        )
    }
    Ok(reorder_according_to_existing_order(
        changed_crate_names_to_publish,
        &publish_group,
    ))
}

fn traverse_dependencies_and_find_changed_crates(
    meta: &Metadata,
    crate_names: &[String],
    ctx: &Context,
    Options {
        verbose,
        allow_auto_publish_of_stable_crates,
        ..
    }: Options,
) -> anyhow::Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut changed_crate_names_to_publish = Vec::new();
    for crate_name in crate_names {
        if seen.contains(crate_name) {
            continue;
        }
        if dependency_tree_has_link_to_existing_crate_names(meta, crate_name, &changed_crate_names_to_publish)? {
            // redo all work which includes the previous tree. Could be more efficient but that would be more complicated.
            seen.clear();
            changed_crate_names_to_publish.clear();
        }
        let num_crates_for_publishing_without_dependencies = changed_crate_names_to_publish.len();
        let package = package_by_name(meta, crate_name)?;
        let skipped = depth_first_traversal(
            meta,
            ctx,
            allow_auto_publish_of_stable_crates,
            &mut seen,
            &mut changed_crate_names_to_publish,
            package,
            verbose,
        )?;
        if !verbose && skipped > 0 {
            log::info!(
                "Skipped {} dependent crates as they didn't change since their last release. Use --verbose/-v to see much more.",
                skipped
            );
        }
        if num_crates_for_publishing_without_dependencies == changed_crate_names_to_publish.len() {
            let crate_package = package_by_name(meta, crate_name)?;
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
    Ok(changed_crate_names_to_publish)
}

fn depth_first_traversal(
    meta: &Metadata,
    ctx: &Context,
    allow_auto_publish_of_stable_crates: bool,
    seen: &mut BTreeSet<String>,
    changed_crate_names_to_publish: &mut Vec<String>,
    package: &Package,
    verbose: bool,
) -> anyhow::Result<usize> {
    let mut skipped = 0;
    for dependency in package.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal) {
        if seen.contains(&dependency.name) || !is_workspace_member(meta, &dependency.name) {
            continue;
        }
        seen.insert(dependency.name.clone());
        let dep_package = package_by_name(meta, &dependency.name)?;
        skipped += depth_first_traversal(
            meta,
            ctx,
            allow_auto_publish_of_stable_crates,
            seen,
            changed_crate_names_to_publish,
            dep_package,
            verbose,
        )?;
        if git::has_changed_since_last_release(dep_package, ctx, verbose)? {
            if version::is_pre_release(&dep_package.version) || allow_auto_publish_of_stable_crates {
                if verbose {
                    log::info!(
                        "Adding {} v{} to set of published crates as it changed since last release",
                        dep_package.name,
                        dep_package.version
                    );
                }
                changed_crate_names_to_publish.push(dependency.name.clone());
            } else {
                log::warn!(
                    "{} v{} changed since last release - consider releasing it beforehand.",
                    dep_package.name,
                    dep_package.version
                );
            }
        } else {
            if verbose {
                log::info!(
                    "{} v{}  - skipped release as it didn't change",
                    dep_package.name,
                    dep_package.version
                );
            }
            skipped += 1;
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

fn reorder_according_to_existing_order(reference_order: &[String], names_to_order: &[String]) -> Vec<String> {
    let new_order = reference_order
        .iter()
        .filter(|name| names_to_order.contains(name))
        .fold(Vec::new(), |mut acc, name| {
            acc.push(name.clone());
            acc
        });
    assert_eq!(
        new_order.len(),
        names_to_order.len(),
        "the reference order must contain all items to be ordered"
    );
    new_order
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

fn perform_single_release<'repo>(
    meta: &Metadata,
    publishee: &Package,
    options: Options,
    ctx: &'repo Context,
) -> anyhow::Result<(String, Option<Oid<'repo>>)> {
    let bump_spec = version::select_publishee_bump_spec(&publishee.name, ctx);
    let new_version = version::bump(publishee, bump_spec, ctx, options.bump_when_needed)?;
    log::info!(
        "{} prepare release of {} v{}",
        will(options.dry_run),
        publishee.name,
        new_version
    );
    let new_version = new_version.to_string();
    let commit_id =
        manifest::edit_version_and_fixup_dependent_crates(meta, &[(publishee, new_version.clone())], options, ctx)?;
    cargo::publish_crate(publishee, &[], options)?;
    Ok((new_version, commit_id))
}
