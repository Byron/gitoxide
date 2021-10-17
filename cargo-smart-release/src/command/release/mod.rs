use anyhow::bail;
use cargo_metadata::{Metadata, Package};

use crate::{
    changelog,
    changelog::{write::Linkables, Section},
    command::release::Options,
    traverse::{self, dependency, dependency::VersionAdjustment},
    utils::{names_and_versions, package_by_name, tag_name, will},
    version,
    version::BumpSpec,
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
        present_dependencies(&dependencies, &ctx, verbose, dry_run)?;
        dependencies
            .into_iter()
            .filter_map(|d| matches!(d.mode, dependency::Mode::ToBePublished { .. }).then(|| d.package))
            .collect()
    };

    assure_working_tree_is_unchanged(options)?;

    if !changed_crate_names_to_publish.is_empty() {
        perform_multi_version_release(&ctx, options, meta, changed_crate_names_to_publish)?;
    }

    Ok(())
}

fn present_dependencies(
    deps: &[traverse::Dependency<'_>],
    ctx: &Context,
    _verbose: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    use dependency::Kind;
    let num_skipped = deps
        .iter()
        .filter(|dep| matches!(&dep.mode, dependency::Mode::Skipped { .. }))
        .count();
    if num_skipped != 0 {
        log::info!(
            "Skipped {} dependent crates as they didn't change since their last release.",
            num_skipped
        );
    }

    let mut error = false;
    for dep in deps {
        let (bump_spec, kind) = match dep.kind {
            Kind::UserSelection => (ctx.base.bump, "provided"),
            Kind::DependencyOrDependentOfUserSelection => (ctx.base.bump_dependencies, "dependent"),
        };
        match &dep.mode {
            dependency::Mode::ToBePublished { adjustment } => {
                let (bump, breaking_dependency) = match adjustment {
                    VersionAdjustment::Breakage {
                        bump,
                        direct_dependency,
                        ..
                    } => (bump, Some(direct_dependency)),
                    VersionAdjustment::Changed { bump, .. } => (bump, None),
                };
                match &bump.next_release {
                    Ok(next_release) => {
                        if next_release > &dep.package.version {
                            log::info!(
                                "{} {}-bump {} package '{}' from {} to {}{}{}{}",
                                will(dry_run),
                                bump_spec,
                                kind,
                                dep.package.name,
                                dep.package.version,
                                next_release,
                                bump.latest_release
                                    .as_ref()
                                    .and_then(|latest_release| {
                                        (dep.package.version != *latest_release)
                                            .then(|| format!(", {} on crates.io", latest_release))
                                    })
                                    .unwrap_or_default(),
                                (*next_release != bump.desired_release)
                                    .then(|| format!(", ignoring computed version {}", bump.desired_release))
                                    .unwrap_or_default(),
                                breaking_dependency
                                    .map(|cause| format!(", for SAFETY due to breaking package '{}'", cause.name))
                                    .unwrap_or_default()
                            );
                        } else {
                            log::info!(
                            "Manifest version of {} package '{}' at {} is sufficient{}, ignoring computed version {}",
                            kind,
                            dep.package.name,
                            dep.package.version,
                            bump.latest_release
                                .as_ref()
                                .map(|latest_release| format!(" to succeed latest released version {}", latest_release))
                                .unwrap_or_else(|| ", creating a new release 🎉".into()),
                            bump.desired_release
                        );
                        }
                    }
                    Err(version::bump::Error::LatestReleaseMoreRecentThanDesiredOne(latest_release)) => {
                        log::warn!(
                        "Latest published version of '{}' is {}, the new version is {}. Consider using --bump <level> or --bump-dependencies <level> or update the index with --update-crates-index.",
                        dep.package.name,
                        latest_release,
                        bump.desired_release
                    );
                        error = true;
                    }
                };
            }
            dependency::Mode::Skipped {
                adjustment:
                    Some(VersionAdjustment::Breakage {
                        bump,
                        direct_dependency,
                        ..
                    }),
                ..
            } => {
                log::info!(
                    "Adjusting manifest of {} package '{}' to {} for SAFETY due to breaking change in '{}'",
                    kind,
                    dep.package.name,
                    bump.next_release
                        .as_ref()
                        .expect("next version always set for safety bumps"),
                    direct_dependency.name
                );
            }
            dependency::Mode::ManifestNeedsUpdate => {
                log::info!(
                    "Manifest of {} package '{}' will be adjusted as its dependencies see a version change",
                    kind,
                    dep.package.name
                );
            }
            dependency::Mode::Skipped { .. } => {}
        }
    }

    if error {
        bail!("Aborting due to previous error(s)");
    } else {
        Ok(())
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
            .map(|v| (p, v))
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
