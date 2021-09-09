use anyhow::bail;
use cargo_metadata::Package;
use semver::Version;

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
    use semver::Prerelease;
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
    validate(publishee, v, ctx, bump_when_needed)
}

fn validate(
    publishee: &Package,
    mut new_version: Version,
    ctx: &Context,
    bump_when_needed: bool,
) -> anyhow::Result<Version> {
    match ctx.crates_index.crate_(&publishee.name) {
        Some(published_crate) => {
            let latest_published_version = semver::Version::parse(published_crate.latest_version().version())?;
            if latest_published_version >= new_version {
                bail!(
                "Latest published version of '{}' is {}, the new version is {}. Consider using --bump <level> or --bump-dependencies <level> or update the index with --update-crates-index.",
                publishee.name,
                published_crate.latest_version().version(),
                new_version
            );
            }
            if bump_when_needed && publishee.version > latest_published_version {
                if new_version > publishee.version {
                    log::info!(
                        "Using manifest version {} of crate {} instead of new version {} as it is sufficient to succeed latest published version {}.",
                        publishee.version,
                        publishee.name,
                        new_version,
                        latest_published_version
                    );
                } else {
                    log::info!(
                        "Using manifest version {} of crate {} as it is sufficient to succeed latest published version {}.",
                        publishee.version,
                        publishee.name,
                        latest_published_version
                    );
                }
                new_version = publishee.version.clone();
            }
        }
        None => {
            if bump_when_needed {
                log::info!(
                    "Using current version {} instead of bumped one {}.",
                    publishee.version,
                    new_version
                );
                new_version = publishee.version.clone();
            }
            log::info!("Congratulations for the new release of '{}' 🎉", publishee.name);
        }
    };
    Ok(new_version)
}

pub(crate) fn is_pre_release(semver: &Version) -> bool {
    semver.major == 0
}
