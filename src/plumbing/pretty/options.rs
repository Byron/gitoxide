use std::{ffi::OsString, path::PathBuf};

use clap::{AppSettings, Clap};
use gitoxide_core as core;

#[derive(Debug, Clap)]
#[clap(name = "gix-plumbing", about = "The git underworld", version = clap::crate_version!())]
#[clap(setting = AppSettings::SubcommandRequired)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(long, short = 't')]
    /// The amount of threads to use for some operations.
    ///
    /// If unset, or the value is 0, there is no limit and all logical cores can be used.
    pub threads: Option<usize>,

    /// Display verbose messages and progress information
    #[clap(long, short = 'v')]
    pub verbose: bool,

    /// Bring up a terminal user interface displaying progress visually
    #[clap(long, conflicts_with("verbose"))]
    pub progress: bool,

    /// The progress TUI will stay up even though the work is already completed.
    ///
    /// Use this to be able to read progress messages or additional information visible in the TUI log pane.
    #[clap(long, conflicts_with("verbose"), requires("progress"))]
    pub progress_keep_open: bool,

    /// Determine the format to use when outputting statistics.
    #[clap(
        long,
        short = 'f',
        default_value = "human",
        possible_values(core::OutputFormat::variants())
    )]
    pub format: core::OutputFormat,

    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, Clap)]
