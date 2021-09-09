use anyhow::bail;
use cargo_metadata::Package;
use semver::Version;

use super::Context;

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
    no_bump_on_demand: bool,
) -> anyhow::Result<Version> {
    fn validated_new_version(
        publishee: &Package,
        mut new_version: Version,
        ctx: &Context,
        no_bump_on_demand: bool,
    ) -> anyhow::Result<Version> {
        let bump_on_demand = !no_bump_on_demand;
        match ctx.crates_index.crate_(&publishee.name) {
            Some(existing_release) => {
                let existing_version = semver::Version::parse(existing_release.latest_version().version())?;
                if existing_version >= new_version {
                    bail!(
                "Latest published version of '{}' is {}, the new version is {}. Consider using --bump <level> or --bump-dependencies <level>.",
                publishee.name,
                existing_release.latest_version().version(),
                new_version
            );
                }
                if bump_on_demand && publishee.version > existing_version {
                    if new_version > publishee.version {
                        log::info!(
                        "Using manifest version {} of crate {} instead of new version {} as it is sufficient to succeed latest published version {}.",
                        publishee.version,
                        publishee.name,
                        new_version,
                        existing_version
                    );
                    } else {
                        log::info!(
                        "Using manifest version {} of crate {} as it is sufficient to succeed latest published version {}.",
                        publishee.version,
                        publishee.name,
                        existing_version
                    );
                    }
                    new_version = publishee.version.clone();
                }
            }
            None => {
                if bump_on_demand {
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
    validated_new_version(publishee, v, ctx, no_bump_on_demand)
}
