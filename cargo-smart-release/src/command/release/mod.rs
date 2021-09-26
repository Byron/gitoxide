use std::collections::BTreeSet;

use anyhow::bail;
use cargo_metadata::{Dependency, DependencyKind, Metadata, Package};
use crates_index::Index;

use crate::{
    command::release::Options,
    utils::{
        is_dependency_with_version_requirement, names_and_versions, package_by_id, package_by_name,
        package_eq_dependency, package_for_dependency, tag_name, will, workspace_package_by_id,
    },
};

mod cargo;
mod git;
mod manifest;
mod version;

type Oid<'repo> = git_repository::easy::Oid<'repo, git_repository::Easy>;

pub(crate) struct Context {
    base: crate::Context,
    crates_index: Index,
    history: Option<crate::commit::History>,
    bump: String,
    bump_dependencies: String,
}

impl Context {
    fn new(crate_names: Vec<String>, bump: String, bump_dependencies: String, changelog: bool) -> anyhow::Result<Self> {
        let crates_index = Index::new_cargo_default();
        let base = crate::Context::new(crate_names)?;
        let history = changelog
            .then(|| crate::git::history::collect(&base.repo))
            .transpose()?
            .flatten();
        Ok(Context {
            base,
            history,
            crates_index,
            bump,
            bump_dependencies,
        })
    }
}

/// In order to try dealing with https://github.com/sunng87/cargo-release/issues/224 and also to make workspace
/// releases more selective.
pub fn release(opts: Options, crates: Vec<String>, bump: String, bump_dependencies: String) -> anyhow::Result<()> {
    if opts.dry_run_cargo_publish && !opts.dry_run {
        bail!("The --no-dry-run-cargo-publish flag is only effective without --execute")
    }
    let ctx = Context::new(crates, bump, bump_dependencies, opts.changelog)?;
    if opts.update_crates_index {
        log::info!("Updating crates-io index at '{}'", ctx.crates_index.path().display());
        ctx.crates_index.update()?;
    } else if opts.bump_when_needed {
        log::warn!(
            "Consider running with --update-crates-index to assure bumping on demand uses the latest information"
        );
    }
    if !ctx.crates_index.exists() {
        log::warn!("Crates.io index doesn't exist. Consider using --update-crates-index to help determining if release versions are published already");
    }

    release_depth_first(ctx, opts)?;
    Ok(())
}

fn release_depth_first(ctx: Context, options: Options) -> anyhow::Result<()> {
    let meta = &ctx.base.meta;
    let changed_crate_names_to_publish = if options.skip_dependencies {
        ctx.base.crate_names.clone()
    } else {
        crate::traverse::dependencies(&ctx.base, options.verbose, options.allow_auto_publish_of_stable_crates)?
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
            let tag_name = git::create_version_tag(publishee, &new_version, commit_id, &ctx.base, options)?;
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
        if let Err(err) = crate::git::assure_clean_working_tree() {
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

    let commit_id = manifest::edit_version_and_fixup_dependent_crates_and_handle_changelog(
        meta,
        &crates_to_publish_together,
        options,
        ctx,
    )?;

    crates_to_publish_together.reverse();
    let mut tag_names = Vec::new();
    while let Some((publishee, new_version)) = crates_to_publish_together.pop() {
        let unpublished_crates: Vec<_> = crates_to_publish_together
            .iter()
            .map(|(p, _)| p.name.to_owned())
            .collect();

        cargo::publish_crate(publishee, &unpublished_crates, options)?;
        if let Some(tag_name) = git::create_version_tag(publishee, &new_version, commit_id.clone(), &ctx.base, options)?
        {
            tag_names.push(tag_name);
        };
    }
    git::push_tags_and_head(tag_names, options)?;
    Ok(())
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
    let commit_id = manifest::edit_version_and_fixup_dependent_crates_and_handle_changelog(
        meta,
        &[(publishee, new_version.clone())],
        options,
        ctx,
    )?;
    cargo::publish_crate(publishee, &[], options)?;
    Ok((new_version, commit_id))
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