pub enum Subcommands {
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    PackCreate {
        #[clap(long, short = 'r')]
        /// the directory containing the '.git' repository from which objects should be read.
        repository: Option<PathBuf>,

        #[clap(long, short = 'e', possible_values(core::pack::create::ObjectExpansion::variants()))]
        /// the way objects are expanded. They differ in costs.
        ///
        /// Possible values are "none" and "tree-traversal". Default is "none".
        expansion: Option<core::pack::create::ObjectExpansion>,

        #[clap(long)]
        /// if set, the counting phase may be accelerated using multithreading.
        ///
        /// On the flip side, however, one will loose deterministic counting results which affects the
        /// way the resulting pack is structured.
        nondeterministic_count: bool,

        #[clap(long, short = 's')]
        /// If set statistical information will be presented to inform about pack creation details.
        /// It's a form of instrumentation for developers to help improve pack generation.
        statistics: bool,

        #[clap(long)]
        /// The size in megabytes for a cache to speed up pack access for packs with long delta chains.
        /// It is shared among all threads, so 4 threads would use their own cache 1/4th of the size.
        ///
        /// If unset, no cache will be used.
        pack_cache_size_mb: Option<usize>,

        #[clap(long)]
        /// The size in megabytes for a cache to speed up accessing entire objects, bypassing object database access when hit.
        /// It is shared among all threads, so 4 threads would use their own cache 1/4th of the size.
        ///
        /// This cache type is currently only effective when using the 'diff-tree' object expansion.
        ///
        /// If unset, no cache will be used.
        object_cache_size_mb: Option<usize>,

        #[clap(long)]
        /// if set, delta-objects whose base object wouldn't be in the pack will not be recompressed as base object, but instead
        /// refer to its base object using its object id.
        ///
        /// This allows for smaller packs but requires the receiver of the pack to resolve these ids before storing the pack.
        /// Packs produced with this option enabled are only valid in transit, but not at rest.
        thin: bool,

        /// The directory into which to write the pack file.
        #[clap(long, short = 'o')]
        output_directory: Option<PathBuf>,

        /// The tips from which to start the commit graph iteration, either as fully qualified commit hashes
        /// or as branch names.
        ///
        /// If empty, we expect to read objects on stdin and default to 'none' as expansion mode.
        /// Otherwise the expansion mode is 'tree-traversal' by default.
        tips: Vec<OsString>,
    },
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    PackReceive {
        /// The protocol version to use. Valid values are 1 and 2
        #[clap(long, short = 'p')]
        protocol: Option<core::net::Protocol>,

        /// the directory into which to write references. Existing files will be overwritten.
        ///
        /// Note that the directory will be created if needed.
        #[clap(long, short = 'd')]
        refs_directory: Option<PathBuf>,

        /// The URLs or path from which to receive the pack.
        ///
        /// See here for a list of supported URLs: <https://www.git-scm.com/docs/git-clone#_git_urls>
        url: String,

        /// If set once or more times, these references will be fetched instead of all advertised ones.
        ///
        /// Note that this requires a reasonably modern git server.
        #[clap(long = "reference", short = 'r')]
        refs: Vec<String>,

        /// The directory into which to write the received pack and index.
        ///
        /// If unset, they will be discarded.
        directory: Option<PathBuf>,
    },
    /// List remote references from a remote identified by a url.
    ///
    /// This is the plumbing equivalent of `git ls-remote`.
    /// Supported URLs are documented here: <https://www.git-scm.com/docs/git-clone#_git_urls>
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    RemoteRefList {
        /// The protocol version to use. Valid values are 1 and 2
        #[clap(long, short = 'p')]
        protocol: Option<core::net::Protocol>,

        /// the URLs or path from which to receive references
        ///
        /// See here for a list of supported URLs: <https://www.git-scm.com/docs/git-clone#_git_urls>
        url: String,
    },
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    PackIndexFromData {
        /// Specify how to iterate the pack, defaults to 'verify'
        ///
        /// Valid values are
        ///
        ///  **as-is** do not do anything and expect the pack file to be valid as per the trailing hash,
        ///  **verify** the input ourselves and validate that it matches with the hash provided in the pack,
        ///  **restore** hash the input ourselves and ignore failing entries, instead finish the pack with the hash we computed
        #[clap(
            long,
            short = 'i',
            default_value = "verify",
            possible_values(core::pack::index::IterationMode::variants())
        )]
        iteration_mode: core::pack::index::IterationMode,

        /// Path to the pack file to read (with .pack extension).
        ///
        /// If unset, the pack file is expected on stdin.
        #[clap(long, short = 'p')]
        pack_path: Option<PathBuf>,

        /// The folder into which to place the pack and the generated index file
        ///
        /// If unset, only informational output will be provided to standard output.
        #[clap(parse(from_os_str))]
        directory: Option<PathBuf>,
    },
    /// Verify the integrity of a pack or index file
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    PackExplode {
        #[clap(long)]
        /// Read written objects back and assert they match their source. Fail the operation otherwise.
        ///
        /// Only relevant if an object directory is set.
        verify: bool,

        /// delete the pack and index file after the operation is successful
        #[clap(long)]
        delete_pack: bool,

        /// The amount of checks to run
        #[clap(
            long,
            short = 'c',
            default_value = "all",
            possible_values(core::pack::explode::SafetyCheck::variants())
        )]
        check: core::pack::explode::SafetyCheck,

        /// Compress bytes even when using the sink, i.e. no object directory is specified
        ///
        /// This helps to determine overhead related to compression. If unset, the sink will
        /// only create hashes from bytes, which is usually limited by the speed at which input
        /// can be obtained.
        #[clap(long)]
        sink_compress: bool,

        /// The '.pack' or '.idx' file to explode into loose objects
        #[clap(parse(from_os_str))]
        pack_path: PathBuf,

        /// The path into which all objects should be written. Commonly '.git/objects'
        #[clap(parse(from_os_str))]
        object_path: Option<PathBuf>,
    },
    /// Verify the integrity of a pack or index file
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    PackVerify {
        /// output statistical information about the pack
        #[clap(long, short = 's')]
        statistics: bool,
        /// The algorithm used to verify the pack. They differ in costs.
        #[clap(
            long,
            short = 'a',
            default_value = "less-time",
            possible_values(core::pack::verify::Algorithm::variants())
        )]
        algorithm: core::pack::verify::Algorithm,

        #[clap(long, conflicts_with("re-encode"))]
        /// Decode and parse tags, commits and trees to validate their correctness beyond hashing correctly.
        ///
        /// Malformed objects should not usually occur, but could be injected on purpose or accident.
        /// This will reduce overall performance.
        decode: bool,

        #[clap(long)]
        /// Decode and parse tags, commits and trees to validate their correctness, and re-encode them.
        ///
        /// This flag is primarily to test the implementation of encoding, and requires to decode the object first.
        /// Encoding an object after decoding it should yield exactly the same bytes.
        /// This will reduce overall performance even more, as re-encoding requires to transform zero-copy objects into
        /// owned objects, causing plenty of allocation to occour.
        re_encode: bool,

        /// The '.pack' or '.idx' file whose checksum to validate.
        #[clap(parse(from_os_str))]
        path: PathBuf,
    },
    /// Verify the integrity of a commit graph
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersionFlag)]
    CommitGraphVerify {
        /// The path to '.git/objects/info/', '.git/objects/info/commit-graphs/', or '.git/objects/info/commit-graph' to validate.
        #[clap(parse(from_os_str))]
        path: PathBuf,
        /// output statistical information about the pack
        #[clap(long, short = 's')]
        statistics: bool,
    },
}
