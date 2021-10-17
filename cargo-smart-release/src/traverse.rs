use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

use crate::{
    git,
    utils::{is_pre_release_version, package_by_name, workspace_package_by_name},
};

pub mod dependency {
    /// Skipped crates are always dependent ones
    #[derive(Copy, Clone, Debug)]
    pub enum SkippedReason {
        Unchanged,
        DeniedAutopublishOfProductionCrate,
    }

    #[derive(Clone, Debug)]
    pub enum Kind {
        UserSelection,
        DependencyOfUserSelection,
    }

    #[derive(Clone, Debug)]
    pub enum Mode {
        ToBePublished {
            kind: Kind,
            change_kind: crate::git::PackageChangeKind,
        },
        Skipped {
            kind: Kind,
            reason: SkippedReason,
        },
    }
}

#[derive(Debug)]
pub struct Dependency<'meta> {
    pub package: &'meta Package,
    pub mode: dependency::Mode,
}

pub fn dependencies(ctx: &crate::Context, add_production_crates: bool) -> anyhow::Result<Vec<Dependency<'_>>> {
    let mut seen = BTreeSet::new();
    let mut crates = Vec::new();
    for crate_name in &ctx.crate_names {
        let package = package_by_name(&ctx.meta, crate_name)?;
        if seen.contains(&&package.id) {
            continue;
        }
        if dependency_tree_has_link_to_existing_crate_names(&ctx.meta, package, &crates)? {
            // redo all work which includes the previous tree. Could be more efficient but that would be more complicated.
            seen.clear();
            crates.clear();
        }
        let num_crates_for_publishing_without_dependencies = crates.len();
        let current_skipped = depth_first_traversal(ctx, add_production_crates, &mut seen, &mut crates, package)?;
        crates.extend(current_skipped);

        match git::change_since_last_release(package, ctx)? {
            Some(user_package_change) => {
                crates.push(Dependency {
                    package,
                    mode: dependency::Mode::ToBePublished {
                        kind: dependency::Kind::UserSelection,
                        change_kind: user_package_change,
                    },
                });
                seen.insert(&package.id);
            }
            None => {
                let found_no_dependencies = num_crates_for_publishing_without_dependencies == crates.len();
                if found_no_dependencies {
                    crates.push(Dependency {
                        package,
                        mode: dependency::Mode::Skipped {
                            kind: dependency::Kind::UserSelection,
                            reason: dependency::SkippedReason::Unchanged,
                        },
                    });
                    continue;
                }
            }
        }
    }
    Ok(crates)
}

fn depth_first_traversal<'meta>(
    ctx: &'meta crate::Context,
    add_production_crates: bool,
    seen: &mut BTreeSet<&'meta PackageId>,
    crates: &mut Vec<Dependency<'meta>>,
    root: &Package,
) -> anyhow::Result<Vec<Dependency<'meta>>> {
    let mut skipped = Vec::new();
    for dependency in root.dependencies.iter().filter(|d| d.kind == DependencyKind::Normal) {
        let workspace_dependency = match workspace_package_by_name(&ctx.meta, &dependency.name) {
            Some(p) => p,
            None => continue,
        };
        if seen.contains(&&workspace_dependency.id) {
            continue;
        }
        seen.insert(&workspace_dependency.id);
        skipped.extend(depth_first_traversal(
            ctx,
            add_production_crates,
            seen,
            crates,
            workspace_dependency,
        )?);
        if let Some(change) = git::change_since_last_release(workspace_dependency, ctx)? {
            if is_pre_release_version(&workspace_dependency.version) || add_production_crates {
                crates.push(Dependency {
                    package: workspace_dependency,
                    mode: dependency::Mode::ToBePublished {
                        kind: dependency::Kind::DependencyOfUserSelection,
                        change_kind: change,
                    },
                });
            } else {
                crates.push(Dependency {
                    package: workspace_dependency,
                    mode: dependency::Mode::Skipped {
                        kind: dependency::Kind::DependencyOfUserSelection,
                        reason: dependency::SkippedReason::DeniedAutopublishOfProductionCrate,
                    },
                });
            }
        } else {
            skipped.push(Dependency {
                package: workspace_dependency,
                mode: dependency::Mode::Skipped {
                    kind: dependency::Kind::DependencyOfUserSelection,
                    reason: dependency::SkippedReason::Unchanged,
                },
            });
        }
    }
    Ok(skipped)
}

fn dependency_tree_has_link_to_existing_crate_names(
    meta: &Metadata,
    root: &Package,
    existing: &[Dependency<'_>],
) -> anyhow::Result<bool> {
    let mut dependencies = vec![root];
    let mut seen = BTreeSet::new();
    while let Some(package) = dependencies.pop() {
        if !seen.insert(&package.id) {
            continue;
        }
        if existing.iter().any(|d| d.package.id == package.id) {
            return Ok(true);
        }
        dependencies.extend(
            package
                .dependencies
                .iter()
                .filter_map(|dep| workspace_package_by_name(meta, &dep.name)),
        )
    }
    Ok(false)
}
