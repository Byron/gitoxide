use std::collections::BTreeSet;

use anyhow::bail;
use cargo_metadata::{Dependency, DependencyKind, Metadata, Package};

use crate::version::BumpSpec;
use crate::{
    changelog,
    changelog::{write::Linkables, Section},
    command::release::Options,
    traverse::{self, dependency},
    utils::{
        is_dependency_with_version_requirement, names_and_versions, package_by_id, package_by_name,
        package_eq_dependency, package_for_dependency, tag_name, will, workspace_package_by_id,
    },
    version,
};

mod cargo;
mod git;
mod github;
mod manifest;

type Oid<'repo> = git_repository::easy::Oid<'repo, git_repository::Easy>;

pub(crate) struct Context {
    base: crate::Context,
    changelog_links: Linkables,
}

impl Context {
    fn new(
        crate_names: Vec<String>,
        bump: BumpSpec,
        bump_dependencies: BumpSpec,
        changelog: bool,
        changelog_links: bool,
    ) -> anyhow::Result<Self> {
        let changelog_links = if changelog_links {
            crate::git::remote_url()?
                .map(|url| Linkables::AsLinks {
                    repository_url: url.into(),
                })
                .unwrap_or(Linkables::AsText)
        } else {
            Linkables::AsText
        };
        Ok(Context {
            base: crate::Context::new(crate_names, changelog, bump, bump_dependencies)?,
            changelog_links,
        })
    }
}

/// In order to try dealing with https://github.com/sunng87/cargo-release/issues/224 and also to make workspace
/// releases more selective.
pub fn release(opts: Options, crates: Vec<String>, bump: BumpSpec, bump_dependencies: BumpSpec) -> anyhow::Result<()> {
    if opts.dry_run_cargo_publish && !opts.dry_run {
        bail!("The --no-dry-run-cargo-publish flag is only effective without --execute")
    }
    let allow_changelog = if opts.changelog && opts.skip_tag {
        log::warn!("With --no-tag enabled, changelog generation will be disabled as it relies on tags to segment commit history.");
        false
    } else {
        opts.changelog
    };
    let ctx = Context::new(
        crates,
        bump,
        bump_dependencies,
        allow_changelog,
        !opts.no_changelog_links,
    )?;
    if opts.update_crates_index {
        log::info!(
            "Updating crates-io index at '{}'",
            ctx.base.crates_index.path().display()
        );
        ctx.base.crates_index.update()?;
    } else if opts.bump_when_needed {
        log::warn!(
            "Consider running with --update-crates-index to assure bumping on demand uses the latest information"
        );
    }
    if !ctx.base.crates_index.exists() {
        log::warn!("Crates.io index doesn't exist. Consider using --update-crates-index to help determining if release versions are published already");
    }

    release_depth_first(ctx, opts)?;
    Ok(())
}

fn release_depth_first(ctx: Context, options: Options) -> anyhow::Result<()> {
    let meta = &ctx.base.meta;
    let Options {
        bump_when_needed,
        dry_run,
        allow_auto_publish_of_stable_crates,
        skip_dependencies,
        multi_crate_release,
        allow_changelog_github_release,
        verbose,
        ..
    } = options;
    let changed_crate_names_to_publish = if skip_dependencies {
        ctx.base
            .crate_names
            .iter()
            .map(|name| package_by_name(&ctx.base.meta, name))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let dependencies =
            crate::traverse::dependencies(&ctx.base, allow_auto_publish_of_stable_crates, bump_when_needed)?;
        present_dependencies(&dependencies, &ctx, verbose, dry_run);
        dependencies
            .into_iter()
            .filter_map(|d| matches!(d.mode, dependency::Mode::ToBePublished { .. }).then(|| d.package))
            .collect()
    };

    let crates_to_publish_together = resolve_cycles_with_publish_group(meta, &changed_crate_names_to_publish, options)?;

    assure_working_tree_is_unchanged(options)?;

    if multi_crate_release && !changed_crate_names_to_publish.is_empty() {
        perform_multi_version_release(&ctx, options, meta, changed_crate_names_to_publish)?;
    } else {
        for publishee in changed_crate_names_to_publish
            .iter()
            .filter(|package| !crates_to_publish_together.iter().any(|p| p.id == package.id))
        {
            let (
                new_version,
                manifest::Outcome {
                    commit_id,
                    section_by_package: release_section_by_publishee,
                    ..
                },
            ) = perform_single_release(meta, publishee, options, &ctx)?;
            let tag_name = git::create_version_tag(
                publishee,
                &new_version,
                commit_id,
                release_section_by_publishee
                    .get(publishee.name.as_str())
                    .and_then(|s| section_to_string(s, WriteMode::Tag)),
                &ctx.base,
                options,
            )?;
            git::push_tags_and_head(tag_name.as_ref(), options)?;
            if let Some(message) = allow_changelog_github_release
                .then(|| {
                    release_section_by_publishee
                        .get(publishee.name.as_str())
                        .and_then(|s| section_to_string(s, WriteMode::GitHubRelease))
                })
                .flatten()
            {
                github::create_release(publishee, &new_version, &message, options, &ctx.base)?;
            }
        }
    }

    if !crates_to_publish_together.is_empty() {
        perform_multi_version_release(&ctx, options, meta, crates_to_publish_together)?;
    }

    Ok(())
}

