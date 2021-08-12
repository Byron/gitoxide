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

    /// allow publishes to take place on a dirty working tree. Really not recommended alongside --execute.
    #[argh(switch)]
    pub allow_dirty: bool,

    /// don't actually publish, but perform all other operations like manifest adjustments and tag creation.
    #[argh(switch)]
    pub skip_publish: bool,

    /// if set it will be allowed to publish crates with cycles to other workspace crates which are not published.
    ///
    /// Doing so causes repeated publishes to never stabilize, as one set of dependencies destabilizes another set.
    /// A possible fix is to bump version numbers of all crates in the set at once and publishing those who would
    /// depend on an unpublished version with "--no-validate".
    #[argh(switch)]
    pub ignore_instability: bool,

    /// a specification of the kind of version bump you seek for the crate and potentially it's dependencies.
    ///
    /// Can be 'major', 'minor' or 'patch', or 'keep' which doesn't alter the version.
    #[argh(positional)]
    pub version_bump_spec: String,

    /// the name of the crates to be released, along with all of their dependencies if needed, using `cargo release`
    #[argh(positional)]
    pub crates: Vec<String>,
}
