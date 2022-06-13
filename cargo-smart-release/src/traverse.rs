use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Package, PackageId};

use crate::{
    git,
    traverse::dependency::{ManifestAdjustment, VersionAdjustment},
    utils::{
        is_pre_release_version, package_by_id, package_by_name, package_eq_dependency_ignore_dev_without_version,
        workspace_package_by_dependency,
    },
    version,
    version::{Bump, BumpSpec},
    Context,
};

pub mod dependency {
    use crate::{git, version};

    /// Skipped crates are always dependent ones
    #[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, PartialEq)]
    pub enum NoPublishReason {
        Unchanged,
        DeniedAutopublishOfProductionCrate,
        PublishDisabledInManifest,
        BreakingChangeCausesManifestUpdate,
    }

    impl std::fmt::Display for NoPublishReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                NoPublishReason::PublishDisabledInManifest => "disabled",
                NoPublishReason::DeniedAutopublishOfProductionCrate => "denied",
                NoPublishReason::Unchanged => "unchanged",
                NoPublishReason::BreakingChangeCausesManifestUpdate => "dep-breaking",
            })
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Kind {
        /// Initially selected by user
        UserSelection,
        /// A changed dependency of the user selected crate that thus needs publishing
        DependencyOrDependentOfUserSelection,
    }

    #[derive(Clone, Debug)]
    pub enum VersionAdjustment {
        /// The crate changed and should see a version change
        Changed {
            change: git::PackageChangeKind,
            bump: version::Bump,
        },
        /// One of the crates dependencies signalled breaking changes, and is published because of that.
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

    #[allow(clippy::large_enum_variant)]
    #[derive(Clone, Debug)]
    pub enum ManifestAdjustment {
        DueToDependencyChange,
        Version(VersionAdjustment),
    }

    #[derive(Clone, Debug)]
    pub enum Mode {
        ToBePublished {
            adjustment: VersionAdjustment,
        },
        /// Won't be published but manifest might have to be fixed if a version bump is present.
        NotForPublishing {
            reason: NoPublishReason,
            adjustment: Option<ManifestAdjustment>,
        },
    }

    impl Mode {
        pub fn manifest_will_change(&self) -> bool {
            matches!(
                self,
                Mode::ToBePublished { .. }
                    | Mode::NotForPublishing {
                        adjustment: Some(_),
                        ..
                    }
            )
        }
        pub fn safety_bump(&self) -> Option<&version::Bump> {
            match self {
                Mode::ToBePublished { adjustment }
                | Mode::NotForPublishing {
                    adjustment: Some(ManifestAdjustment::Version(adjustment)),
                    ..
                } => match adjustment {
                    VersionAdjustment::Breakage { bump, .. } => Some(bump),
                    VersionAdjustment::Changed { .. } => None,
                },
                _ => None,
            }
        }
        pub fn version_adjustment_bump(&self) -> Option<&version::Bump> {
            match self {
                Mode::ToBePublished { adjustment }
                | Mode::NotForPublishing {
                    adjustment: Some(ManifestAdjustment::Version(adjustment)),
                    ..
                } => Some(match adjustment {
                    VersionAdjustment::Breakage { bump, .. } | VersionAdjustment::Changed { bump, .. } => bump,
                }),
                _ => None,
            }
        }
    }
}

#[derive(Clone)]
pub struct Dependency<'meta> {
    pub package: &'meta Package,
    pub kind: dependency::Kind,
    pub mode: dependency::Mode,
}

impl<'a> std::fmt::Debug for Dependency<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Dependency");
        dbg.field("package", &self.package.id.repr);
        dbg.field("kind", &self.kind);
        dbg.field("mode", &self.mode);
        dbg.finish()
    }
}

pub struct Options {
    pub allow_auto_publish_of_stable_crates: bool,
    pub bump_when_needed: bool,
    pub isolate_dependencies_from_breaking_changes: bool,
    pub traverse_graph: bool,
}

