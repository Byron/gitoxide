use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

use crate::{
    git,
    utils::{is_pre_release_version, package_by_name, workspace_package_by_name},
    version,
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
        /// Initially selected by user
        UserSelection,
        // /// A dependency of the user selection, added because it needs a breaking version bump. It may also have changed.
        // DependencyOfUserSelectionForBreakingReleaseSafety,
        /// A changed dependency of the user selected crate that thus needs publishing
        DependencyOfUserSelection,
    }

    #[derive(Clone, Debug)]
    pub enum Mode {
        ToBePublished {
            kind: Kind,
            change_kind: crate::git::PackageChangeKind,
            /// The version suitable for the upcoming release. Maybe the same as in the current manifest as the latter already
            /// is sufficient to fulfill our constraints.
            next_release_version: semver::Version,
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

pub fn dependencies(
    ctx: &crate::Context,
    add_production_crates: bool,
    bump_when_needed: bool,
) -> anyhow::Result<Vec<Dependency<'_>>> {
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
        let current_skipped = depth_first_traversal(
            ctx,
            &mut seen,
            &mut crates,
            package,
            add_production_crates,
            bump_when_needed,
        )?;
        crates.extend(current_skipped);

        match git::change_since_last_release(package, ctx)? {
            Some(user_package_change) => {
                crates.push(Dependency {
                    package,
                    mode: dependency::Mode::ToBePublished {
                        kind: dependency::Kind::UserSelection,
                        change_kind: user_package_change,
                        next_release_version: version::bump_package(package, ctx, bump_when_needed)?,
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
    seen: &mut BTreeSet<&'meta PackageId>,
    crates: &mut Vec<Dependency<'meta>>,
    root: &Package,
    add_production_crates: bool,
    bump_when_needed: bool,
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
            seen,
            crates,
            workspace_dependency,
            add_production_crates,
            bump_when_needed,
        )?);
        if let Some(change) = git::change_since_last_release(workspace_dependency, ctx)? {
            if is_pre_release_version(&workspace_dependency.version) || add_production_crates {
                crates.push(Dependency {
                    package: workspace_dependency,
                    mode: dependency::Mode::ToBePublished {
                        kind: dependency::Kind::DependencyOfUserSelection,
                        change_kind: change,
                        next_release_version: version::bump_package(workspace_dependency, ctx, bump_when_needed)?,
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
