#![allow(unused)]
use anyhow::{anyhow, bail, Context};

mod options {
    use argh::FromArgs;

    #[derive(FromArgs)]
    #[argh(name = "utils")]
    /// Utilities for maintaining the gitoxide project
    pub struct Args {
        #[argh(subcommand)]
        pub subcommand: SubCommands,
    }

    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum SubCommands {
        Release(Release),
    }

    /// Verify a pack
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "release")]
    pub struct Release {
        /// don't actually perform a release.
        #[argh(switch, short = 'n')]
        pub dry_run: bool,

        /// a specification of the kind of version bump you seek for the crate and potentially it's dependencies.
        ///
        /// Can be 'major', 'minor' or 'patch'
        #[argh(positional)]
        pub version_bump_spec: String,

        /// the name of the crates to be released, along with all of their dependencies if needed, using `cargo release`
        #[argh(positional)]
        pub crates: Vec<String>,
    }
}

use options::{Args, Release, SubCommands};

fn main() -> anyhow::Result<()> {
    let opts: options::Args = argh::from_env();
    match opts.subcommand {
        SubCommands::Release(Release {
            dry_run,
            version_bump_spec,
            crates,
        }) => todo!("release"),
    }
    Ok(())
}

fn crate_version(name: &str) -> anyhow::Result<String> {
    let mut args = std::env::args().skip(1);
    let cmd = args
        .next()
        .ok_or_else(|| anyhow!("The first argument is the subcommand"))?;

    let crate_name = args
        .next()
        .ok_or_else(|| anyhow!("The first argument is the name of the crate whose path "))?;
    let crate_dir = std::env::current_dir()?.join(&crate_name);
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let manifest = cargo_toml::Manifest::from_path(&cargo_toml_path)
        .with_context(|| format!("Couldn't read Cargo manifest at '{}'", cargo_toml_path.display()))?;
    let version = manifest
        .package
        .ok_or_else(|| anyhow!("Need package information"))
        .map(|p| p.version)?;
    Ok(version)
}
