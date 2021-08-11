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
    /// actually perform a release. Dry-run mode is the default
    #[argh(switch, short = 'n')]
    pub execute: bool,

    /// a specification of the kind of version bump you seek for the crate and potentially it's dependencies.
    ///
    /// Can be 'major', 'minor' or 'patch'
    #[argh(positional)]
    pub version_bump_spec: String,

    /// the name of the crates to be released, along with all of their dependencies if needed, using `cargo release`
    #[argh(positional)]
    pub crates: Vec<String>,
}
