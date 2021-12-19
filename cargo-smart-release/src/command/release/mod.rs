use std::collections::BTreeMap;

use anyhow::bail;

use crate::{
    changelog,
    changelog::{write::Linkables, Section},
    command::release::Options,
    traverse::{
        self, dependency,
        dependency::{ManifestAdjustment, VersionAdjustment},
        Dependency,
    },
    utils::{tag_name, try_to_published_crate_and_new_version, will, Program},
    version,
    version::BumpSpec,
};

mod cargo;
mod git;
mod github;
mod manifest;

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
    let ctx = Context::new(crates, bump, bump_dependencies, allow_changelog, opts.changelog_links)?;
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

impl From<Options> for crate::traverse::Options {
    fn from(v: Options) -> Self {
        Self {
            allow_auto_publish_of_stable_crates: v.allow_auto_publish_of_stable_crates,
            bump_when_needed: v.bump_when_needed,
            isolate_dependencies_from_breaking_changes: v.isolate_dependencies_from_breaking_changes,
            traverse_graph: v.dependencies,
        }
    }
}

fn release_depth_first(ctx: Context, opts: Options) -> anyhow::Result<()> {
    let crates = {
        crate::traverse::dependencies(&ctx.base, opts.into())
            .and_then(|crates| assure_crates_index_is_uptodate(crates, &ctx.base, opts.into()))
            .and_then(|crates| {
                present_and_validate_dependencies(&crates, &ctx, opts.verbose, opts.dry_run).map(|_| crates)
            })?
    };

    assure_working_tree_is_unchanged(opts)?;
    perform_release(&ctx, opts, &crates)?;

    Ok(())
}

fn assure_crates_index_is_uptodate<'meta>(
    crates: Vec<Dependency<'meta>>,
    ctx: &'meta crate::Context,
    opts: crate::traverse::Options,
) -> anyhow::Result<Vec<Dependency<'meta>>> {
    if let Some(dep) = crates
        .iter()
        .filter_map(|d| d.mode.version_adjustment_bump().map(|b| (d, b)))
        .find_map(|(d, b)| {
            b.latest_release
                .as_ref()
                .and_then(|lr| (lr >= &b.desired_release).then(|| d))
        })
    {
        let index = crates_index::Index::new_cargo_default();
        if index.exists() {
            log::warn!("Crate '{}' computed version not greater than the current package version. Updating crates index to assure correct results.", dep.package.name);
            index.update()?;
            return crate::traverse::dependencies(ctx, opts);
        }
    }
    Ok(crates)
}

