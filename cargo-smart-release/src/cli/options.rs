use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::SubcommandRequired)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::DisableHelpSubcommand)]
/// Release workspace crates fearlessly.
///
/// Use --execute to actually perform the operation.
pub struct Args {
    #[clap(subcommand)]
    pub subcommands: SubCommands,
}

#[derive(Clap)]
pub enum SubCommands {
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    #[clap(name = "smart-release")]
    /// Release workspace crates fearlessly.
    ///
    /// Use --execute to actually perform the operation.
    SmartRelease {
        /// Actually perform a release. Dry-run mode is the default
        #[clap(long, short = 'n')]
        execute: bool,

        /// Provide more detailed messages on the INFO log level in dry-run mode.
        ///
        /// Note --verbose is implied with --execute.
        #[clap(long, short = 'v')]
        verbose: bool,

        /// Always update the crates-index beforehand. It is used to determine if the computed version to be published was
        /// already published.
        #[clap(long, short = 'u')]
        update_crates_index: bool,

        /// Don't generate a changelog automatically or update existing ones. This is useful if a manual changelog
        /// is preferred or if its format strays to far from the suggestions on https://keepachangelog.com, making
        /// generated content impossible to properly integrate with what's there.
        #[clap(long)]
        no_changelog: bool,

        /// Specify the kind of version bump you seek for the crate and potentially it's dependencies.
        ///
        /// Can be 'major', 'minor' or 'patch', 'keep' and 'auto'.
        /// With 'keep', the current version will be kept, useful if versions are specified by hand in the manifest.
        ///
        /// The default is 'auto', which derives the necessary information from the git commit history and occasional
        /// conventional messages.
        #[clap(long, short = 'b')]
        bump: Option<String>,

        /// Specify the kind of version bump to apply to dependencies only.
        ///
        /// Can be 'major', 'minor' or 'patch', 'keep' and 'auto'.
        /// With 'keep', the current version will be kept, useful if versions are specified by hand in the manifest.
        ///
        /// The default is 'auto', which derives the necessary information from the git commit history and occasional
        /// conventional messages.
        #[clap(long, short = 'd')]
        bump_dependencies: Option<String>,

        /// The name of the crates to be released, along with all of their dependencies if needed.
        ///
        /// Defaults to the top-level workspace crate if unset.
        crates: Vec<String>,

        /// Additionally run 'cargo publish --dry-run' when --execute is not set. This can be useful to see which local
        /// crates do not build with the released versions of their workspace dependencies anymore.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        dry_run_cargo_publish: bool,

        /// As dependencies are automatically picked up for release, put all manifest changes into a single commit.
        ///
        /// If this flag is set, each dependency will yield its own commit with respective manifest changes, possibly
        /// adding a lot of additional commits even though the release could have looked like coherent.
        #[clap(long, help_heading = Some("EXPERT"))]
        no_multi_crate_release: bool,

        /// Always bump versions as specified by --bump or --bump-dependencies even if this is not required
        /// to publish a new version to crates.io.
        ///
        /// For instance, if the currently set version is 1.0 and the latest published version is 0.5, while
        /// a minor version bump is specified like -b minor, 1.0 would be published instead of 1.1 unless this
        /// flag is set.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_bump_on_demand: bool,

        /// Do not create a github release (if the repo is located on github) reflecting the content of the changelogs most recent
        /// release section.
        #[clap(long, help_heading = Some("CHANGELOG"))]
        no_changelog_github_release: bool,

        /// If changelogs are generated from git-conventional comments extracted from the commit history exclusively, publishes
        /// will fail as in order to give opportunity to author at least a portion of the upcoming release.
        ///
        /// With this flag set, the release will not stop.
        /// Note that changelog entries purely composed of statistical information will always stop a release.
        #[clap(long, help_heading = Some("CHANGELOG-EXPERT"))]
        allow_fully_generated_changelogs: bool,

        /// Do not generate links to commits and issues when writing the changelogs. This currently only works for GitHub.
        #[clap(long, help_heading = Some("CHANGELOG"))]
        no_changelog_links: bool,

        /// Omits these kinds of generated changelog content, values are 'clippy', 'commit-statistics' and 'commit-details'
        #[clap(long, help_heading = Some("CHANGELOG"))]
        changelog_without: Vec<String>,
        /// If unset, about-to-be changed changelogs will be previewed using 'bat', if available.
        ///
        /// If set, no preview will ever be displayed, but note that empty changelogs will always stop the release process.
        #[clap(long, help_heading = Some("CHANGELOG"))]
        no_changelog_preview: bool,

        /// Allow publishes to take place on a dirty working tree. Really not recommended alongside --execute.
        #[clap(long, help_heading = Some("EXPERT"))]
        allow_dirty: bool,

        /// Disallow to also publish stable crates when discovering changed crates, bumping their version according to -d <spec>.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_auto_publish_of_stable_crates: bool,

        /// Do not force pre-release version requirements to be updated in dependent crates on patch update, forcing
        /// an update of the lower bound.
        ///
        /// It's best to look at this issue for a deeper understanding: https://github.com/Byron/gitoxide/issues/194
        #[clap(long, help_heading = Some("EXPERT"))]
        no_conservative_pre_release_version_handling: bool,

        /// Do not bump versions of dependent crates if the crates to be published indicate breaking changes with their semantic version.
        ///
        /// For details, it's best to look at https://github.com/Byron/gitoxide/issues/192
        #[clap(long, help_heading = Some("EXPERT"))]
        no_isolate_dependencies_from_breaking_changes: bool,

        /// Don't actually publish, but perform all other operations like manifest adjustments and tag creation.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_publish: bool,

        /// Don't create tags indicating the version numbers of all crates that are to be published after changing
        /// their manifests.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_tag: bool,

        /// Don't push tags and the HEAD branch after any successful run of `cargo publish`.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_push: bool,

        /// Do not take into consideration any dependencies of the crates to publish.
        ///
        /// This flag is useful when various `--skip-X` are specified in order to bump versions only, without publishing.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_dependencies: bool,

        /// Pass --no-verify to 'cargo publish' which should only be a last resort when fixing up packages that
        /// otherwise wouldn't publish, but need to be publish to resolve the situation.
        #[clap(long, help_heading = Some("EXPERT"))]
        dangerously_pass_no_verify: bool,

        /// If set it will be allowed to publish crates with cycles to other workspace crates which are not published.
        ///
        /// Doing so causes repeated publishes to never stabilize, as one set of dependencies destabilizes another set.
        /// A possible fix is to bump version numbers of all crates in the set at once and publishing those who would
        /// depend on an unpublished version with "--no-validate".
        #[clap(long, help_heading = Some("EXPERT"))]
        ignore_instability: bool,
    },
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    #[clap(name = "changelog")]
    /// Generate changelogs from commit histories, non-destructively.
    ///
    /// Use --write to actually write generated changelogs
    Changelog {
        /// Actually write the changelog to the respective files
        #[clap(long, short = 'w')]
        write: bool,

        /// omits these kinds of generated changelog content, values are 'clippy', 'commit-statistics' and 'commit-details'
        #[clap(long)]
        without: Vec<String>,

        /// Do not take into consideration any dependencies of the crates to generate the changelog for.
        ///
        /// This flag is useful if you plan to review and finalize changelogs before a a smart-release, where dependencies
        /// are taken into consideration by default, but would like to do so one at a time.
        #[clap(long)]
        no_dependencies: bool,

        /// The name of the crates to generate a changelog for.
        ///
        /// Defaults to the top-level workspace crate if unset.
        crates: Vec<String>,

        /// Allow changelog updates to take place on a dirty working tree when --write is set as well.
        ///
        /// For now this is not recommended as changelogs might be damaged beyond repair.
        #[clap(long, help_heading = Some("EXPERT"))]
        allow_dirty: bool,

        /// If --write is not set, 'bat' will be used (if available) to print the new changelog to stdout as preview. Use this flag
        /// to disable such behaviour.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_preview: bool,

        /// Do not generate links to commits and issues when writing the changelogs. This currently only works for GitHub.
        #[clap(long, help_heading = Some("CUSTOMIZATION"))]
        no_links: bool,
    },
}
