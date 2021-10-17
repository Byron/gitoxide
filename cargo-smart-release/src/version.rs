use anyhow::bail;
use cargo_metadata::Package;
use semver::{BuildMetadata, Prerelease, Version};

use crate::Context;

#[derive(Copy, Clone)]
pub enum BumpSpec {
    Auto,
    Keep,
    Patch,
    Minor,
    Major,
}

impl std::fmt::Display for BumpSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BumpSpec::Auto => "auto",
            BumpSpec::Keep => "no",
            BumpSpec::Patch => "patch",
            BumpSpec::Minor => "minor",
            BumpSpec::Major => "major",
        })
    }
}

#[allow(clippy::ptr_arg)]
pub(crate) fn select_publishee_bump_spec(name: &String, ctx: &Context) -> BumpSpec {
    if ctx.crate_names.contains(name) {
        ctx.bump
    } else {
        ctx.bump_dependencies
    }
}

/// Returns true if this would be a breaking change for `v`.
fn bump_major_minor_patch(v: &mut semver::Version, bump_spec: BumpSpec) -> bool {
    use BumpSpec::*;
    match bump_spec {
        Major => {
            v.major += 1;
            v.minor = 0;
            v.patch = 0;
            v.pre = Prerelease::EMPTY;
            true
        }
        Minor => {
            v.minor += 1;
            v.patch = 0;
            v.pre = Prerelease::EMPTY;
            is_pre_release(v)
        }
        Patch => {
            v.patch += 1;
            v.pre = Prerelease::EMPTY;
            false
        }
        Keep | Auto => unreachable!("BUG: auto mode or keep are unsupported"),
    }
}

pub mod bump {
    #[derive(Clone, Debug)]
    pub enum Error {
        LatestReleaseMoreRecentThanDesiredOne(semver::Version),
    }
    #[derive(Clone, Debug)]
    pub struct Outcome {
        pub next_release: Result<semver::Version, Error>,
        pub latest_release: Option<semver::Version>,
        pub desired_release: semver::Version,
    }
}

pub(crate) fn bump_package(package: &Package, ctx: &Context, bump_when_needed: bool) -> anyhow::Result<bump::Outcome> {
    let mut v = package.version.clone();
    let bump_spec = select_publishee_bump_spec(&package.name, ctx);
    use BumpSpec::*;
    let package_version_must_be_breaking = match bump_spec {
        Major | Minor | Patch => bump_major_minor_patch(&mut v, bump_spec),
        Keep => false,
        Auto => {
            let segments = crate::git::history::crate_ref_segments(
                package,
                ctx,
                ctx.history.as_ref().expect("BUG: assure history is set here"),
                crate::git::history::SegmentScope::Unreleased,
            )?;
            assert_eq!(
                segments.len(),
                1,
                "there should be exactly one section, the 'unreleased' one"
            );
            let unreleased = &segments[0];
            if unreleased.history.is_empty() {
                log::trace!("{}: no changes since the last release. Version unchanged", package.name);
                false
            } else if unreleased.history.iter().any(|item| item.message.breaking) {
                let is_breaking = if is_pre_release(&v) {
                    bump_major_minor_patch(&mut v, Minor)
                } else {
                    bump_major_minor_patch(&mut v, Major)
                };
                assert!(is_breaking, "BUG: breaking changes are…breaking :D");
                is_breaking
            } else if unreleased
                .history
                .iter()
                .any(|item| item.message.kind.map(|kind| kind == "feat").unwrap_or(false))
            {
                let is_breaking = if is_pre_release(&v) {
                    bump_major_minor_patch(&mut v, Patch)
                } else {
                    bump_major_minor_patch(&mut v, Minor)
                };
                assert!(!is_breaking, "BUG: new features are never breaking");
                is_breaking
            } else {
                let is_breaking = bump_major_minor_patch(&mut v, Patch);
                assert!(!is_breaking, "BUG: patch releases are never breaking");
                false
            }
        }
    };
    let desired_release = v;
    let (latest_release, next_release) = match ctx.crates_index.crate_(&package.name) {
        Some(published_crate) => {
            let latest_release = semver::Version::parse(published_crate.latest_version().version())
                .expect("valid version in crate index");
            let next_release = if latest_release >= desired_release {
                Err(bump::Error::LatestReleaseMoreRecentThanDesiredOne(
                    latest_release.clone(),
                ))
            } else {
                let mut next_release = desired_release.clone();
                if bump_when_needed && package.version > latest_release && desired_release != package.version {
                    if package_version_must_be_breaking {
                        if rhs_is_breaking_bump_for_lhs(&latest_release, &package.version) {
                            next_release = package.version.clone();
                        }
                    } else {
                        next_release = package.version.clone();
                    };
                }
                Ok(next_release)
            };
            (Some(latest_release), next_release)
        }
        None => (
            None,
            Ok(if bump_when_needed {
                package.version.clone()
            } else {
                desired_release.clone()
            }),
        ),
    };
    Ok(bump::Outcome {
        next_release,
        desired_release,
        latest_release,
    })
}

