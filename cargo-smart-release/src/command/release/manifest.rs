use std::{borrow::Cow, collections::BTreeMap, io::Write, str::FromStr};

use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package};
use semver::{Op, Version, VersionReq};

use super::{cargo, git, version, Context, Oid, Options};
use crate::changelog::write::Linkables;
use crate::{
    changelog,
    utils::{names_and_versions, package_by_id, package_eq_dependency, will},
    ChangeLog,
};

pub(in crate::command::release_impl) fn edit_version_and_fixup_dependent_crates_and_handle_changelog<'repo, 'a>(
    meta: &Metadata,
    publishees: &[(&'a Package, String)],
    opts: Options,
    ctx: &'repo Context,
) -> anyhow::Result<(Option<Oid<'repo>>, BTreeMap<&'a str, changelog::Section>)> {
    let mut locks_by_manifest_path = BTreeMap::new();
    let mut pending_changelog_changes = Vec::new();
    let Options {
        verbose,
        dry_run,
        skip_publish,
        preview,
        allow_fully_generated_changelogs,
        ..
    } = opts;
    let mut changelog_ids_with_statistical_segments_only = Vec::new();
    let mut changelog_ids_probably_lacking_user_edits = Vec::new();
    let mut made_change = false;
    let next_commit_date = crate::utils::time_to_offset_date_time(crate::git::author()?.time);
    let mut release_section_by_publishee = BTreeMap::default();
    for (publishee, new_version) in publishees {
        let lock = git_repository::lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_repository::lock::acquire::Fail::Immediately,
            None,
        )?;
        let previous = locks_by_manifest_path.insert(&publishee.manifest_path, lock);
        assert!(previous.is_none(), "publishees are unique so insertion always happens");
        if let Some(history) = ctx.history.as_ref() {
            let changelog::init::Outcome {
                mut log,
                state: log_init_state,
                previous_content,
                mut lock,
            } = ChangeLog::for_package_with_write_lock(publishee, history, &ctx.base, opts.generator_segments)?;

            log::info!(
                "{} {} changelog for '{}'.",
                will(opts.dry_run),
                match log_init_state {
                    changelog::init::State::Created => "create a new",
                    changelog::init::State::Modified => "modify existing",
                    changelog::init::State::Unchanged => "leave alone the",
                },
                publishee.name
            );

            let (recent_idx, recent_release_section_in_log) = log.most_recent_release_section_mut();
            if !recent_release_section_in_log.is_essential() {
                changelog_ids_with_statistical_segments_only.push(pending_changelog_changes.len());
            } else if recent_release_section_in_log.is_probably_lacking_user_edits() {
                changelog_ids_probably_lacking_user_edits.push(pending_changelog_changes.len());
            }
            let new_version: semver::Version = new_version.parse()?;
            match recent_release_section_in_log {
                changelog::Section::Release {
                    name: name @ changelog::Version::Unreleased,
                    date,
                    ..
                } => {
                    if !log_init_state.is_modified() {
                        log::info!(
                            "{}: {} only change headline from 'Unreleased' to '{}'",
                            publishee.name,
                            will(dry_run),
                            new_version
                        );
                    }
                    *name = changelog::Version::Semantic(new_version.clone());
                    *date = Some(next_commit_date);
                    let recent_section = log.sections.remove(recent_idx);
                    match log
                        .sections
                        .iter_mut()
                        .find(|s| matches!(s, changelog::Section::Release {name: changelog::Version::Semantic(v), ..} if *v == new_version))
                    {
                        Some(version_section) => {
                            version_section.merge(recent_section);
                            let pop_if_changelog_id_is_last = |v: &mut Vec<usize>| {
                                if v.last().filter(|&&idx| idx == pending_changelog_changes.len()).is_some() {
                                    v.pop();
                                }
                            };
                            if version_section.is_essential() {
                                pop_if_changelog_id_is_last(&mut changelog_ids_with_statistical_segments_only);
                            } else if !version_section.is_probably_lacking_user_edits() {
                                pop_if_changelog_id_is_last(&mut changelog_ids_probably_lacking_user_edits);
                            }
                        }
                        None => log.sections.insert(recent_idx, recent_section),
                    }
                }
                changelog::Section::Release {
                    name: changelog::Version::Semantic(recent_version),
                    date,
                    ..
                } => {
                    if *recent_version != new_version {
                        anyhow::bail!(
                            "'{}' does not have an unreleased version, and most recent release is unexpected. Wanted {}, got {}.",
                            publishee.name,
                            new_version,
                            recent_version
                        );
                    }
                    *date = Some(next_commit_date);
                }
                changelog::Section::Verbatim { .. } => unreachable!("BUG: checked in prior function"),
            };
            let mut write_buf = String::new();
            log.write_to(
                &mut write_buf,
                if opts.dry_run {
                    &Linkables::AsText
                } else {
                    &ctx.changelog_links
                },
                if opts.dry_run {
                    changelog::write::Components::SECTION_TITLE
                } else {
                    changelog::write::Components::all()
                },
            )?;
            lock.with_mut(|file| file.write_all(write_buf.as_bytes()))?;
            made_change |= previous_content.map(|previous| write_buf != previous).unwrap_or(true);
            pending_changelog_changes.push((publishee, log_init_state.is_modified(), lock));
            release_section_by_publishee.insert(publishee.name.as_str(), log.take_recent_release_section());
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

    let would_stop_release = !(changelog_ids_with_statistical_segments_only.is_empty()
        && changelog_ids_probably_lacking_user_edits.is_empty());
    let message = format!(
        "{} {}{}",
        if would_stop_release {
            "Adjusting changelogs prior to release of"
        } else if skip_publish {
            "Bump"
        } else {
            "Release"
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
                pending_changelog_changes.iter().fold(0usize, |mut acc, (_, _, lock)| {
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
        let additional_info =
            "use --no-changelog-preview to disable or Ctrl-C to abort, or the 'changelog' subcommand.";
        log::info!(
            "About to preview {} pending changelog(s), {}",
            pending_changelog_changes
                .iter()
                .filter(|(_, has_changes, _)| *has_changes)
                .count(),
            additional_info
        );

        let bat = crate::bat::Support::new();
        let additional_info = format!(
            "PREVIEW, {}{}",
            if opts.dry_run { "simplified, " } else { "" },
            additional_info
        );
        for (_, _, lock) in pending_changelog_changes
            .iter()
            .filter(|(_, has_changes, _)| *has_changes)
        {
            bat.display_to_tty(
                lock.lock_path(),
                lock.resource_path().strip_prefix(&ctx.base.root.to_path_buf())?,
                &additional_info,
            )?;
        }
    }

    let bail_message_after_commit = if !dry_run {
        let mut packages_whose_changelogs_need_edits = None;
        let mut packages_which_might_be_fully_generated = None;
        for (idx, (package, _, lock)) in pending_changelog_changes.into_iter().enumerate() {
            if changelog_ids_with_statistical_segments_only.is_empty()
                || changelog_ids_with_statistical_segments_only.contains(&idx)
            {
                lock.commit()?;
                if !changelog_ids_with_statistical_segments_only.is_empty() {
                    packages_whose_changelogs_need_edits
                        .get_or_insert_with(Vec::new)
                        .push(package);
                }
            } else {
                drop(lock);
            }
            if changelog_ids_probably_lacking_user_edits.contains(&idx) {
                packages_which_might_be_fully_generated
                    .get_or_insert_with(Vec::new)
                    .push(package);
            }
        }
        for manifest_lock in locks_by_manifest_path.into_values() {
            manifest_lock.commit()?;
        }
        // This is dangerous as incompatibilities can happen here, leaving the working tree dirty.
        // For now we leave it that way without auto-restoring originals to facilitate debugging.
        cargo::refresh_lock_file()?;

        packages_whose_changelogs_need_edits
            .and_then(|logs| {
                let names_of_crates_in_need_of_changelog_entry =
                    logs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ");
                if skip_publish {
                    log::warn!(
                        "Please consider creating changelog entries for crate{}: {}",
                        if logs.len() == 1 { "" } else { "s" },
                        names_of_crates_in_need_of_changelog_entry
                    );
                    None
                } else {
                    Some(format!(
                        "Write changelog entries for crate(s) {} and try again",
                        names_of_crates_in_need_of_changelog_entry
                    ))
                }
            })
            .or_else(|| {
                packages_which_might_be_fully_generated.and_then(|packages| {
                    let crate_names = packages.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ");
                    (!allow_fully_generated_changelogs).then(|| {
                        format!(
                            "{} edits by hand to avoid being entirely generated: {}",
                            if crate_names.len() == 1 {
                                "This changelog needs"
                            } else {
                                "These changelogs need"
                            },
                            crate_names
                        )
                    })
                })
            })
    } else {
        let comma_separated_crate_names = |ids: &[usize]| {
            ids.iter()
                .filter_map(|idx| {
                    pending_changelog_changes
                        .iter()
                        .enumerate()
                        .find_map(
                            |(pidx, (p, _, _))| {
                                if *idx == pidx {
                                    Some(p.name.as_str())
                                } else {
                                    None
                                }
                            },
                        )
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        if !changelog_ids_with_statistical_segments_only.is_empty() {
            let names_of_crates_that_would_need_review =
                comma_separated_crate_names(&changelog_ids_with_statistical_segments_only);
            log::warn!(
                "WOULD {} as the changelog entry is empty for crate{}: {}",
                if skip_publish {
                    "ask for review after commit"
                } else {
                    "stop release after commit"
                },
                if changelog_ids_with_statistical_segments_only.len() == 1 {
                    ""
                } else {
                    "s"
                },
                names_of_crates_that_would_need_review
            );
        }
        if !changelog_ids_probably_lacking_user_edits.is_empty() {
            log::warn!(
                "{} likely to be fully generated from commit history or contain lower-case git-conventional headlines: {}{}",
                if changelog_ids_probably_lacking_user_edits.len() == 1 {
                    "This changelog is"
                } else {
                    "These changelogs are"
                },
                comma_separated_crate_names(&changelog_ids_probably_lacking_user_edits),
                if allow_fully_generated_changelogs {
                    ""
                } else {
                    ". The release process would stop to allow edits."
                }
            );
        }
        None
    };

    let res = git::commit_changes(message, verbose, dry_run, !made_change, &ctx.base)?;
    if let Some(bail_message) = bail_message_after_commit {
        bail!(bail_message);
    } else {
        Ok((res, release_section_by_publishee))
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
                            bail!(
                                "{} has it's {} dependency set to a version requirement with comparator {} - cannot currently handle that.",
                                package_to_update.name,
                                name_to_find,
                                current_version_req
                            );
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
