use cargo_metadata::Package;
use semver::{Prerelease, Version};

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

#[derive(Clone, Debug)]
pub struct Bump {
    pub next_release: semver::Version,
    /// The current version of the crate as read from Cargo.toml.
    pub package_version: semver::Version,
    /// The latest released version of the package, as read from the crates-index.
    pub latest_release: Option<semver::Version>,
    /// The computed version, for example based on a user version bump or a computed version bump.
    pub desired_release: semver::Version,
}

impl Bump {
    pub(crate) fn next_release_changes_manifest(&self) -> bool {
        self.next_release > self.package_version
    }
    pub(crate) fn is_breaking(&self) -> bool {
        rhs_is_breaking_bump_for_lhs(&self.package_version, &self.next_release)
    }
}

pub(crate) fn bump_package_with_spec(
    package: &Package,
    bump_spec: BumpSpec,
    ctx: &Context,
    bump_when_needed: bool,
) -> anyhow::Result<Bump> {
    let mut v = package.version.clone();
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
            let latest_release = semver::Version::parse(published_crate.most_recent_version().version())
                .expect("valid version in crate index");
            let next_release = if latest_release >= desired_release {
                desired_release.clone()
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
                next_release
            };
            (Some(latest_release), next_release)
        }
        None => (
            None,
            if bump_when_needed {
                package.version.clone()
            } else {
                desired_release.clone()
            },
        ),
    };
    Ok(Bump {
        next_release,
        package_version: package.version.clone(),
        desired_release,
        latest_release,
    })
}

pub(crate) fn bump_package(package: &Package, ctx: &Context, bump_when_needed: bool) -> anyhow::Result<Bump> {
    let bump_spec = select_publishee_bump_spec(&package.name, ctx);
    bump_package_with_spec(package, bump_spec, ctx, bump_when_needed)
}

pub(crate) fn is_pre_release(semver: &Version) -> bool {
    crate::utils::is_pre_release_version(semver)
}

pub(crate) fn rhs_is_breaking_bump_for_lhs(lhs: &Version, rhs: &Version) -> bool {
    rhs.major > lhs.major || rhs.minor > lhs.minor
}