pub(crate) fn bump(
    publishee: &Package,
    bump_spec: BumpSpec,
    ctx: &Context,
    bump_when_needed: bool,
) -> anyhow::Result<Version> {
    let mut v = publishee.version.clone();
    use BumpSpec::*;
    let package_version_must_be_breaking = match bump_spec {
        Major | Minor | Patch => bump_major_minor_patch(&mut v, bump_spec),
        Keep => false,
        Auto => {
            let segments = crate::git::history::crate_ref_segments(
                publishee,
                ctx,
                ctx.history.as_ref().expect("BUG: assure history is set here"),
                crate::git::history::SegmentScope::Unreleased,
            )?;
            assert_eq!(
                segments.len(),
                1,
                "there should be exactly one section, the 'unreleased' one"
            );
            let unreleased = &segments[0];
            if unreleased.history.is_empty() {
                log::info!(
                    "{}: no changes since the last release. Version unchanged",
                    publishee.name
                );
                false
            } else if unreleased.history.iter().any(|item| item.message.breaking) {
                let (is_breaking, level) = if is_pre_release(&v) {
                    (bump_major_minor_patch(&mut v, Minor), "minor")
                } else {
                    (bump_major_minor_patch(&mut v, Major), "major")
                };
                assert!(is_breaking, "BUG: breaking changes are…breaking :D");
                log::info!(
                    "Auto-bumped '{}' {} version to {} from {} to signal breaking changes.",
                    publishee.name,
                    level,
                    v,
                    publishee.version
                );
                is_breaking
            } else if unreleased
                .history
                .iter()
                .any(|item| item.message.kind.map(|kind| kind == "feat").unwrap_or(false))
            {
                let (is_breaking, level) = if is_pre_release(&v) {
                    (bump_major_minor_patch(&mut v, Patch), "patch")
                } else {
                    (bump_major_minor_patch(&mut v, Minor), "minor")
                };
                assert!(!is_breaking, "BUG: new features are never breaking");
                log::info!(
                    "Auto-bumped '{}' {} version to {} from {} to signal new features due to 'feat:' in commit message.",
                    publishee.name,
                    level,
                    v,
                    publishee.version
                );
                is_breaking
            } else {
                let is_breaking = bump_major_minor_patch(&mut v, Patch);
                assert!(!is_breaking, "BUG: patch releases are never breaking");
                log::info!(
                    "Auto-bumped '{}' patch version to {} from {}.",
                    publishee.name,
                    v,
                    publishee.version
                );
                false
            }
        }
    };

    let new_version = v;
    let verbose = true;
    let assume_it_will_be_published_for_log_messages = true;
    smallest_necessary_version_relative_to_crates_index(
        publishee,
        new_version,
        ctx,
        bump_when_needed,
        verbose,
        assume_it_will_be_published_for_log_messages,
        package_version_must_be_breaking,
    )
}

fn smallest_necessary_version_relative_to_crates_index(
    package: &Package,
    mut new_version: Version,
    ctx: &Context,
    bump_when_needed: bool,
    verbose: bool,
    will_be_published: bool,
    package_version_must_be_breaking: bool,
) -> anyhow::Result<Version> {
    match ctx.crates_index.crate_(&package.name) {
        Some(published_crate) => {
            let latest_published_version = semver::Version::parse(published_crate.latest_version().version())?;
            if latest_published_version >= new_version {
                bail!(
                    "Latest published version of '{}' is {}, the new version is {}. Consider using --bump <level> or --bump-dependencies <level> or update the index with --update-crates-index.",
                    package.name,
                    published_crate.latest_version().version(),
                    new_version
                );
            }
            if bump_when_needed && package.version > latest_published_version && new_version != package.version {
                let verbose = if package_version_must_be_breaking {
                    if rhs_is_breaking_bump_for_lhs(&latest_published_version, &package.version) {
                        new_version = package.version.clone();
                        verbose
                    } else {
                        false
                    }
                } else {
                    new_version = package.version.clone();
                    verbose
                };
                if new_version > package.version {
                    if verbose {
                        log::info!(
                            "Using manifest version {} of crate '{}' instead of new version {} as it is sufficient to succeed latest published version {}.",
                            package.version,
                            package.name,
                            new_version,
                            latest_published_version
                        );
                    }
                } else if verbose {
                    log::info!(
                        "Using manifest version {} of crate '{}' as it is sufficient to succeed latest published version {}.",
                        package.version,
                        package.name,
                        latest_published_version
                    );
                }
            }
        }
        None => {
            if bump_when_needed && new_version > package.version {
                if verbose {
                    log::info!(
                        "Using current version {} of crate {} instead of bumped one {}.",
                        package.version,
                        package.name,
                        new_version
                    );
                }
                new_version = package.version.clone();
            }
            if verbose && will_be_published {
                log::info!("Congratulations for the new release of '{}' 🎉", package.name);
            }
        }
    };
    Ok(new_version)
}

pub(crate) fn is_pre_release(semver: &Version) -> bool {
    crate::utils::is_pre_release_version(semver)
}

pub(crate) fn conservative_dependent_version(
    publishee: &Package,
    new_publishee_version: &str,
    dependent: &Package,
    ctx: &Context,
    bump_when_needed: bool,
    verbose: bool,
) -> Option<Version> {
    let new_publishee_version: Version = new_publishee_version.parse().expect("new versions are always valid");
    if !rhs_is_breaking_bump_for_lhs(&publishee.version, &new_publishee_version) {
        return None;
    }
    let new_dependent_version = breaking_version_bump(&dependent.version);
    smallest_necessary_version_relative_to_crates_index(
        dependent,
        new_dependent_version,
        ctx,
        bump_when_needed,
        verbose,
        false,
        true,
    )
    .ok()
}

fn breaking_version_bump(v: &Version) -> Version {
    let (major, minor, patch) = match (v.major, v.minor, v.patch) {
        (0, 0, patch) => (0, 0, patch + 1),
        (0, minor, _) => (0, minor + 1, 0),
        (major, minor, _) => (major, minor + 1, 0),
    };
    Version {
        major,
        minor,
        patch,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

pub(crate) fn rhs_is_breaking_bump_for_lhs(lhs: &Version, rhs: &Version) -> bool {
    rhs.major > lhs.major || rhs.minor > lhs.minor
}