fn present_dependencies(deps: &[traverse::Dependency<'_>], ctx: &Context, verbose: bool, dry_run: bool) {
    use dependency::{Kind, SkippedReason};
    if verbose {
        for dep in deps {
            match &dep.mode {
                dependency::Mode::ToBePublished {
                    kind,
                    change_kind: crate::git::PackageChangeKind::Untagged { wanted_tag_name },
                    ..
                } => {
                    log::info!(
                        "{} '{}' wasn't tagged with {} yet and thus needs a release",
                        match kind {
                            Kind::UserSelection => "Provided package",
                            Kind::DependencyOfUserSelection => "Dependent package",
                        },
                        dep.package.name,
                        wanted_tag_name
                    );
                }
                dependency::Mode::ToBePublished {
                    kind: Kind::DependencyOfUserSelection,
                    change_kind: crate::git::PackageChangeKind::ChangedOrNew,
                    ..
                } => {
                    log::info!(
                        "Dependent package '{}' v{} will be published as it changed since last release",
                        dep.package.name,
                        dep.package.version
                    );
                }
                dependency::Mode::Skipped {
                    kind,
                    reason: SkippedReason::Unchanged,
                } => {
                    log::info!(
                        "Skipped {} '{}' v{} as it didn't change since last release",
                        match kind {
                            Kind::UserSelection => "provided package",
                            Kind::DependencyOfUserSelection => "dependent package",
                        },
                        dep.package.name,
                        dep.package.version
                    );
                }
                dependency::Mode::Skipped {
                    kind: _,
                    reason: SkippedReason::DeniedAutopublishOfProductionCrate,
                } => {
                    log::warn!(
                        "Production crate '{}' v{} changed since last release - consider releasing it beforehand.",
                        dep.package.name,
                        dep.package.version
                    );
                }
                dependency::Mode::ToBePublished {
                    kind: Kind::UserSelection,
                    change_kind: _,
                    ..
                } => {}
            }
        }
    } else {
        let num_skipped = deps
            .iter()
            .filter(|dep| matches!(&dep.mode, dependency::Mode::Skipped { .. }))
            .count();
        if num_skipped != 0 {
            log::info!(
                "Skipped {} dependent crates as they didn't change since their last release. Use --verbose/-v to see much more.",
                num_skipped
            );
        }
    }
    for dep in deps {
        match &dep.mode {
            dependency::Mode::ToBePublished {
                next_release_version,
                kind,
                ..
            } => {
                let bump_spec = match kind {
                    dependency::Kind::UserSelection => ctx.base.bump,
                    dependency::Kind::DependencyOfUserSelection => ctx.base.bump_dependencies,
                };
                if next_release_version > &dep.package.version {
                    log::info!(
                        "{} {}-bump '{}' from {} to {}",
                        will(dry_run),
                        bump_spec,
                        dep.package.name,
                        dep.package.version,
                        next_release_version
                    );
                } else {
                    log::info!(
                        "'{}' v{} in manifest is sufficient for release",
                        dep.package.name,
                        dep.package.version,
                    );
                }
            }
            dependency::Mode::Skipped { .. } => {}
        }
    }
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

fn perform_multi_version_release(
    ctx: &Context,
    options: Options,
    meta: &Metadata,
    crates_to_publish_together: Vec<&Package>,
) -> anyhow::Result<()> {
    let mut crates_to_publish_together = crates_to_publish_together
        .into_iter()
        .map(|p| {
            version::bump(
                p,
                version::select_publishee_bump_spec(&p.name, &ctx.base),
                &ctx.base,
                options.bump_when_needed,
            )
            .map(|v| (p, v.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let manifest::Outcome {
        commit_id,
        section_by_package: release_section_by_publishee,
        safety_bumped_packages,
    } = manifest::edit_version_and_fixup_dependent_crates_and_handle_changelog(
        meta,
        &crates_to_publish_together,
        options,
        ctx,
    )?;

    log::info!(
        "{} prepare releases of {}{}",
        will(options.dry_run),
        names_and_versions(&crates_to_publish_together),
        if safety_bumped_packages.is_empty() {
            "".to_string()
        } else {
            format!(
                ", bumping {} crate{} for safety",
                safety_bumped_packages.len(),
                if safety_bumped_packages.len() == 1 { "" } else { "s" }
            )
        }
    );

    crates_to_publish_together.reverse();
    let mut tag_names = Vec::new();
    for (publishee, new_version) in crates_to_publish_together.iter().rev() {
        let unpublished_crates: Vec<_> = crates_to_publish_together
            .iter()
            .map(|(p, _)| p.name.to_owned())
            .collect();

        cargo::publish_crate(publishee, &unpublished_crates, options)?;
        if let Some(tag_name) = git::create_version_tag(
            publishee,
            new_version,
            commit_id.clone(),
            release_section_by_publishee
                .get(&publishee.name.as_str())
                .and_then(|s| section_to_string(s, WriteMode::Tag)),
            &ctx.base,
            options,
        )? {
            tag_names.push(tag_name);
        };
    }
    git::push_tags_and_head(tag_names.iter(), options)?;
    if options.allow_changelog_github_release {
        for (publishee, new_version) in crates_to_publish_together.iter().rev() {
            if let Some(message) = release_section_by_publishee
                .get(&publishee.name.as_str())
                .and_then(|s| section_to_string(s, WriteMode::GitHubRelease))
            {
                github::create_release(publishee, new_version, &message, options, &ctx.base)?;
            }
        }
    }
    Ok(())
}

enum WriteMode {
    Tag,
    GitHubRelease,
}

fn section_to_string(section: &Section, mode: WriteMode) -> Option<String> {
    let mut b = String::new();
    section
        .write_to(
            &mut b,
            &changelog::write::Linkables::AsText,
            match mode {
                WriteMode::Tag => changelog::write::Components::empty(),
                WriteMode::GitHubRelease => changelog::write::Components::DETAIL_TAGS,
            },
        )
        .ok()
        .map(|_| b)
}

fn perform_single_release<'repo, 'meta>(
    meta: &'meta Metadata,
    publishee: &'meta Package,
    options: Options,
    ctx: &'repo Context,
) -> anyhow::Result<(String, manifest::Outcome<'repo, 'meta>)> {
    let bump_spec = version::select_publishee_bump_spec(&publishee.name, &ctx.base);
    let new_version = version::bump(publishee, bump_spec, &ctx.base, options.bump_when_needed)?;
    let new_version = new_version.to_string();
    let commit_id_and_changelog_sections = manifest::edit_version_and_fixup_dependent_crates_and_handle_changelog(
        meta,
        &[(publishee, new_version.clone())],
        options,
        ctx,
    )?;
    log::info!(
        "{} prepare release of {} v{}",
        will(options.dry_run),
        publishee.name,
        new_version
    );
    cargo::publish_crate(publishee, &[], options)?;
    Ok((new_version, commit_id_and_changelog_sections))
}

fn resolve_cycles_with_publish_group<'meta>(
    meta: &'meta Metadata,
    changed_crate_names_to_publish: &[&'meta Package],
    options: Options,
) -> anyhow::Result<Vec<&'meta Package>> {
    let mut crates_to_publish_additionally_to_avoid_instability = Vec::new();
    let mut publish_group = Vec::<&Package>::new();
    for publishee in changed_crate_names_to_publish.iter() {
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
                if !changed_crate_names_to_publish.iter().any(|p| p.id == from.id) {
                    crates_to_publish_additionally_to_avoid_instability.push(from.name.as_str());
                } else {
                    for package in &[from, publishee] {
                        if !publish_group.iter().any(|p| p.id == package.id) {
                            publish_group.push(package)
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

fn reorder_according_to_existing_order<'meta>(
    reference_order: &[&'meta Package],
    packages_to_order: &[&'meta Package],
) -> Vec<&'meta Package> {
    let new_order = reference_order
        .iter()
        .filter(|package| packages_to_order.iter().any(|p| p.id == package.id))
        .fold(Vec::new(), |mut acc, package| {
            acc.push(*package);
            acc
        });
    assert_eq!(
        new_order.len(),
        packages_to_order.len(),
        "the reference order must contain all items to be ordered"
    );
    new_order
}

struct Cycle<'meta> {
    from: &'meta Package,
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
