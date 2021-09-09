use std::{collections::BTreeMap, str::FromStr};

use anyhow::bail;
use cargo_metadata::{Metadata, Package};
use semver::{Op, Version, VersionReq};

use super::{
    cargo, git,
    utils::{names_and_versions, package_by_id, package_eq_dependency, will},
    Context, Oid, Options,
};

pub(in crate::command::release_impl) fn edit_version_and_fixup_dependent_crates<'repo>(
    meta: &Metadata,
    publishees: &[(&Package, String)],
    Options {
        verbose,
        dry_run,
        skip_publish,
        conservative_pre_release_version_handling,
        ..
    }: Options,
    ctx: &'repo Context,
) -> anyhow::Result<Option<Oid<'repo>>> {
    let mut locks_by_manifest_path = BTreeMap::new();
    for (publishee, _) in publishees {
        let lock = git_lock::File::acquire_to_update_resource(
            &publishee.manifest_path,
            git_lock::acquire::Fail::Immediately,
            None,
        )?;
        locks_by_manifest_path.insert(&publishee.manifest_path, lock);
    }
    let mut packages_to_fix = Vec::new();
    for package_to_fix in meta
        .workspace_members
        .iter()
        .map(|id| package_by_id(meta, id))
        .filter(|p| {
            p.dependencies.iter().any(|dep| {
                publishees
                    .iter()
                    .any(|(publishee, _)| package_eq_dependency(publishee, dep))
            })
        })
    {
        if locks_by_manifest_path.contains_key(&package_to_fix.manifest_path) {
            continue;
        }
        let lock = git_lock::File::acquire_to_update_resource(
            &package_to_fix.manifest_path,
            git_lock::acquire::Fail::Immediately,
            None,
        )?;
        locks_by_manifest_path.insert(&package_to_fix.manifest_path, lock);
        packages_to_fix.push(package_to_fix);
    }

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
            verbose,
            conservative_pre_release_version_handling,
        )?;
    }

    for package_to_update in packages_to_fix.iter_mut() {
        let mut lock = locks_by_manifest_path
            .get_mut(&package_to_update.manifest_path)
            .expect("lock written once");
        made_change |= set_version_and_update_package_dependency(
            package_to_update,
            None,
            publishees,
            &mut lock,
            verbose,
            conservative_pre_release_version_handling,
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

fn set_version_and_update_package_dependency(
    package_to_update: &Package,
    new_version: Option<&str>,
    publishees: &[(&Package, String)],
    mut out: impl std::io::Write,
    verbose: bool,
    conservative_pre_release_version_handling: bool,
) -> anyhow::Result<bool> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;

    if let Some(new_version) = new_version {
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
                    let force_update = if conservative_pre_release_version_handling && new_version.major == 0 {
                        true
                    } else {
                        false
                    };
                    if !version_req.matches(&new_version) || force_update {
                        let supported_op = Op::Caret;
                        if version_req.comparators.is_empty()
                            || (version_req.comparators.len() > 1)
                            || version_req.comparators.last().expect("exists").op != supported_op
                        {
                            bail!("{} has it's {} dependency set to a version requirement with comparator {} - cannot currently handle that.", package_to_update.name, name_to_find, current_version_req);
                        }
                        let new_version = format!("^{}", new_version);
                        if verbose {
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
