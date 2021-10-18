use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

use crate::utils::package_eq_dependency;
use crate::version::rhs_is_breaking_bump_for_lhs;
use crate::{
    git,
    traverse::dependency::VersionAdjustment,
    utils::{is_pre_release_version, package_by_id, package_by_name, workspace_package_by_dependency},
    version, Context,
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
            /// Set if there is a change at all, which might not be the case for previously skipped crates.
            change: Option<git::PackageChangeKind>,
            /// The direct dependency causing the breakage because it's breaking itself
            direct_dependency: &'meta Package,
        },
    }

    impl<'meta> VersionAdjustment<'meta> {
        pub fn bump(&self) -> &version::bump::Outcome {
            match self {
                VersionAdjustment::Breakage { bump, .. } | VersionAdjustment::Changed { bump, .. } => bump,
            }
        }
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
        pub fn version_adjustment_bump(&self) -> Option<&version::bump::Outcome> {
            match self {
                Mode::ToBePublished { adjustment }
                | Mode::Skipped {
                    adjustment: Some(adjustment),
                    ..
                } => Some(match adjustment {
                    VersionAdjustment::Breakage { bump, .. } | VersionAdjustment::Changed { bump, .. } => bump,
                }),
                _ => None,
            }
        }
        pub fn has_version_adjustment(&self) -> bool {
            self.version_adjustment_bump().is_some()
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
    isolate_dependencies_from_breaking_changes: bool,
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

    if isolate_dependencies_from_breaking_changes {
        // TODO: before this, traverse forward through all dependencies from our crates with breaking changes
        //       and propagate such breaking change, either lifting skipped crates to those with adjustment,
        //       If about to be published crates can reach these newly added crates, they must be made publishable too
        let mut seen = BTreeSet::default();
        let mut edits = Vec::new();
        // skipped don't have version bumps, we don't have manifest updates yet
        for (idx, starting_crate_for_backward_search) in crates
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, c)| matches!(c.mode, dependency::Mode::ToBePublished { .. }))
        {
            find_safety_bump_edits_backwards_from_crates_for_publish(
                &crates,
                (idx, starting_crate_for_backward_search),
                &mut seen,
                &mut edits,
            );
            seen.insert(idx);
        }
        for _edit in edits {
            todo!("apply edit");
        }

        // TODO: forward traversal from low-level crates upward if (similarly to what's done already in manifest) to find
        //       crates that need a version adjustment, without publishing, to be added to the list
    }
    crates.extend(find_workspace_crates_depending_on_adjusted_crates(ctx, &crates));
    Ok(crates)
}

struct EditForPublish {
    crates_idx: usize,
    bump: Option<version::bump::Outcome>,
}

impl EditForPublish {
    fn breaking_release(idx: usize, _existing: Option<&version::bump::Outcome>) -> Self {
        EditForPublish {
            crates_idx: idx,
            bump: todo!("breaking change"),
        }
    }
}

fn find_safety_bump_edits_backwards_from_crates_for_publish(
    crates: &[Dependency<'_>],
    start: (usize, &Dependency<'_>),
    seen: &mut BTreeSet<usize>,
    edits: &mut Vec<EditForPublish>,
) -> bool {
    let (current_idx, current) = start;
    for (dep_idx, dep) in current.package.dependencies.iter().filter_map(|dep| {
        crates
            .iter()
            .enumerate()
            .find(|(_, c)| package_eq_dependency(c.package, dep))
    }) {
        if seen.contains(&dep_idx) {
            return false;
        }
        seen.insert(dep_idx);

        match dep.mode.version_adjustment_bump() {
            Some(dep_bump)
                if rhs_is_breaking_bump_for_lhs(
                    &dep.package.version,
                    dep_bump.next_release.as_ref().expect("only valid versions here"),
                ) =>
            {
                edits.push(EditForPublish::breaking_release(
                    current_idx,
                    current.mode.version_adjustment_bump(),
                ));
                return true;
            }
            _ => {
                let has_breaking_in_dependencies =
                    find_safety_bump_edits_backwards_from_crates_for_publish(crates, (dep_idx, dep), seen, edits);
                if has_breaking_in_dependencies {
                    edits.push(EditForPublish::breaking_release(
                        current_idx,
                        current.mode.version_adjustment_bump(),
                    ));
                    return true;
                }
            }
        }
    }
    false
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
        let workspace_dependency = match workspace_package_by_dependency(&ctx.meta, dependency) {
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
                .filter_map(|dep| workspace_package_by_dependency(meta, dep)),
        )
    }
    Ok(false)
}

fn find_workspace_crates_depending_on_adjusted_crates<'meta>(
    ctx: &'meta Context,
    crates: &[Dependency<'_>],
) -> Vec<Dependency<'meta>> {
    ctx.meta
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
        .collect()
}