pub fn dependencies(
    ctx: &crate::Context,
    Options {
        allow_auto_publish_of_stable_crates,
        bump_when_needed,
        isolate_dependencies_from_breaking_changes,
        traverse_graph,
    }: Options,
) -> anyhow::Result<Vec<Dependency<'_>>> {
    let mut seen = BTreeSet::new();
    let mut crates = Vec::new();
    for crate_name in &ctx.crate_names {
        let mut crates_this_round = Vec::new();
        let package = package_by_name(&ctx.meta, crate_name)?;
        if seen.contains(&&package.id) {
            continue;
        }
        if traverse_graph {
            depth_first_traversal(
                ctx,
                &mut seen,
                &mut crates_this_round,
                package,
                allow_auto_publish_of_stable_crates,
                bump_when_needed,
            )?;
        }

        match git::change_since_last_release(package, ctx)? {
            Some(user_package_change) => {
                crates_this_round.push(Dependency {
                    package,
                    kind: dependency::Kind::UserSelection,
                    mode: if package_may_be_published(package) {
                        dependency::Mode::ToBePublished {
                            adjustment: VersionAdjustment::Changed {
                                change: user_package_change,
                                bump: version::bump_package(package, ctx, bump_when_needed)?,
                            },
                        }
                    } else {
                        dependency::Mode::NotForPublishing {
                            reason: dependency::NoPublishReason::PublishDisabledInManifest,
                            adjustment: None,
                        }
                    },
                });
                seen.insert(&package.id);
            }
            None => {
                crates_this_round.push(Dependency {
                    package,
                    kind: dependency::Kind::UserSelection,
                    mode: dependency::Mode::NotForPublishing {
                        reason: dependency::NoPublishReason::Unchanged,
                        adjustment: None,
                    },
                });
            }
        }
        merge_crates(&mut crates, crates_this_round);
    }

    if isolate_dependencies_from_breaking_changes {
        forward_propagate_breaking_changes_for_publishing(
            ctx,
            &mut crates,
            bump_when_needed,
            allow_auto_publish_of_stable_crates,
        )?;
        forward_propagate_breaking_changes_for_manifest_updates(
            ctx,
            &mut crates,
            bump_when_needed,
            allow_auto_publish_of_stable_crates,
        )?;
    }
    crates.extend(find_workspace_crates_depending_on_adjusted_crates(ctx, &crates));
    Ok(crates)
}

fn merge_crates<'meta>(dest: &mut Vec<Dependency<'meta>>, src: Vec<Dependency<'meta>>) {
    if dest.is_empty() {
        *dest = src;
    } else {
        for dep in src {
            if !dest.iter().any(|dest| dest.package.id == dep.package.id) {
                dest.push(dep);
            }
        }
    }
}

