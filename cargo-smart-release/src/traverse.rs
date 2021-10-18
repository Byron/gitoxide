use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

use crate::utils::package_eq_dependency;
use crate::version::{Bump, BumpSpec};
use crate::{
    git,
    traverse::dependency::VersionAdjustment,
    utils::{is_pre_release_version, package_by_id, package_by_name, workspace_package_by_dependency},
    version, Context,
};

pub mod dependency {
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
    pub enum VersionAdjustment {
        Changed {
            change: git::PackageChangeKind,
            bump: version::Bump,
        },
        Breakage {
            bump: version::Bump,
            /// Set if there is a change at all, which might not be the case for previously skipped crates.
            change: Option<git::PackageChangeKind>,
            /// The direct dependency causing the breakage because it's breaking itself
            causing_dependency_names: Vec<String>,
        },
    }

    impl VersionAdjustment {
        pub fn bump(&self) -> &version::Bump {
            match self {
                VersionAdjustment::Breakage { bump, .. } | VersionAdjustment::Changed { bump, .. } => bump,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Mode {
        ToBePublished {
            adjustment: VersionAdjustment,
        },
        /// Won't be published but manifest might have to be fixed if a version bump is present.
        Skipped {
            reason: SkippedReason,
            adjustment: Option<VersionAdjustment>,
        },
        /// One of our dependencies will see a version adjustment, which we must update in our manifest
        ManifestNeedsUpdateDueToDependencyChange,
    }

    impl Mode {
        pub fn version_adjustment_bump(&self) -> Option<&version::Bump> {
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

#[derive(Clone, Debug)]
pub struct Dependency<'meta> {
    pub package: &'meta Package,
    pub kind: dependency::Kind,
    pub mode: dependency::Mode,
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
        forward_propagate_breaking_changes_for_publishing(ctx, &mut crates)?;
        forward_propagate_breaking_changes_for_manifest_updates(&ctx, &mut crates)?;
    }
    crates.extend(find_workspace_crates_depending_on_adjusted_crates(ctx, &crates));
    Ok(crates)
}

fn forward_propagate_breaking_changes_for_manifest_updates<'meta>(
    ctx: &'meta Context,
    crates: &mut Vec<Dependency<'meta>>,
) -> anyhow::Result<()> {
    let mut non_publishing_crates_with_safety_bumps = Vec::new();
    let mut backing = crates
        .iter()
        .filter(
            |c| matches!(&c.mode, dependency::Mode::ToBePublished { adjustment } if adjustment.bump().is_breaking()),
        )
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let workspace_packages: Vec<_> = ctx
        .meta
        .workspace_members
        .iter()
        .map(|wmid| package_by_id(&ctx.meta, wmid))
        .filter(|p| p.publish.is_none()) // will publish, non-publishing ones need no safety bumps
        .collect();
    let mut set_to_expand_from = &backing;
    let mut seen = BTreeSet::default();
    loop {
        let mut new_crates_this_round = Vec::<Dependency<'_>>::new();
        for dependee in set_to_expand_from {
            for dependant in workspace_packages.iter().filter(|p| {
                p.dependencies
                    .iter()
                    .any(|dep| package_eq_dependency(dependee.package, dep))
            }) {
                if seen.contains(&&dependant.id) {
                    continue;
                }
                seen.insert(&dependant.id);
                let bump_of_crate_with_that_name = crates.iter().find_map(|c| {
                    (c.package.id == dependant.id)
                        .then(|| c.mode.version_adjustment_bump())
                        .flatten()
                });
                debug_assert!(
                    bump_of_crate_with_that_name.map(|b| b.is_breaking()).unwrap_or(true),
                    "BUG: Found a crate for '{}' that shouldn't be in our set yet",
                    dependant.name
                );
                let bump = breaking_version_bump(ctx, dependant)?;
                if bump.next_release_changes_manifest() && bump_of_crate_with_that_name.is_none() {
                    new_crates_this_round.push(Dependency {
                        package: dependant,
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        mode: dependency::Mode::Skipped {
                            reason: dependency::SkippedReason::Unchanged,
                            adjustment: Some(dependency::VersionAdjustment::Breakage {
                                bump,
                                change: None,
                                causing_dependency_names: vec![dependee.package.name.to_owned()],
                            }),
                        },
                    });
                }
            }
        }

        if new_crates_this_round.is_empty() {
            break;
        }
        non_publishing_crates_with_safety_bumps.extend(new_crates_this_round.iter().cloned());
        backing = new_crates_this_round;
        set_to_expand_from = &backing;
    }
    crates.extend(non_publishing_crates_with_safety_bumps);
    Ok(())
}

fn forward_propagate_breaking_changes_for_publishing(
    ctx: &Context,
    mut crates: &mut Vec<Dependency<'_>>,
) -> anyhow::Result<()> {
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
            crates,
            (idx, starting_crate_for_backward_search),
            &mut seen,
            &mut edits,
        );
        seen.insert(idx);
    }
    for edit_for_publish in edits {
        edit_for_publish.apply(&mut crates, ctx)?;
    }
    Ok(())
}

struct EditForPublish {
    crates_idx: usize,
    causing_dependency_indices: Vec<usize>,
}

impl EditForPublish {
    fn from(idx: usize, causing_dependency_indices: Vec<usize>) -> Self {
        EditForPublish {
            crates_idx: idx,
            causing_dependency_indices,
        }
    }

