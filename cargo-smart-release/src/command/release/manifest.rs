use std::{
    borrow::Cow,
    collections::{btree_map::Entry, BTreeMap},
    io::Write,
    str::FromStr,
};

use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Package};
use git_repository::{easy::Oid, lock::File};
use semver::{Version, VersionReq};

use super::{cargo, git, Context, Options};
use crate::{
    changelog,
    changelog::{write::Linkables, Section},
    traverse::Dependency,
    utils::{names_and_versions, try_to_published_crate_and_new_version, version_req_unset_or_default, will},
    version, ChangeLog,
};

pub struct Outcome<'repo, 'meta> {
    pub commit_id: Option<Oid<'repo>>,
    pub section_by_package: BTreeMap<&'meta str, changelog::Section>,
}

pub(in crate::command::release_impl) fn edit_version_and_fixup_dependent_crates_and_handle_changelog<'repo, 'meta>(
    crates: &[Dependency<'meta>],
    opts: Options,
    ctx: &'repo Context,
) -> anyhow::Result<Outcome<'repo, 'meta>> {
    let Options { dry_run, changelog, .. } = opts;
    let crates_and_versions_to_be_published: Vec<_> = crates
        .iter()
        .filter_map(try_to_published_crate_and_new_version)
        .collect();
    let GatherOutcome {
        pending_changelogs,
        mut locks_by_manifest_path,
        changelog_ids_with_statistical_segments_only,
        changelog_ids_probably_lacking_user_edits,
        release_section_by_publishee,
        mut made_change,
    } = changelog
        .then(|| gather_changelog_data(ctx, &crates_and_versions_to_be_published, opts))
        .transpose()?
        .unwrap_or_default();

    let crates_with_version_change: Vec<_> = crates
        .iter()
        .filter_map(|c| c.mode.version_adjustment_bump().map(|b| (c.package, &b.next_release)))
        .collect();
    for (package, possibly_new_version) in crates
        .iter()
        .filter(|c| c.mode.manifest_will_change())
        .map(|c| (c.package, c.mode.version_adjustment_bump().map(|b| &b.next_release)))
    {
        let mut entry_store;
        let lock = match locks_by_manifest_path.entry(&package.manifest_path) {
            Entry::Occupied(entry) => {
                entry_store = entry;
                entry_store.get_mut()
            }
            Entry::Vacant(entry) => entry.insert(git_repository::lock::File::acquire_to_update_resource(
                &package.manifest_path,
                git_repository::lock::acquire::Fail::Immediately,
                None,
            )?),
        };
        made_change |= set_version_and_update_package_dependency(
            package,
            possibly_new_version,
            &crates_with_version_change,
            lock,
            opts,
        )?;
    }

    let would_stop_release = !(changelog_ids_with_statistical_segments_only.is_empty()
        && changelog_ids_probably_lacking_user_edits.is_empty());
    let safety_bumped_packages = crates
        .iter()
        .filter_map(|c| c.mode.safety_bump().map(|b| (c.package, &b.next_release)))
        .collect::<Vec<_>>();
    let commit_message = generate_commit_message(
        &crates_and_versions_to_be_published,
        &safety_bumped_packages,
        would_stop_release,
        locks_by_manifest_path.len(),
        &pending_changelogs,
        opts,
    );

    preview_changelogs(ctx, &pending_changelogs, opts)?;

    let bail_message = commit_locks_and_generate_bail_message(
        ctx,
        pending_changelogs,
        locks_by_manifest_path,
        changelog_ids_with_statistical_segments_only,
        changelog_ids_probably_lacking_user_edits,
        opts,
    )?;

    let res = git::commit_changes(commit_message, dry_run, !made_change, &ctx.base)?;
    if let Some(bail_message) = bail_message {
        bail!(bail_message);
    } else {
        Ok(Outcome {
            commit_id: res,
            section_by_package: release_section_by_publishee,
        })
    }
}

fn commit_locks_and_generate_bail_message(
    ctx: &Context,
    pending_changelogs: Vec<(&Package, bool, File)>,
    locks_by_manifest_path: BTreeMap<&Utf8PathBuf, File>,
    changelog_ids_with_statistical_segments_only: Vec<usize>,
    changelog_ids_probably_lacking_user_edits: Vec<usize>,
    Options {
        dry_run,
        skip_publish,
        allow_fully_generated_changelogs,
        ..
    }: Options,
) -> anyhow::Result<Option<String>> {
    let bail_message_after_commit = if !dry_run {
        let mut packages_whose_changelogs_need_edits = None;
        let mut packages_which_might_be_fully_generated = None;
        for (idx, (package, _, lock)) in pending_changelogs.into_iter().enumerate() {
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
        let crate_names = |ids: &[usize]| {
            ids.iter()
                .filter_map(|idx| {
                    pending_changelogs.iter().enumerate().find_map(|(pidx, (p, _, _))| {
                        if *idx == pidx {
                            Some(p.name.as_str())
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
        };
        let fix_preamble = "";
        if !changelog_ids_with_statistical_segments_only.is_empty() {
            let crate_names = crate_names(&changelog_ids_with_statistical_segments_only);
            let names_of_crates_that_would_need_review = crate_names.join(", ");
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
            log::warn!(
                "To fix the changelog manually, run: cargo changelog --write {}",
                ctx.base.crate_names.join(" ")
            );
        }
        if !changelog_ids_probably_lacking_user_edits.is_empty() {
            let crate_names = crate_names(&changelog_ids_probably_lacking_user_edits);
            log::warn!(
                "{} likely to be fully generated from commit history or contain lower-case git-conventional headlines: {}{}",
                if changelog_ids_probably_lacking_user_edits.len() == 1 {
                    "This changelog is"
                } else {
                    "These changelogs are"
                },
                crate_names.join(", "),
                if allow_fully_generated_changelogs {
                    ""
                } else {
                    ". Would stop after commit to allow edits."
                }
            );
            if !allow_fully_generated_changelogs {
                for crate_name in crate_names {
                    log::warn!("{} {}", fix_preamble, crate_name);
                }
            }
        }
        None
    };
    Ok(bail_message_after_commit)
}

fn preview_changelogs(
    ctx: &Context,
    pending_changelogs: &[(&Package, bool, File)],
    Options { dry_run, preview, .. }: Options,
) -> anyhow::Result<()> {
    if !pending_changelogs.is_empty() && preview && !dry_run {
        let additional_info =
            "use --no-changelog-preview to disable or Ctrl-C to abort, or the 'changelog' subcommand.";
        let changelogs_with_changes = pending_changelogs
            .iter()
            .filter_map(|(_, has_changes, lock)| (*has_changes).then(|| lock))
            .collect::<Vec<_>>();
        log::info!(
            "About to preview {} pending changelog(s), {}",
            changelogs_with_changes.len(),
            additional_info
        );

        let bat = crate::bat::Support::new();
        for (idx, lock) in changelogs_with_changes.iter().enumerate() {
            let additional_info = format!(
                "PREVIEW {} / {}, {}{}",
                idx + 1,
                changelogs_with_changes.len(),
                if dry_run { "simplified, " } else { "" },
                additional_info
            );
            bat.display_to_tty(
                lock.lock_path(),
                lock.resource_path().strip_prefix(&ctx.base.root.to_path_buf())?,
                additional_info,
            )?;
        }
    } else if !pending_changelogs.is_empty() && preview {
        log::info!(
            "Up to {} changelog{} would be previewed if the --execute is set and --no-changelog-preview is unset.",
            pending_changelogs.len(),
            if pending_changelogs.len() == 1 { "" } else { "s" }
        );
    }
    Ok(())
}

fn generate_commit_message(
    crates_and_versions_to_be_published: &[(&Package, &Version)],
    safety_bumped_packages: &[(&Package, &Version)],
    would_stop_release: bool,
    num_locks: usize,
    pending_changelogs: &[(&Package, bool, File)],
    Options {
        skip_publish, dry_run, ..
    }: Options,
) -> String {
    let message = format!(
        "{} {}{}",
        if would_stop_release {
            "Adjusting changelogs prior to release of"
        } else if skip_publish {
            "Bump"
        } else {
            "Release"
        },
        names_and_versions(crates_and_versions_to_be_published),
        {
            if safety_bumped_packages.is_empty() {
                Cow::from("")
            } else {
                let names_and_versions = names_and_versions(safety_bumped_packages);
                match safety_bumped_packages.len() {
                    1 => format!(", safety bump {}", names_and_versions).into(),
                    num_crates => format!(
                        ", safety bump {} crates\n\nSAFETY BUMP: {}",
                        num_crates, names_and_versions
                    )
                    .into(),
                }
            }
        }
    );

    log::trace!(
        "{} persist changes to {} manifests {}with: {:?}",
        will(dry_run),
        num_locks,
        match (
            pending_changelogs.len(),
            pending_changelogs.iter().fold(0usize, |mut acc, (_, _, lock)| {
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
    message
}

#[derive(Default)]
pub struct GatherOutcome<'meta> {
    pending_changelogs: Vec<(&'meta Package, bool, File)>,
    locks_by_manifest_path: BTreeMap<&'meta Utf8PathBuf, File>,
    /// Ids into `pending_changelogs`
    changelog_ids_with_statistical_segments_only: Vec<usize>,
    changelog_ids_probably_lacking_user_edits: Vec<usize>,
    release_section_by_publishee: BTreeMap<&'meta str, Section>,
    made_change: bool,
}

fn gather_changelog_data<'a, 'meta>(
    ctx: &Context,
    crates_and_versions_to_be_published: &[(&'meta Package, &'a Version)],
    Options {
        dry_run,
        generator_segments,
        ..
    }: Options,
) -> anyhow::Result<GatherOutcome<'meta>> {
    let mut out = GatherOutcome::default();
    let GatherOutcome {
        pending_changelogs,
        locks_by_manifest_path,
        changelog_ids_with_statistical_segments_only,
        changelog_ids_probably_lacking_user_edits,
        release_section_by_publishee,
        made_change,
    } = &mut out;
    let next_commit_date = crate::utils::time_to_offset_date_time(crate::git::author()?.time);
    for (publishee, new_version) in crates_and_versions_to_be_published {
        let lock = git_repository::lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_repository::lock::acquire::Fail::Immediately,
            None,
        )?;
        let previous = locks_by_manifest_path.insert(&publishee.manifest_path, lock);
        assert!(previous.is_none(), "publishees are unique so insertion always happens");
        if let Some(history) = ctx.base.history.as_ref() {
            let changelog::init::Outcome {
                mut log,
                state: log_init_state,
                previous_content,
                mut lock,
            } = ChangeLog::for_package_with_write_lock(publishee, history, &ctx.base, generator_segments)?;

            log::info!(
                "{} {} changelog for '{}'.",
                will(dry_run),
                match log_init_state {
                    changelog::init::State::Created => "create a new",
                    changelog::init::State::Modified => "modify existing",
                    changelog::init::State::Unchanged => "leave alone the",
                },
                publishee.name
            );

            let (recent_idx, recent_release_section_in_log) = log.most_recent_release_section_mut();
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
                    *name = changelog::Version::Semantic((*new_version).to_owned());
                    *date = Some(next_commit_date);
                    let recent_section = log.sections.remove(recent_idx);
                    match log
                        .sections
                        .iter_mut()
                        .find(|s| matches!(s, changelog::Section::Release {name: changelog::Version::Semantic(v), ..} if v == *new_version))
                    {
                        Some(version_section) => {
                            version_section.merge(recent_section);
                        }
                        None => log.sections.insert(recent_idx, recent_section),
                    }
                }
                changelog::Section::Release {
                    name: changelog::Version::Semantic(recent_version),
                    date,
                    ..
                } => {
                    if recent_version != *new_version {
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
            {
                let (_, recent_release_section_in_log) = log.most_recent_release_section_mut();
                if !recent_release_section_in_log.is_essential() {
                    changelog_ids_with_statistical_segments_only.push(pending_changelogs.len());
                } else if recent_release_section_in_log.is_probably_lacking_user_edits() {
                    changelog_ids_probably_lacking_user_edits.push(pending_changelogs.len());
                }
            }
            let mut write_buf = String::new();
            log.write_to(
                &mut write_buf,
                if dry_run {
                    &Linkables::AsText
                } else {
                    &ctx.changelog_links
                },
                if dry_run {
                    changelog::write::Components::SECTION_TITLE
                } else {
                    changelog::write::Components::all()
                },
            )?;
            lock.with_mut(|file| file.write_all(write_buf.as_bytes()))?;
            *made_change |= previous_content.map(|previous| write_buf != previous).unwrap_or(true);
            pending_changelogs.push((publishee, log_init_state.is_modified(), lock));
            release_section_by_publishee.insert(publishee.name.as_str(), log.take_recent_release_section());
        }
    }
    Ok(out)
}

fn set_version_and_update_package_dependency(
    package_to_update: &Package,
    new_package_version: Option<&semver::Version>,
    crates: &[(&Package, &semver::Version)],
    mut out: impl std::io::Write,
    Options {
        conservative_pre_release_version_handling,
        ..
    }: Options,
) -> anyhow::Result<bool> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;

    if let Some(new_version) = new_package_version {
        let new_version = new_version.to_string();
        if doc["package"]["version"].as_str() != Some(new_version.as_str()) {
            log::trace!(
                "Pending '{}' manifest version update: \"{}\"",
                package_to_update.name,
                new_version
            );
            doc["package"]["version"] = toml_edit::value(new_version);
        }
    }
    for dep_type in &["dependencies", "dev-dependencies", "build-dependencies"] {
        for (name_to_find, new_version) in crates.iter().map(|(p, nv)| (&p.name, nv)) {
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
                        && version::is_pre_release(new_version) // setting the lower bound unnecessarily can be harmful
                        // don't claim to be conservative if this is necessary anyway
                        && req_as_version(&version_req).map(|req_version|!version::rhs_is_breaking_bump_for_lhs(&req_version, new_version)).unwrap_or(false);
                    if !version_req.matches(new_version) || force_update {
                        if !version_req_unset_or_default(&version_req) {
                            bail!(
                                "{} has it's {} dependency set to a version requirement with comparator {} - cannot currently handle that.",
                                package_to_update.name,
                                name_to_find,
                                current_version_req
                            );
                        }
                        let new_version = format!("^{}", new_version);
                        if version_req.to_string() != new_version {
                            log::trace!(
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
    let new_manifest = doc.to_string();
    out.write_all(new_manifest.as_bytes())?;

    Ok(manifest != new_manifest)
}

fn req_as_version(req: &VersionReq) -> Option<Version> {
    req.comparators.get(0).map(|comp| Version {
        major: comp.major,
        minor: comp.minor.unwrap_or(0),
        patch: comp.patch.unwrap_or(0),
        pre: comp.pre.clone(),
        build: Default::default(),
    })
}
