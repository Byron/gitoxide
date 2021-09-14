use anyhow::bail;
use cargo_metadata::Package;
use semver::{BuildMetadata, Prerelease, Version};

use super::Context;

#[allow(clippy::ptr_arg)]
pub(crate) fn select_publishee_bump_spec<'a>(name: &String, ctx: &'a Context) -> &'a str {
    if ctx.crate_names.contains(name) {
        &ctx.bump
    } else {
        &ctx.bump_dependencies
    }
}

pub(crate) fn bump(
    publishee: &Package,
    bump_spec: &str,
    ctx: &Context,
    bump_when_needed: bool,
) -> anyhow::Result<Version> {
    let mut v = publishee.version.clone();
    match bump_spec {
        "major" => {
            v.major += 1;
            v.minor = 0;
            v.patch = 0;
            v.pre = Prerelease::EMPTY;
        }
        "minor" => {
            v.minor += 1;
            v.patch = 0;
            v.pre = Prerelease::EMPTY;
        }
        "patch" => {
            v.patch += 1;
            v.pre = Prerelease::EMPTY;
        }
        "keep" => {}
        _ => bail!("Invalid version specification: '{}'", bump_spec),
    };
    smallest_necessary_version_relative_to_crates_index(publishee, v, ctx, bump_when_needed, true, true, false)
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
            if bump_when_needed && package.version > latest_published_version {
                let verbose = if package_version_must_be_breaking {
                    if rhs_is_breaking_bump_for_lhs(&package.version, &new_version) {
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
                            "Using manifest version {} of crate {} instead of new version {} as it is sufficient to succeed latest published version {}.",
                            package.version,
                            package.name,
                            new_version,
                            latest_published_version
                        );
                    }
                } else if verbose {
                    log::info!(
                        "Using manifest version {} of crate {} as it is sufficient to succeed latest published version {}.",
                        package.version,
                        package.name,
                        latest_published_version
                    );
                }
            }
        }
        None => {
            if bump_when_needed {
                if verbose {
                    log::info!(
                        "Using current version {} instead of bumped one {}.",
                        package.version,
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
    semver.major == 0
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