fn forward_propagate_breaking_changes_for_manifest_updates<'meta>(
    ctx: &'meta Context,
    crates: &mut Vec<Dependency<'meta>>,
    bump_when_needed: bool,
    allow_auto_publish_of_stable_crates: bool,
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
        .filter(|p| package_may_be_published(p)) // will publish, non-publishing ones need no safety bumps
        .collect();
    let mut set_to_expand_from = &backing;
    let mut seen = BTreeSet::default();
    loop {
        let mut new_crates_this_round = Vec::<Dependency<'_>>::new();
        for dependee in set_to_expand_from {
            for dependant in workspace_packages.iter().filter(|p| {
                p.dependencies
                    .iter()
                    .any(|dep| package_eq_dependency_ignore_dev_without_version(dependee.package, dep))
            }) {
                if seen.contains(&&dependant.id) {
                    continue;
                }
                seen.insert(&dependant.id);
                let crate_is_known_already = crates
                    .iter()
                    .find_map(|c| {
                        (c.package.id == dependant.id)
                            .then(|| c.mode.version_adjustment_bump().map(|b| b.is_breaking()))
                    })
                    .flatten();

                let bump = breaking_version_bump(ctx, dependant, bump_when_needed)?;
                if bump.next_release_changes_manifest() {
                    if crate_is_known_already.is_some() {
                        let is_breaking = crate_is_known_already.unwrap_or(false);
                        if !is_breaking {
                            log::debug!(
                                "Wanted to mark '{}' for breaking manifest change, but its already known without breaking change.",
                                dependant.name
                            );
                        }
                    } else if is_pre_release_version(&dependant.version) || allow_auto_publish_of_stable_crates {
                        let kind = ctx
                            .crate_names
                            .contains(&dependant.name)
                            .then(|| dependency::Kind::UserSelection)
                            .unwrap_or(dependency::Kind::DependencyOrDependentOfUserSelection);
                        let adjustment = dependency::VersionAdjustment::Breakage {
                            bump,
                            change: None,
                            causing_dependency_names: vec![dependee.package.name.to_owned()],
                        };
                        new_crates_this_round.push(Dependency {
                            package: dependant,
                            kind,
                            mode: match kind {
                                dependency::Kind::UserSelection => dependency::Mode::ToBePublished { adjustment },
                                dependency::Kind::DependencyOrDependentOfUserSelection => {
                                    dependency::Mode::NotForPublishing {
                                        reason: dependency::NoPublishReason::BreakingChangeCausesManifestUpdate,
                                        adjustment: Some(ManifestAdjustment::Version(adjustment)),
                                    }
                                }
                            },
                        });
                    } else {
                        log::trace!(
                            "Ignored stable crate '{}' despite being eligible for safety bump and manifest change.",
                            dependant.name
                        );
                    }
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

fn package_may_be_published(p: &Package) -> bool {
    p.publish.is_none()
}

fn forward_propagate_breaking_changes_for_publishing(
    ctx: &Context,
    crates: &mut [Dependency<'_>],
    bump_when_needed: bool,
    allow_auto_publish_of_stable_crates: bool,
) -> anyhow::Result<()> {
    let mut previous_edits = Vec::new();
    loop {
        let mut seen_this_round = BTreeSet::default();
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
                &mut seen_this_round,
                &mut edits,
            );
            seen_this_round.insert(idx);
        }

        if edits == previous_edits {
            break;
        }

        previous_edits = edits.clone();
        for edit_for_publish in edits {
            edit_for_publish.apply(crates, ctx, bump_when_needed, allow_auto_publish_of_stable_crates)?;
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq, Clone)]
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

    fn apply(
        self,
        crates: &mut [Dependency<'_>],
        ctx: &Context,
        bump_when_needed: bool,
        allow_auto_publish_of_stable_crates: bool,
    ) -> anyhow::Result<()> {
        let causing_dependency_names = self
            .causing_dependency_indices
            .into_iter()
            .map(|idx| crates[idx].package.name.clone())
            .collect();
        let dep_mut = &mut crates[self.crates_idx];
        if is_pre_release_version(&dep_mut.package.version) || allow_auto_publish_of_stable_crates {
            let breaking_bump = breaking_version_bump(ctx, dep_mut.package, bump_when_needed)?;
            match &mut dep_mut.mode {
                dependency::Mode::NotForPublishing {
                    adjustment: maybe_adjustment,
                    ..
                } => {
                    let adjustment = match maybe_adjustment.take() {
                        Some(ManifestAdjustment::DueToDependencyChange) => {
                            unreachable!("BUG: code generating these runs later")
                        }
                        Some(ManifestAdjustment::Version(mut adjustment)) => {
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
            }
        } else {
            log::trace!(
                "Ignored stable crate '{}' despite being eligible for safety bump and publishing.",
                dep_mut.package.name
            );
        }
        Ok(())
    }
}

fn breaking_version_bump(ctx: &Context, package: &Package, bump_when_needed: bool) -> anyhow::Result<Bump> {
    let breaking_spec = is_pre_release_version(&package.version)
        .then(|| BumpSpec::Minor)
        .unwrap_or(BumpSpec::Major);
    version::bump_package_with_spec(package, breaking_spec, ctx, bump_when_needed)
}

fn make_breaking(adjustment: &mut VersionAdjustment, breaking_bump: version::Bump, breaking_crate_names: Vec<String>) {
    match adjustment {
        VersionAdjustment::Breakage { .. } => {}
        VersionAdjustment::Changed { change, bump } => {
            bump.next_release = breaking_bump.next_release;
            *adjustment = VersionAdjustment::Breakage {
                bump: bump.clone(),
                change: Some(change.clone()),
                causing_dependency_names: breaking_crate_names,
            };
        }
    }
}

fn find_safety_bump_edits_backwards_from_crates_for_publish(
    crates: &[Dependency<'_>],
    start: (usize, &Dependency<'_>),
    seen: &mut BTreeSet<usize>,
    edits: &mut Vec<EditForPublish>,
) -> Vec<usize> {
    let (current_idx, current) = start;
    let mut breaking_indices = Vec::new();
    for (dep_idx, dep) in current.package.dependencies.iter().filter_map(|dep| {
        crates
            .iter()
            .enumerate()
            .find(|(_, c)| package_eq_dependency_ignore_dev_without_version(c.package, dep))
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
                let breaking_package_indices =
                    find_safety_bump_edits_backwards_from_crates_for_publish(crates, (dep_idx, dep), seen, edits);
                if !breaking_package_indices.is_empty() {
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
    breaking_indices
}

fn depth_first_traversal<'meta>(
    ctx: &'meta crate::Context,
    seen: &mut BTreeSet<&'meta PackageId>,
    crates: &mut Vec<Dependency<'meta>>,
    root: &Package,
    allow_auto_publish_of_stable_crates: bool,
    bump_when_needed: bool,
) -> anyhow::Result<()> {
    for workspace_dependency in root
        .dependencies
        .iter()
        .filter(|d| d.kind == DependencyKind::Normal)
        .filter_map(|d| workspace_package_by_dependency(&ctx.meta, d))
    {
        if seen.contains(&&workspace_dependency.id) {
            continue;
        }
        seen.insert(&workspace_dependency.id);
        depth_first_traversal(
            ctx,
            seen,
            crates,
            workspace_dependency,
            allow_auto_publish_of_stable_crates,
            bump_when_needed,
        )?;

        crates.push(match git::change_since_last_release(workspace_dependency, ctx)? {
            Some(change) => {
                if is_pre_release_version(&workspace_dependency.version) || allow_auto_publish_of_stable_crates {
                    Dependency {
                        package: workspace_dependency,
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        mode: dependency::Mode::ToBePublished {
                            adjustment: VersionAdjustment::Changed {
                                change,
                                bump: version::bump_package(workspace_dependency, ctx, bump_when_needed)?,
                            },
                        },
                    }
                } else {
                    Dependency {
                        package: workspace_dependency,
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        mode: dependency::Mode::NotForPublishing {
                            reason: dependency::NoPublishReason::DeniedAutopublishOfProductionCrate,
                            adjustment: None,
                        },
                    }
                }
            }
            None => Dependency {
                package: workspace_dependency,
                kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                mode: dependency::Mode::NotForPublishing {
                    reason: dependency::NoPublishReason::Unchanged,
                    adjustment: None,
                },
            },
        });
    }
    Ok(())
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
            wsp.dependencies.iter().any(|d| {
                crates
                    .iter()
                    .filter(|c| c.mode.manifest_will_change())
                    .any(|c| package_eq_dependency_ignore_dev_without_version(c.package, d))
            })
        })
        .map(|wsp| Dependency {
            kind: dependency::Kind::DependencyOrDependentOfUserSelection,
            package: wsp,
            mode: dependency::Mode::NotForPublishing {
                adjustment: ManifestAdjustment::DueToDependencyChange.into(),
                reason: dependency::NoPublishReason::Unchanged,
            },
        })
        .collect()
}