fn present_and_validate_dependencies(
    crates: &[traverse::Dependency<'_>],
    ctx: &Context,
    verbose: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    use dependency::Kind;
    let all_skipped: Vec<_> = crates
        .iter()
        .filter_map(|dep| match &dep.mode {
            dependency::Mode::NotForPublishing { reason, adjustment } => {
                Some((dep.package.name.as_str(), adjustment.is_some(), *reason))
            }
            _ => None,
        })
        .collect();
    let mut num_refused = 0;
    for (refused_crate, has_adjustment, _) in all_skipped
        .iter()
        .filter(|(name, _, _)| ctx.base.crate_names.iter().any(|n| n == *name))
    {
        num_refused += 1;
        log::warn!(
            "Refused to publish '{}' as {}.",
            refused_crate,
            has_adjustment
                .then(|| "only a manifest change is needed")
                .unwrap_or("as it didn't change")
        );
    }

    let no_requested_crate_will_publish = num_refused == ctx.base.crate_names.len();
    let no_crate_being_published_message = "No provided crate is actually eligible for publishing.";
    if no_requested_crate_will_publish && !verbose {
        bail!(
            "{} Use --verbose to see the release plan nonetheless.",
            no_crate_being_published_message
        )
    }

    let skipped = all_skipped
        .iter()
        .filter_map(|(name, has_adjustment, reason)| (!has_adjustment).then(|| (*name, reason)))
        .collect::<Vec<_>>();
    if !skipped.is_empty() {
        let skipped_len = skipped.len();
        let mut crates_by_reason: Vec<_> = skipped
            .into_iter()
            .fold(BTreeMap::default(), |mut acc, (name, reason)| {
                acc.entry(*reason).or_insert_with(Vec::new).push(name);
                acc
            })
            .into_iter()
            .collect();
        crates_by_reason.sort_by_key(|(k, _)| *k);

        log::info!(
            "Will not publish or alter {} dependent crate{}: {}",
            skipped_len,
            (skipped_len != 1).then(|| "s").unwrap_or(""),
            crates_by_reason
                .into_iter()
                .map(|(key, names)| format!(
                    "{} = {}",
                    key,
                    names.iter().map(|n| format!("'{}'", n)).collect::<Vec<_>>().join(", ")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    if all_skipped.len() == crates.len() {
        bail!("There is no crate eligible for publishing.");
    }

    let mut error = false;
    for dep in crates {
        let (bump_spec, kind) = match dep.kind {
            Kind::UserSelection => (ctx.base.bump, "provided"),
            Kind::DependencyOrDependentOfUserSelection => (ctx.base.bump_dependencies, "dependent"),
        };
        match &dep.mode {
            dependency::Mode::ToBePublished { adjustment } => {
                let (bump, breaking_dependencies) = match adjustment {
                    VersionAdjustment::Breakage {
                        bump,
                        causing_dependency_names,
                        ..
                    } => (bump, Some(causing_dependency_names)),
                    VersionAdjustment::Changed { bump, .. } => (bump, None),
                };
                if let Some(latest_release) = bump
                    .latest_release
                    .as_ref()
                    .and_then(|lr| (*lr >= bump.desired_release).then(|| lr))
                {
                    let bump_flag = match dep.kind {
                        dependency::Kind::UserSelection => "--bump <level>",
                        dependency::Kind::DependencyOrDependentOfUserSelection => "--bump-dependencies <level>",
                    };
                    if bump.desired_release == bump.package_version {
                        log::warn!(
                            "'{}' is unchanged. Consider using {} along with --no-bump-on-demand to force a version change.",
                            dep.package.name,
                            bump_flag
                        );
                    } else {
                        log::warn!(
                            "Latest published version of '{}' is {}, the new version is {}. Consider using {} or update the index with --update-crates-index.",
                            dep.package.name,
                            latest_release,
                            bump.desired_release,
                            bump_flag
                        );
                    }
                    error = true;
                }
                if bump.next_release > dep.package.version {
                    log::info!(
                        "{} {}-bump {} package '{}' from {} to {} for publishing{}{}{}",
                        will(dry_run),
                        bump_spec,
                        kind,
                        dep.package.name,
                        dep.package.version,
                        bump.next_release,
                        bump.latest_release
                            .as_ref()
                            .and_then(|latest_release| {
                                (dep.package.version != *latest_release)
                                    .then(|| format!(", {} on crates.io", latest_release))
                            })
                            .unwrap_or_default(),
                        breaking_dependencies
                            .map(|causes| format!(
                                ", for SAFETY due to breaking package{} {}",
                                if causes.len() == 1 { "" } else { "s" },
                                causes.iter().map(|n| format!("'{}'", n)).collect::<Vec<_>>().join(", ")
                            ))
                            .unwrap_or_default(),
                        (bump.next_release != bump.desired_release)
                            .then(|| format!(", ignoring computed version {}", bump.desired_release))
                            .unwrap_or_default(),
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
                };
            }
            dependency::Mode::NotForPublishing { .. } => {}
        }
    }

    {
        let affected_crates_by_cause = crates
            .iter()
            .filter_map(|dep| match &dep.mode {
                dependency::Mode::NotForPublishing {
                    adjustment:
                        Some(ManifestAdjustment::Version(VersionAdjustment::Breakage {
                            bump,
                            causing_dependency_names,
                            ..
                        })),
                    ..
                } => Some((dep, bump, causing_dependency_names)),
                _ => None,
            })
            .fold(
                Vec::new(),
                |mut acc: Vec<(&str, Vec<(&str, &version::Bump)>)>, (dep, bump, causing_names)| {
                    for name in causing_names {
                        match acc.iter_mut().find(|(k, _v)| k == name) {
                            Some((_k, deps)) => deps.push((dep.package.name.as_str(), bump)),
                            None => acc.push((name, vec![(dep.package.name.as_str(), bump)])),
                        }
                    }
                    acc
                },
            );
        for (cause, deps_and_bumps) in affected_crates_by_cause {
            let plural_s = (deps_and_bumps.len() != 1).then(|| "s").unwrap_or("");
            log::info!(
                "{} adjust {} manifest version{} due to breaking change in '{}': {}",
                will(dry_run),
                deps_and_bumps.len(),
                plural_s,
                cause,
                deps_and_bumps
                    .into_iter()
                    .map(|(dep_name, bump)| format!("'{}' {} ➡ {}", dep_name, bump.package_version, bump.next_release))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    {
        let crate_names_for_manifest_updates = crates
            .iter()
            .filter_map(|d| {
                matches!(
                    d.mode,
                    dependency::Mode::NotForPublishing {
                        adjustment: Some(ManifestAdjustment::DueToDependencyChange),
                        ..
                    }
                )
                .then(|| d.package.name.as_str())
            })
            .collect::<Vec<_>>();
        if !crate_names_for_manifest_updates.is_empty() {
            let plural_s = (crate_names_for_manifest_updates.len() > 1)
                .then(|| "s")
                .unwrap_or_default();
            log::info!(
                "{} adjust version constraints in manifest{} of {} package{} as direct dependencies are changing: {}",
                will(dry_run),
                plural_s,
                crate_names_for_manifest_updates.len(),
                plural_s,
                crate_names_for_manifest_updates.join(", ")
            );
        }
    }

    if error {
        bail!("Aborting due to previous error(s)");
    } else {
        if no_requested_crate_will_publish {
            bail!(no_crate_being_published_message)
        }
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

fn perform_release(ctx: &Context, options: Options, crates: &[traverse::Dependency<'_>]) -> anyhow::Result<()> {
    let manifest::Outcome {
        commit_id,
        section_by_package: release_section_by_publishee,
    } = manifest::edit_version_and_fixup_dependent_crates_and_handle_changelog(crates, options, ctx)?;

    let should_publish_to_github = options.allow_changelog_github_release
        && if Program::named("gh").found {
            true
        } else {
            log::warn!("To create github releases, please install the 'gh' program and try again");
            false
        };
    let mut tag_names = Vec::new();
    let mut successful_publishees_and_version = Vec::new();
    let mut publish_err = None;
    for (publishee, new_version) in crates.iter().filter_map(try_to_published_crate_and_new_version) {
        if let Err(err) = cargo::publish_crate(publishee, options) {
            publish_err = Some(err);
            break;
        }
        successful_publishees_and_version.push((publishee, new_version));
        if let Some(tag_name) = git::create_version_tag(
            publishee,
            new_version,
            commit_id,
            release_section_by_publishee
                .get(&publishee.name.as_str())
                .and_then(|s| section_to_string(s, WriteMode::Tag)),
            &ctx.base,
            options,
        )? {
            tag_names.push(tag_name);
        }
    }
    git::push_tags_and_head(&tag_names, options)?;
    if should_publish_to_github {
        for (publishee, new_version) in successful_publishees_and_version {
            release_section_by_publishee
                .get(&publishee.name.as_str())
                .and_then(|s| section_to_string(s, WriteMode::GitHubRelease))
                .map(|release_notes| github::create_release(publishee, new_version, &release_notes, options, &ctx.base))
                .transpose()?;
        }
    }

    publish_err.map(Err).unwrap_or(Ok(()))
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
