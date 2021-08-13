use argh::FromArgs;

#[derive(FromArgs)]
#[argh(name = "cargo smart-release")]
/// Release workspace crates fearlessly.
///
/// Use --execute to actually perform the operation.
pub struct Args {
    #[argh(subcommand)]
    pub subcommands: SubCommands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubCommands {
    SmartRelease(SmartRelease),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "smart-release")]
/// Release workspace crates fearlessly.
///
/// Use --execute to actually perform the operation.
pub struct SmartRelease {
    /// actually perform a release. Dry-run mode is the default
    #[argh(switch, short = 'n')]
    pub execute: bool,

    /// do not run 'cargo publish --dry-run' even without --execute. This is useful for getting all program output fast.
    #[argh(switch)]
    pub no_dry_run_cargo_publish: bool,

    /// allow publishes to take place on a dirty working tree. Really not recommended alongside --execute.
    #[argh(switch)]
    pub allow_dirty: bool,

    /// always update the crates-index beforehand. It is used to determine if the computed version to be published was
    /// already published.
    #[argh(switch, short = 'u')]
    pub update_crates_index: bool,

    /// allow discovery of changed crates to also publish stable crates, bumping their version according to --bump <spec>.
    #[argh(switch)]
    pub allow_auto_publish_of_stable_crates: bool,

    /// don't actually publish, but perform all other operations like manifest adjustments and tag creation.
    #[argh(switch)]
    pub skip_publish: bool,

    /// don't create tags indicating the version numbers of all crates that are to be published after changing
    /// their manifests.
    #[argh(switch)]
    pub skip_tag: bool,

    /// pass --no-verify to 'cargo publish' which should only be a last resort when fixing up packages that
    /// otherwise wouldn't publish, but need to be publish to resolve the situation.
    #[argh(switch)]
    pub dangerously_pass_no_verify: bool,

    /// if set it will be allowed to publish crates with cycles to other workspace crates which are not published.
    ///
    /// Doing so causes repeated publishes to never stabilize, as one set of dependencies destabilizes another set.
    /// A possible fix is to bump version numbers of all crates in the set at once and publishing those who would
    /// depend on an unpublished version with "--no-validate".
    #[argh(switch)]
    pub ignore_instability: bool,

    /// specify the kind of version bump you seek for the crate and potentially it's dependencies.
    ///
    /// Can be 'major', 'minor' or 'patch', or 'keep' which doesn't alter the version.
    /// If unspecified, the current version will be kept, useful if versions are specified manually.
    #[argh(option, short = 'b')]
    pub bump: Option<String>,

    /// the name of the crates to be released, along with all of their dependencies if needed, using `cargo release`
    #[argh(positional)]
    pub crates: Vec<String>,
}
