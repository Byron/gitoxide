use std::{borrow::Cow, collections::BTreeMap, str::FromStr};

use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package};
use semver::{Op, Version, VersionReq};

use super::{cargo, git, version, Context, Oid, Options};
use crate::{
    changelog,
    utils::{names_and_versions, package_by_id, package_eq_dependency, will},
    ChangeLog,
};

pub(in crate::command::release_impl) fn edit_version_and_fixup_dependent_crates_and_handle_changelog<'repo>(
    meta: &Metadata,
    publishees: &[(&Package, String)],
    opts: Options,
    ctx: &'repo Context,
) -> anyhow::Result<Option<Oid<'repo>>> {
    let mut locks_by_manifest_path = BTreeMap::new();
    let mut pending_changelog_changes = Vec::new();
    let Options {
        verbose,
        dry_run,
        skip_publish,
        preview,
        ..
    } = opts;
    let mut empty_changelogs_for_current_version = Vec::new();
    for (publishee, new_version) in publishees {
        let lock = git_repository::lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_repository::lock::acquire::Fail::Immediately,
            None,
        )?;
        let previous = locks_by_manifest_path.insert(&publishee.manifest_path, lock);
        assert!(previous.is_none(), "publishees are unique so insertion always happens");
        if let Some(history) = ctx.history.as_ref() {
            let (mut log, mut changed, mut lock) =
                ChangeLog::for_package_with_write_lock(publishee, history, &ctx.base, opts.dry_run)?;
            let release_in_log = log.most_recent_release_mut();
            let new_version: semver::Version = new_version.parse()?;
            match release_in_log {
                changelog::Section::Release {
                    name: name @ changelog::Version::Unreleased,
                    ..
                } => {
                    if !changed {
                        log::info!(
                            "{}: {} only change headline from 'Unreleased' to '{}'",
                            publishee.name,
                            will(dry_run),
                            new_version
                        );
                    }
                    *name = changelog::Version::Semantic(new_version);
                    changed = true;
                }
                changelog::Section::Release {
                    name: changelog::Version::Semantic(recent_version),
                    ..
                } => {
                    if *recent_version != new_version {
                        anyhow::bail!("'{}' does not have an unreleased version, and most recent release is unexpected. Wanted {}, got {}.", publishee.name, new_version, recent_version);
                    }
                }
                changelog::Section::Verbatim { .. } => unreachable!("BUG: checked in prior function"),
            };
            if changed {
                if !release_in_log.is_essential() {
                    empty_changelogs_for_current_version.push(pending_changelog_changes.len());
                }
                lock.with_mut(|file| log.write_to(file))?;
                pending_changelog_changes.push((publishee, lock));
            } else {
                log::info!("Won't rewrite changelog for '{}' as it didn't change", publishee.name);
            }
        }
    }

    let mut dependent_packages =
        collect_directly_dependent_packages(meta, publishees, &mut locks_by_manifest_path, ctx, opts)?;
    let publishees_and_bumped_dependent_packages = publishees
        .iter()
        .map(|(p, v)| (*p, v.to_owned()))
        .chain(
            dependent_packages
                .clone()
                .into_iter()
                .filter_map(|(p, v)| v.map(|v| (p, v))),
        )
        .collect::<Vec<_>>();
    let mut made_change = false;
    for (publishee, new_version) in publishees {
        let mut lock = locks_by_manifest_path
            .get_mut(&publishee.manifest_path)
            .expect("lock available");
        made_change |= set_version_and_update_package_dependency(
            publishee,
            Some(&new_version.to_string()),
            &publishees_and_bumped_dependent_packages,
            &mut lock,
            opts,
        )?;
    }

    for (dependant_on_publishee, possibly_new_version) in dependent_packages.iter_mut() {
        let mut lock = locks_by_manifest_path
            .get_mut(&dependant_on_publishee.manifest_path)
            .expect("lock written once");
        made_change |= set_version_and_update_package_dependency(
            dependant_on_publishee,
            possibly_new_version.as_deref(),
            &publishees_and_bumped_dependent_packages,
            &mut lock,
            opts,
        )?;
    }

    let message = format!(
        "{} {}{}",
        if skip_publish {
            "Bump"
        } else if empty_changelogs_for_current_version.is_empty() {
            "Release"
        } else {
            "Adjusting Changelogs prior to release of"
        },
        names_and_versions(publishees),
        {
            let safety_bumped_packages = dependent_packages
                .into_iter()
                .filter_map(|(p, v)| v.map(|v| (p, v)))
                .collect::<Vec<_>>();
            if safety_bumped_packages.is_empty() {
                Cow::from("")
            } else {
                match safety_bumped_packages.len() {
                    1 => format!(", safety bump {}", names_and_versions(&safety_bumped_packages)).into(),
                    num_crates => format!(
                        ", safety bump {} crates\n\nSAFETY BUMP: {}",
                        num_crates,
                        names_and_versions(&safety_bumped_packages)
                    )
                    .into(),
                }
            }
        }
    );

    if verbose {
        log::info!(
            "{} persist changes to {} manifests {}with: {:?}",
            will(dry_run),
            locks_by_manifest_path.len(),
            match (
                pending_changelog_changes.len(),
                pending_changelog_changes.iter().fold(0usize, |mut acc, (_, lock)| {
                    acc += if !lock.resource_path().is_file() { 1 } else { 0 };
                    acc
                })
            ) {
                (0, _) => Cow::Borrowed(""),
                (num_logs, num_new) => format!(
                    "and {} changelogs {}",
                    num_logs,
                    match num_new {
                        0 => Cow::Borrowed(""),
                        num_new => format!("({} new) ", num_new).into(),
                    }
                )
                .into(),
            },
            message
        );
    }

    if !pending_changelog_changes.is_empty() && preview {
        log::info!(
            "About to preview {} changelog(s), use --no-changelog-preview to disable or Ctrl-C to abort, or the 'changelog' subcommand to write it out for adjustments.",
            pending_changelog_changes.len()
        );

        let bat = crate::bat::Support::new();
        for (_package, lock) in &pending_changelog_changes {
            bat.display_to_tty(lock.lock_path())?;
        }
    }

    let bail_message_after_commit = if !dry_run {
        let mut committed_changelogs = None;
        for (idx, (package, lock)) in pending_changelog_changes.into_iter().enumerate() {
            if empty_changelogs_for_current_version.is_empty() || empty_changelogs_for_current_version.contains(&idx) {
                lock.commit()?;
                if !empty_changelogs_for_current_version.is_empty() {
                    committed_changelogs.get_or_insert_with(Vec::new).push(package);
                }
            }
        }
        for manifest_lock in locks_by_manifest_path.into_values() {
            manifest_lock.commit()?;
        }
        // This is dangerous as incompatibilities can happen here, leaving the working tree dirty.
        // For now we leave it that way without auto-restoring originals to facilitate debugging.
        cargo::refresh_lock_file()?;

        if let Some(committed_changelogs) = committed_changelogs {
            let names_of_crates_in_need_of_changelog_entry = committed_changelogs
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            if skip_publish {
                log::warn!(
                    "Please consider creating changelog entries for crate(s) {}",
                    names_of_crates_in_need_of_changelog_entry
                );
                None
            } else {
                Some(format!(
                    "Write changelog entries for crate(s) {} and try again",
                    names_of_crates_in_need_of_changelog_entry
                ))
            }
        } else {
            None
        }
    } else {
        if !empty_changelogs_for_current_version.is_empty() {
            let names_of_crates_that_would_need_review = empty_changelogs_for_current_version
                .iter()
                .filter_map(|idx| {
                    pending_changelog_changes.iter().enumerate().find_map(|(pidx, (p, _))| {
                        if *idx == pidx {
                            Some(p.name.as_str())
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
                .join(", ");
            log::warn!(
                "WOULD {} as the changelog entry is empty for crates {}",
                if skip_publish {
                    "ask for review after commit"
                } else {
                    "stop release after commit"
                },
                names_of_crates_that_would_need_review
            );
        }
        None
    };

    let res = git::commit_changes(message, verbose, dry_run, !made_change, &ctx.base)?;
    if let Some(bail_message) = bail_message_after_commit {
        bail!(bail_message);
    } else {
        Ok(res)
    }
}

/// Packages that depend on any of the publishees, where publishee is used by them.
fn collect_directly_dependent_packages<'a>(
    meta: &'a Metadata,
    publishees: &[(&Package, String)],
    locks_by_manifest_path: &mut BTreeMap<&'a Utf8PathBuf, git_repository::lock::File>,
    ctx: &Context,
    Options {
        isolate_dependencies_from_breaking_changes,
        bump_when_needed,
        verbose,
        ..
    }: Options,
) -> anyhow::Result<Vec<(&'a Package, Option<String>)>> {
    let mut packages_to_fix = Vec::<(&Package, Option<String>)>::new();
    let mut dependent_packages_this_round = Vec::new();
    let publishees_backing = publishees
        .iter()
        .map(|(p, v)| (*p, Some(v.to_owned())))
        .collect::<Vec<_>>();
    let mut publishees_and_dependents = publishees_backing.as_slice();

    loop {
        for workspace_package in meta.workspace_members.iter().map(|id| package_by_id(meta, id)) {
            if !isolate_dependencies_from_breaking_changes {
                let has_publishee_in_dependencies = workspace_package.dependencies.iter().any(|dep| {
                    publishees_and_dependents
                        .iter()
                        .any(|(publishee, _)| package_eq_dependency(publishee, dep))
                });
                if !has_publishee_in_dependencies
                    || locks_by_manifest_path.contains_key(&workspace_package.manifest_path)
                {
                    continue;
                }
                let lock = git_repository::lock::File::acquire_to_update_resource(
                    &workspace_package.manifest_path,
                    git_repository::lock::acquire::Fail::Immediately,
                    None,
                )?;
                locks_by_manifest_path.insert(&workspace_package.manifest_path, lock);
                dependent_packages_this_round.push((workspace_package, None));
            } else {
                let mut desired_versions = Vec::<Version>::new();
                for dep in workspace_package.dependencies.iter() {
                    for (publishee_as_dependency, new_version) in
                        publishees_and_dependents.iter().filter_map(|(publishee, new_version)| {
                            new_version
                                .as_deref()
                                .and_then(|v| package_eq_dependency(publishee, dep).then(|| (*publishee, v)))
                        })
                    {
                        if let Some(version) = version::conservative_dependent_version(
                            publishee_as_dependency,
                            new_version,
                            workspace_package,
                            ctx,
                            bump_when_needed,
                            verbose,
                        ) {
                            desired_versions.push(version)
                        }
                    }
                }
                if desired_versions.is_empty() {
                    continue;
                }
                desired_versions.sort();

                let greatest_version = desired_versions.pop().expect("at least one version");
                let new_version = version::rhs_is_breaking_bump_for_lhs(&workspace_package.version, &greatest_version)
                    .then(|| greatest_version.to_string());

                if locks_by_manifest_path.contains_key(&workspace_package.manifest_path) {
                    if let Some(previous_version) = packages_to_fix
                        .iter()
                        .find_map(|(p, v)| (p.id == workspace_package.id && *v < new_version).then(|| v))
                    {
                        log::warn!(
                            "BUG: we encountered package {} again, and would need to update its version {:?} to {:?}",
                            workspace_package.name,
                            previous_version,
                            new_version
                        )
                    }
                    continue;
                }
                if new_version.is_some() || is_direct_dependency_of(publishees, workspace_package) {
                    let lock = git_repository::lock::File::acquire_to_update_resource(
                        &workspace_package.manifest_path,
                        git_repository::lock::acquire::Fail::Immediately,
                        None,
                    )?;
                    locks_by_manifest_path.insert(&workspace_package.manifest_path, lock);
                    dependent_packages_this_round.push((workspace_package, new_version));
                }
            };
        }
        if dependent_packages_this_round.is_empty() {
            break;
        }
        packages_to_fix.append(&mut dependent_packages_this_round);
        publishees_and_dependents = packages_to_fix.as_slice();

        if !isolate_dependencies_from_breaking_changes {
            break;
        }
    }
    Ok(packages_to_fix)
}

fn is_direct_dependency_of(publishees: &[(&Package, String)], package_to_fix: &Package) -> bool {
    package_to_fix.dependencies.iter().any(|dep| {
        publishees
            .iter()
            .any(|(publishee, _)| package_eq_dependency(publishee, dep))
    })
}

fn set_version_and_update_package_dependency(
    package_to_update: &Package,
    new_package_version: Option<&str>,
    publishees: &[(&Package, String)],
    mut out: impl std::io::Write,
    Options {
        verbose,
        conservative_pre_release_version_handling,
        ..
    }: Options,
) -> anyhow::Result<bool> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;

    if let Some(new_version) = new_package_version {
        if doc["package"]["version"].as_str() != Some(new_version) {
            doc["package"]["version"] = toml_edit::value(new_version);
            if verbose {
                log::info!(
                    "Pending '{}' manifest version update: \"{}\"",
                    package_to_update.name,
                    new_version
                );
            }
        }
    }
    for dep_type in &["dependencies", "dev-dependencies", "build-dependencies"] {
        for (name_to_find, new_version) in publishees.iter().map(|(p, nv)| (&p.name, nv)) {
            let new_version = Version::parse(new_version)?;
            for name_to_find in package_to_update
                .dependencies
                .iter()
                .filter(|dep| &dep.name == name_to_find)
                .map(|dep| dep.rename.as_ref().unwrap_or(&dep.name))
            {
                if let Some(current_version_req) = doc
                    .as_table_mut()
                    .get_mut(dep_type)
                    .and_then(|deps| deps.as_table_mut())
                    .and_then(|deps| deps.get_mut(name_to_find).and_then(|name| name.as_inline_table_mut()))
                    .and_then(|name_table| name_table.get_mut("version"))
                {
                    let version_req = VersionReq::parse(current_version_req.as_str().expect("versions are strings"))?;
                    let force_update = conservative_pre_release_version_handling
                        && version::is_pre_release(&new_version) // setting the lower bound unnecessarily can be harmful
                        && !version::rhs_is_breaking_bump_for_lhs(&req_as_version(&version_req), &new_version); // don't claim to be conservative if this is necessary anyway
                    if !version_req.matches(&new_version) || force_update {
                        let supported_op = Op::Caret;
                        if version_req.comparators.is_empty()
                            || (version_req.comparators.len() > 1)
                            || version_req.comparators.last().expect("exists").op != supported_op
                        {
                            bail!("{} has it's {} dependency set to a version requirement with comparator {} - cannot currently handle that.", package_to_update.name, name_to_find, current_version_req);
                        }
                        let new_version = format!("^{}", new_version);
                        if verbose && version_req.to_string() != new_version {
                            log::info!(
                                "Pending '{}' {}manifest {} update: '{} = \"{}\"' (from {})",
                                package_to_update.name,
                                if force_update { "conservative " } else { "" },
                                dep_type,
                                name_to_find,
                                new_version,
                                current_version_req.to_string()
                            );
                        }
                        *current_version_req = toml_edit::Value::from(new_version.as_str());
                    }
                }
            }
        }
    }
    let new_manifest = doc.to_string_in_original_order();
    out.write_all(new_manifest.as_bytes())?;

    Ok(manifest != new_manifest)
}

fn req_as_version(req: &VersionReq) -> Version {
    let comp = &req.comparators.get(0).expect("at least one version comparator");
    Version {
        major: comp.major,
        minor: comp.minor.unwrap_or(0),
        patch: comp.patch.unwrap_or(0),
        pre: comp.pre.clone(),
        build: Default::default(),
    }
}
