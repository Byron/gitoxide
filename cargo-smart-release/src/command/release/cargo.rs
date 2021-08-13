use super::{
    git,
    utils::{
        bump_spec_may_cause_empty_commits, bump_version, names_and_versions, package_by_id, package_eq_dependency, will,
    },
    Context, Options,
};
use anyhow::bail;
use cargo_metadata::{Metadata, Package};
use git_repository::hash::ObjectId;
use std::{collections::BTreeMap, process::Command, str::FromStr};

pub(in crate::command::release_impl) fn perform_single_release(
    meta: &Metadata,
    publishee: &Package,
    options: Options,
    bump_spec: &str,
    ctx: &Context,
) -> anyhow::Result<(String, ObjectId)> {
    let new_version = bump_version(&publishee.version.to_string(), bump_spec)?.to_string();
    log::info!(
        "{} prepare release of {} v{}",
        will(options.dry_run),
        publishee.name,
        new_version
    );
    let commit_id = edit_manifest_and_fixup_dependent_crates(
        meta,
        &[(publishee, new_version.clone())],
        bump_spec_may_cause_empty_commits(bump_spec),
        options,
        ctx,
    )?;
    publish_crate(publishee, &[], options)?;
    Ok((new_version, commit_id))
}

pub(in crate::command::release_impl) fn publish_crate(
    publishee: &Package,
    other_publishee_names: &[String],
    Options {
        skip_publish,
        dry_run,
        allow_dirty,
        no_verify,
        ..
    }: Options,
) -> anyhow::Result<()> {
    if skip_publish {
        return Ok(());
    }
    let max_attempts = 3;
    let must_not_verify = publishee
        .dependencies
        .iter()
        .any(|dep| other_publishee_names.contains(&dep.name));
    for attempt in 1..=max_attempts {
        let mut c = Command::new("cargo");
        c.arg("publish");

        if allow_dirty {
            c.arg("--allow-dirty");
        }
        if no_verify || must_not_verify {
            c.arg("--no-verify");
        }
        c.arg("--manifest-path").arg(&publishee.manifest_path);
        log::info!("{} run {:?}", will(dry_run), c);
        if dry_run || c.status()?.success() {
            break;
        } else if attempt == max_attempts {
            bail!("Could not successfully execute 'cargo publish' even ")
        } else {
            log::warn!(
                "'cargo publish' run {} failed but we retry up to {} times to rule out flakiness",
                attempt,
                max_attempts
            );
        }
    }
    Ok(())
}

pub(in crate::command::release_impl) fn edit_manifest_and_fixup_dependent_crates(
    meta: &Metadata,
    publishees: &[(&Package, String)],
    empty_commit_possible: bool,
    Options {
        dry_run, allow_dirty, ..
    }: Options,
    ctx: &Context,
) -> anyhow::Result<ObjectId> {
    if !allow_dirty {
        git::assure_clean_working_tree()?;
    }
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

    for (publishee, new_version) in publishees {
        let mut lock = locks_by_manifest_path
            .get_mut(&publishee.manifest_path)
            .expect("lock available");
        set_version_and_update_package_dependency(publishee, Some(&new_version.to_string()), publishees, &mut lock)?;
    }

    for package_to_update in packages_to_fix.iter_mut() {
        let mut lock = locks_by_manifest_path
            .get_mut(&package_to_update.manifest_path)
            .expect("lock written once");
        set_version_and_update_package_dependency(package_to_update, None, publishees, &mut lock)?;
    }

    let message = format!("Release {}", names_and_versions(publishees));
    if dry_run {
        log::info!("WOULD commit changes to manifests with {:?}", message);
        Ok(ObjectId::null_sha1())
    } else {
        log::info!("Persisting changes to manifests");
        for manifest_lock in locks_by_manifest_path.into_values() {
            manifest_lock.commit()?;
        }
        refresh_cargo_lock()?;
        git::commit_changes(message, empty_commit_possible, ctx)
    }
}

fn set_version_and_update_package_dependency(
    package_to_update: &Package,
    new_version: Option<&str>,
    publishees: &[(&Package, String)],
    mut out: impl std::io::Write,
) -> anyhow::Result<()> {
    let manifest = std::fs::read_to_string(&package_to_update.manifest_path)?;
    let mut doc = toml_edit::Document::from_str(&manifest)?;

    if let Some(new_version) = new_version {
        doc["package"]["version"] = toml_edit::value(new_version);
        log::info!(
            "Pending '{}' manifest version update: \"{}\"",
            package_to_update.name,
            new_version
        );
    }
    for dep_type in &["dependencies", "dev-dependencies", "build-dependencies"] {
        for (name_to_find, new_version) in publishees.iter().map(|(p, nv)| (&p.name, nv)) {
            if let Some(name_table) = doc
                .as_table_mut()
                .get_mut(dep_type)
                .and_then(|deps| deps.as_table_mut())
                .and_then(|deps| deps.get_mut(name_to_find).and_then(|name| name.as_inline_table_mut()))
            {
                log::info!(
                    "Pending '{}' manifest {} update: '{} = \"{}\"'",
                    package_to_update.name,
                    dep_type,
                    name_to_find,
                    new_version,
                );
                *name_table.get_or_insert("version", new_version.as_str()) =
                    toml_edit::Value::from(new_version.as_str());
            }
        }
    }
    out.write_all(doc.to_string_in_original_order().as_bytes())?;

    Ok(())
}

fn refresh_cargo_lock() -> anyhow::Result<()> {
    cargo_metadata::MetadataCommand::new().exec()?;
    Ok(())
}
