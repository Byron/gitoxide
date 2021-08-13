use anyhow::{anyhow, bail};
use cargo_metadata::{Dependency, Metadata, Package, PackageId};
use semver::{BuildMetadata, Prerelease, Version};

pub fn will(not_really: bool) -> &'static str {
    if not_really {
        "WOULD"
    } else {
        "Will"
    }
}

pub fn is_dependency_with_version_requirement(dep: &Dependency) -> bool {
    !dep.req.comparators.is_empty()
}

pub fn bump_spec_may_cause_empty_commits(bump_spec: &str) -> bool {
    bump_spec == "keep"
}

pub fn bump_version(version: &str, bump_spec: &str) -> anyhow::Result<Version> {
    let mut v = Version::parse(version)?;
    match bump_spec {
        "major" => {
            v.major += 1;
            v.minor = 0;
            v.patch = 0;
            v.build = BuildMetadata::EMPTY;
            v.pre = Prerelease::EMPTY;
        }
        "minor" => {
            v.minor += 1;
            v.patch = 0;
            v.build = BuildMetadata::EMPTY;
            v.pre = Prerelease::EMPTY;
        }
        "patch" => {
            v.patch += 1;
            v.build = BuildMetadata::EMPTY;
            v.pre = Prerelease::EMPTY;
        }
        "keep" => {}
        _ => bail!("Invalid version specification: '{}'", bump_spec),
    };
    Ok(v)
}

pub fn is_workspace_member(meta: &Metadata, crate_name: &str) -> bool {
    workspace_package_by_name(meta, crate_name).is_some()
}

pub fn package_eq_dependency(package: &Package, dependency: &Dependency) -> bool {
    package.name == dependency.name
}

pub fn workspace_package_by_name<'a>(meta: &'a Metadata, crate_name: &str) -> Option<&'a Package> {
    meta.packages
        .iter()
        .find(|p| p.name == crate_name)
        .filter(|p| meta.workspace_members.iter().any(|m| m == &p.id))
}

pub fn workspace_package_by_id<'a>(meta: &'a Metadata, id: &PackageId) -> Option<&'a Package> {
    meta.packages
        .iter()
        .find(|p| &p.id == id)
        .filter(|p| meta.workspace_members.iter().any(|m| m == &p.id))
}

pub fn package_by_name<'a>(meta: &'a Metadata, name: &str) -> anyhow::Result<&'a Package> {
    meta.packages
        .iter()
        .find(|p| p.name == name)
        .ok_or_else(|| anyhow!("workspace member '{}' must be a listed package", name))
}

pub fn package_for_dependency<'a>(meta: &'a Metadata, dep: &Dependency) -> &'a Package {
    meta.packages
        .iter()
        .find(|p| package_eq_dependency(p, dep))
        .expect("dependency always available as package")
}

pub fn names_and_versions(publishees: &[(&Package, String)]) -> String {
    publishees
        .iter()
        .map(|(p, nv)| format!("{} v{}", p.name, nv))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn package_by_id<'a>(meta: &'a Metadata, id: &PackageId) -> &'a Package {
    meta.packages
        .iter()
        .find(|p| &p.id == id)
        .expect("workspace members are in packages")
}

pub fn tag_name_for(package: &str, version: &str) -> String {
    format!("{}-v{}", package, version)
}
