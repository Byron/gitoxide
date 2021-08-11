use anyhow::{anyhow, bail};
use cargo_metadata::{Metadata, PackageId};
use std::collections::BTreeSet;

pub fn release(dry_run: bool, version_bump_spec: String, crates: Vec<String>) -> anyhow::Result<()> {
    let meta = cargo_metadata::MetadataCommand::new().exec()?;
    let mut seen = BTreeSet::new();
    for crate_name in crates {
        if !meta.workspace_members.iter().any(|p| to_name(p) == crate_name) {
            bail!("Package to release must be a workspace member: '{}'", crate_name);
        }
        release_depth_first(dry_run, &meta, &crate_name, &version_bump_spec, &mut seen)?;
    }
    Ok(())
}

fn release_depth_first(
    dry_run: bool,
    meta: &Metadata,
    crate_name: &str,
    bump_spec: &str,
    seen: &mut BTreeSet<String>,
) -> anyhow::Result<()> {
    let package = meta
        .packages
        .iter()
        .find(|p| p.name == crate_name)
        .ok_or_else(|| anyhow!("workspace member must be a listed package: '{}'", crate_name))?;
    for dependency in &package.dependencies {
        if seen.contains(&dependency.name) || !meta.workspace_members.iter().any(|p| to_name(p) == dependency.name) {
            continue;
        }
        seen.insert(dependency.name.clone());
        release_depth_first(dry_run, meta, &dependency.name, bump_spec, seen)?;
    }
    log::info!("ready to release {}", crate_name);
    Ok(())
}

fn to_name(p: &PackageId) -> &str {
    p.repr
        .splitn(2, " ")
        .next()
        .expect("crate-name <additional data we don't need>")
}
