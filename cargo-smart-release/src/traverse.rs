use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

use crate::{
    git,
    traverse::dependency::VersionAdjustment,
    utils::{is_pre_release_version, package_by_id, package_by_name, workspace_package_by_name},
    version,
};

pub mod dependency {
    use cargo_metadata::Package;

    use crate::{git, version};

    /// Skipped crates are always dependent ones
    #[derive(Copy, Clone, Debug)]
    pub enum SkippedReason {
        Unchanged,
        DeniedAutopublishOfProductionCrate,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Kind {
        /// Initially selected by user
        UserSelection,
        /// A changed dependency of the user selected crate that thus needs publishing
        DependencyOrDependentOfUserSelection,
    }

    #[derive(Clone, Debug)]
    pub enum VersionAdjustment<'meta> {
        Changed {
            change: git::PackageChangeKind,
            bump: version::bump::Outcome,
        },
        Breakage {
            bump: version::bump::Outcome,
            change: Option<git::PackageChangeKind>,
            /// The direct dependency causing the breakage because it's breaking itself
            direct_dependency: &'meta Package,
        },
    }

    #[derive(Clone, Debug)]
    pub enum Mode<'meta> {
        ToBePublished {
            adjustment: VersionAdjustment<'meta>,
        },
        /// Won't be published but manifest might have to be fixed if a version bump is present.
        Skipped {
            reason: SkippedReason,
            adjustment: Option<VersionAdjustment<'meta>>,
        },
        /// One of our dependencies will see a version adjustment, which we must update in our manifest
        ManifestNeedsUpdate,
    }

    impl<'meta> Mode<'meta> {
        pub fn has_version_adjustment(&self) -> bool {
            matches!(
                self,
                Mode::ToBePublished { .. }
                    | Mode::Skipped {
                        adjustment: Some(_),
                        ..
                    }
            )
        }
    }
}

#[derive(Debug)]
pub struct Dependency<'meta> {
    pub package: &'meta Package,
    pub kind: dependency::Kind,
    pub mode: dependency::Mode<'meta>,
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
                    kind: dependency::Kind::UserSelection,
                    mode: dependency::Mode::ToBePublished {
                        adjustment: VersionAdjustment::Changed {
                            change: user_package_change,
                            bump: version::bump_package(package, ctx, bump_when_needed)?,
                        },
                    },
                });
                seen.insert(&package.id);
            }
            None => {
                let found_no_dependencies = num_crates_for_publishing_without_dependencies == crates.len();
                if found_no_dependencies {
                    crates.push(Dependency {
                        package,
                        kind: dependency::Kind::UserSelection,
                        mode: dependency::Mode::Skipped {
                            reason: dependency::SkippedReason::Unchanged,
                            adjustment: None,
                        },
                    });
                    continue;
                }
            }
        }
    }

    let crates_for_manifest_update = ctx
        .meta
        .workspace_members
        .iter()
        .map(|id| package_by_id(&ctx.meta, id))
        .filter(|wsp| crates.iter().all(|c| c.package.id != wsp.id))
        .filter(|wsp| {
            wsp.dependencies
                .iter()
                .any(|d| crates.iter().any(|c| c.package.name == d.name))
        })
        .map(|wsp| Dependency {
            kind: dependency::Kind::DependencyOrDependentOfUserSelection,
            package: wsp,
            mode: dependency::Mode::ManifestNeedsUpdate,
        })
        .collect::<Vec<_>>();
    crates.extend(crates_for_manifest_update);
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
                    kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                    mode: dependency::Mode::ToBePublished {
                        adjustment: VersionAdjustment::Changed {
                            change,
                            bump: version::bump_package(workspace_dependency, ctx, bump_when_needed)?,
                        },
                    },
                });
            } else {
                crates.push(Dependency {
                    package: workspace_dependency,
                    kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                    mode: dependency::Mode::Skipped {
                        reason: dependency::SkippedReason::DeniedAutopublishOfProductionCrate,
                        adjustment: None,
                    },
                });
            }
        } else {
            skipped.push(Dependency {
                package: workspace_dependency,
                kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                mode: dependency::Mode::Skipped {
                    reason: dependency::SkippedReason::Unchanged,
                    adjustment: None,
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
