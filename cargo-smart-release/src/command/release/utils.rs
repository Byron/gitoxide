use anyhow::anyhow;
use cargo_metadata::{Dependency, Metadata, Package, PackageId};

pub fn will(not_really: bool) -> &'static str {
    if not_really {
        "WOULD"
    } else {
        "Will"
    }
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
        .ok_or_else(|| anyhow!("workspace member must be a listed package: '{}'", name))
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
