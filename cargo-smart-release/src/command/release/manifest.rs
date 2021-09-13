use std::{collections::BTreeMap, str::FromStr};

use anyhow::bail;
use cargo_metadata::{camino::Utf8PathBuf, Metadata, Package};
use git_lock::File;
use semver::{Op, Version, VersionReq};

use super::{
    cargo, git,
    utils::{names_and_versions, package_by_id, package_eq_dependency, will},
    version, Context, Oid, Options,
};

pub(in crate::command::release_impl) fn edit_version_and_fixup_dependent_crates<'repo>(
    meta: &Metadata,
    publishees: &[(&Package, String)],
    opts: Options,
    ctx: &'repo Context,
) -> anyhow::Result<Option<Oid<'repo>>> {
    let mut locks_by_manifest_path = BTreeMap::new();
    let Options {
        verbose,
        dry_run,
        skip_publish,
        ..
    } = opts;
    for (publishee, _) in publishees {
        let lock = git_lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_lock::acquire::Fail::Immediately,
            None,
        )?;
        let previous = locks_by_manifest_path.insert(&publishee.manifest_path, lock);
        assert!(previous.is_none(), "publishees are unique so insertion always happens");
    }

    let mut dependent_packages =
        collect_directly_dependent_packages(meta, publishees, &mut locks_by_manifest_path, ctx, opts)?;
    let mut made_change = false;
    for (publishee, new_version) in publishees {
        let mut lock = locks_by_manifest_path
            .get_mut(&publishee.manifest_path)
            .expect("lock available");
        made_change |= set_version_and_update_package_dependency(
            publishee,
            Some(&new_version.to_string()),
            publishees,
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
            publishees,
            &mut lock,
            opts,
        )?;
    }

    let message = format!(
        "{} {}",
        if skip_publish { "Bump" } else { "Release" },
        names_and_versions(publishees)
    );
    if verbose {
        log::info!("{} persist changes to manifests with: {:?}", will(dry_run), message);
    }
    if !dry_run {
        for manifest_lock in locks_by_manifest_path.into_values() {
            manifest_lock.commit()?;
        }
        // This is dangerous as incompatibilities can happen here, leaving the working tree dirty.
        // For now we leave it that way without auto-restoring originals to facilitate debugging.
        cargo::refresh_lock_file()?;
    }
    git::commit_changes(message, verbose, dry_run, !made_change, ctx)
}

fn collect_directly_dependent_packages<'a>(
    meta: &'a Metadata,
    publishees: &[(&Package, String)],
    locks_by_manifest_path: &mut BTreeMap<&'a Utf8PathBuf, File>,
    ctx: &Context,
    Options {
        conservative_pre_release_version_handling,
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
    let publishees_and_dependents = publishees_backing.as_slice();

    // eprintln!("start");
    loop {
        // dbg!(publishees.iter().map(|(p, _)| &p.name).collect::<Vec<_>>());
        for package_to_fix in meta.workspace_members.iter().map(|id| package_by_id(meta, id)) {
            if conservative_pre_release_version_handling {
                let has_publishee_in_dependencies = package_to_fix.dependencies.iter().any(|dep| {
                    publishees_and_dependents
                        .iter()
                        .any(|(publishee, _)| package_eq_dependency(publishee, dep))
                });
                if !has_publishee_in_dependencies || locks_by_manifest_path.contains_key(&package_to_fix.manifest_path)
                {
                    continue;
                }
                let lock = git_lock::File::acquire_to_update_resource(
                    &package_to_fix.manifest_path,
                    git_lock::acquire::Fail::Immediately,
                    None,
                )?;
                locks_by_manifest_path.insert(&package_to_fix.manifest_path, lock);
                dependent_packages_this_round.push((package_to_fix, None));
            } else {
                let mut desired_versions = Vec::<Version>::new();
                for dep in package_to_fix.dependencies.iter() {
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
                            package_to_fix,
                            ctx,
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
                let new_version = version::rhs_is_major_bump_for_lhs(&package_to_fix.version, &greatest_version)
                    .then(|| greatest_version.to_string());

                if locks_by_manifest_path.contains_key(&package_to_fix.manifest_path) {
                    if let Some(previous_version) = packages_to_fix
                        .iter()
                        .find_map(|(p, v)| (&p.id == &package_to_fix.id && &*v < &new_version).then(|| v))
                    {
                        log::warn!(
                            "BUG: we encountered package {} again, and would need to update its version {:?} to {:?}",
                            package_to_fix.name,
                            previous_version,
                            new_version
                        )
                    }
                    continue;
                }
                if new_version.is_some()
                    || package_to_fix.dependencies.iter().any(|dep| {
                        publishees
                            .iter()
                            .any(|(publishee, _)| package_eq_dependency(publishee, dep))
                    })
                {
                    let lock = git_lock::File::acquire_to_update_resource(
                        &package_to_fix.manifest_path,
                        git_lock::acquire::Fail::Immediately,
                        None,
                    )?;
                    locks_by_manifest_path.insert(&package_to_fix.manifest_path, lock);
                    dependent_packages_this_round.push((package_to_fix, new_version));
                }
            };
        }
        if dependent_packages_this_round.is_empty() {
            break;
        }
        packages_to_fix.append(&mut dependent_packages_this_round);

        if !conservative_pre_release_version_handling {
            break;
        }
    }
    Ok(packages_to_fix)
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
        doc["package"]["version"] = toml_edit::value(new_version);
        if verbose {
            log::info!(
                "Pending '{}' manifest version update: \"{}\"",
                package_to_update.name,
                new_version
            );
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
                    let force_update =
                        conservative_pre_release_version_handling && version::is_pre_release(&new_version);
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