    fn apply(self, crates: &mut [Dependency<'_>], ctx: &Context) -> anyhow::Result<()> {
        let causing_dependency_names = self
            .causing_dependency_indices
            .into_iter()
            .map(|idx| crates[idx].package.name.clone())
            .collect();
        let dep_mut = &mut crates[self.crates_idx];
        let breaking_bump = breaking_version_bump(ctx, dep_mut.package)?;
        match &mut dep_mut.mode {
            dependency::Mode::Skipped {
                adjustment: maybe_adjustment,
                ..
            } => {
                let adjustment = match maybe_adjustment.take() {
                    Some(mut adjustment) => {
                        make_breaking(&mut adjustment, breaking_bump, causing_dependency_names);
                        adjustment
                    }
                    None => dependency::VersionAdjustment::Breakage {
                        bump: breaking_bump,
                        causing_dependency_names,
                        change: None,
                    },
                };
                dep_mut.mode = dependency::Mode::ToBePublished { adjustment };
            }
            dependency::Mode::ToBePublished { adjustment, .. } => {
                make_breaking(adjustment, breaking_bump, causing_dependency_names);
            }
            dependency::Mode::ManifestNeedsUpdateDueToDependencyChange => {
                unreachable!("BUG: these shouldn't exist at this point")
            }
        }
        Ok(())
    }
}

fn breaking_version_bump(ctx: &Context, package: &Package) -> anyhow::Result<Bump> {
    let breaking_spec = is_pre_release_version(&package.version)
        .then(|| BumpSpec::Minor)
        .unwrap_or(BumpSpec::Major);
    version::bump_package_with_spec(package, breaking_spec, ctx, true)
}

fn make_breaking(adjustment: &mut VersionAdjustment, breaking_bump: version::Bump, breaking_crate_names: Vec<String>) {
    match adjustment {
        VersionAdjustment::Breakage { .. } => {
            unreachable!("BUG: should never try to adjust a package for breaking changes twice")
        }
        VersionAdjustment::Changed { change, bump } => {
            bump.next_release = breaking_bump.next_release;
            *adjustment = VersionAdjustment::Breakage {
                bump: bump.clone(),
                change: Some(change.clone()),
                causing_dependency_names: breaking_crate_names,
            }
        }
    }
}

fn find_safety_bump_edits_backwards_from_crates_for_publish(
    crates: &[Dependency<'_>],
    start: (usize, &Dependency<'_>),
    seen: &mut BTreeSet<usize>,
    edits: &mut Vec<EditForPublish>,
) -> Option<Vec<usize>> {
    let (current_idx, current) = start;
    let mut breaking_indices = Vec::new();
    for (dep_idx, dep) in current.package.dependencies.iter().filter_map(|dep| {
        crates
            .iter()
            .enumerate()
            .find(|(_, c)| package_eq_dependency(c.package, dep))
    }) {
        if seen.contains(&dep_idx) {
            continue;
        }
        match dep.mode.version_adjustment_bump() {
            Some(dep_bump) if dep_bump.is_breaking() => {
                if !edits.iter().any(|e| e.crates_idx == current_idx) {
                    edits.push(EditForPublish::from(current_idx, vec![dep_idx]));
                }
                if !breaking_indices.contains(&dep_idx) {
                    breaking_indices.push(dep_idx);
                }
            }
            _ => {
                seen.insert(dep_idx);
                let root_causes_of_breakage =
                    find_safety_bump_edits_backwards_from_crates_for_publish(crates, (dep_idx, dep), seen, edits);
                if let Some(breaking_package_indices) = root_causes_of_breakage {
                    if !edits.iter().any(|e| e.crates_idx == current_idx) {
                        edits.push(EditForPublish::from(current_idx, breaking_package_indices.clone()));
                    }
                    for idx in breaking_package_indices {
                        if !breaking_indices.contains(&idx) {
                            breaking_indices.push(idx);
                        }
                    }
                }
            }
        }
    }
    (!breaking_indices.is_empty()).then(|| breaking_indices)
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
            mode: dependency::Mode::ManifestNeedsUpdateDueToDependencyChange,
        })
        .collect()
}
