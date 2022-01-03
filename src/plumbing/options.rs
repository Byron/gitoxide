use clap::AppSettings;
use gitoxide_core as core;

#[derive(Debug, clap::Parser)]
#[clap(name = "gix-plumbing", about = "The git underworld", version = clap::crate_version!())]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
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
    #[cfg(feature = "prodash-render-tui")]
    #[clap(long, conflicts_with("verbose"))]
    pub progress: bool,

    /// The progress TUI will stay up even though the work is already completed.
    ///
    /// Use this to be able to read progress messages or additional information visible in the TUI log pane.
    #[cfg(feature = "prodash-render-tui")]
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

    /// The object format to assume when reading files that don't inherently know about it, or when writing files.
    #[clap(long, default_value = "sha1", possible_values(&["sha1"]))]
    pub object_hash: git_repository::hash::Kind,

    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Parser)]
pub enum Subcommands {
    /// Subcommands for interacting with packs and their indices.
    #[clap(subcommand)]
    Pack(pack::Subcommands),
    /// Subcommands for interacting with git remotes, e.g. git repositories hosted on servers.
    #[clap(subcommand)]
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    Remote(remote::Subcommands),
    /// Subcommands for interacting with commit-graphs
    #[clap(subcommand)]
    CommitGraph(commitgraph::Subcommands),
    /// Subcommands for interacting with entire git repositories
    #[clap(subcommand)]
    Repository(repo::Subcommands),
}

///
pub mod pack {
    use std::{ffi::OsString, path::PathBuf};

    use clap::AppSettings;
    use gitoxide_core as core;

    #[derive(Debug, clap::Parser)]
    pub enum Subcommands {
        /// Subcommands for interacting with pack indices (.idx)
        #[clap(subcommand)]
        Index(index::Subcommands),
        /// Subcommands for interacting with multi-pack indices (named "multi-pack-index")
        #[clap(subcommand)]
        MultiIndex(multi_index::Subcommands),
        /// Create a new pack with a set of objects.
        #[clap(setting = AppSettings::DisableVersionFlag)]
        Create {
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
        /// Use the git-protocol to receive a pack, emulating a clone.
        #[clap(setting = AppSettings::DisableVersionFlag)]
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Receive {
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
        /// Dissolve a pack into its loose objects.
        ///
        /// Note that this effectively removes delta compression for an average compression of 2x, creating one file per object in the process.
        /// Thus this should only be done to dissolve small packs after a fetch.
        #[clap(setting = AppSettings::DisableVersionFlag)]
        Explode {
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
        /// Verify the integrity of a pack, index or multi-index file
        #[clap(setting = AppSettings::DisableVersionFlag)]
        Verify {
            #[clap(flatten)]
            args: VerifyOptions,

            /// The '.pack', '.idx' or 'multi-pack-index' file to validate.
            #[clap(parse(from_os_str))]
            path: PathBuf,
        },
    }

    #[derive(Debug, clap::Parser)]
    pub struct VerifyOptions {
        /// output statistical information
        #[clap(long, short = 's')]
        pub statistics: bool,
        /// The algorithm used to verify packs. They differ in costs.
        #[clap(
            long,
            short = 'a',
            default_value = "less-time",
            possible_values(core::pack::verify::Algorithm::variants())
        )]
        pub algorithm: core::pack::verify::Algorithm,

        #[clap(long, conflicts_with("re-encode"))]
        /// Decode and parse tags, commits and trees to validate their correctness beyond hashing correctly.
        ///
        /// Malformed objects should not usually occur, but could be injected on purpose or accident.
        /// This will reduce overall performance.
        pub decode: bool,

        #[clap(long)]
        /// Decode and parse tags, commits and trees to validate their correctness, and re-encode them.
        ///
        /// This flag is primarily to test the implementation of encoding, and requires to decode the object first.
        /// Encoding an object after decoding it should yield exactly the same bytes.
        /// This will reduce overall performance even more, as re-encoding requires to transform zero-copy objects into
        /// owned objects, causing plenty of allocation to occour.
        pub re_encode: bool,
    }

    ///
    pub mod multi_index {
        use std::path::PathBuf;

        use clap::AppSettings;

        #[derive(Debug, clap::Parser)]
        pub enum Subcommands {
            /// Verify a multi-index quickly without inspecting objects themselves
            #[clap(setting = AppSettings::DisableVersionFlag)]
            Verify {
                /// The path to the multi-pack-index to verify.
                multi_index_path: PathBuf,
            },
            /// Create a multi-pack index from one or more pack index files
            #[clap(setting = AppSettings::DisableVersionFlag)]
            Create {
                /// The path to which the multi-index file should be written, overwriting any possibly existing file.
                ///
                /// Note for the multi-index to be useful, it should be side-by-side with the supplied `.idx` files.
                #[clap(long, short = 'o')]
                output_path: PathBuf,

                /// Paths to the pack index files to read (with .idx extension).
                #[clap(required = true)]
                index_paths: Vec<PathBuf>,
            },
        }
    }

    ///
    pub mod index {
        use std::path::PathBuf;

        use clap::AppSettings;
        use gitoxide_core as core;

        #[derive(Debug, clap::Parser)]
        pub enum Subcommands {
            /// create a pack index from a pack data file.
            #[clap(setting = AppSettings::DisableVersionFlag)]
            Create {
                /// Specify how to iterate the pack, defaults to 'verify'
                ///
                /// Valid values are
                ///
                ///  **as-is** do not do anything and expect the pack file to be valid as per the trailing hash,
                ///  **verify** the input ourselves and validate that it matches with the hash provided in the pack,
                ///  **restore** hash the input ourselves and ignore failing entries, instead finish the pack with the hash we computed
                ///  to keep as many objects as possible.
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
        }
    }
}

///
pub mod repo {
    use std::path::PathBuf;

    use clap::AppSettings;

    #[derive(Debug, clap::Parser)]
    #[clap(alias = "repo")]
    pub enum Subcommands {
        /// Verify the integrity of the entire repository
        #[clap(setting = AppSettings::DisableVersionFlag)]
        Verify {
            #[clap(flatten)]
            args: super::pack::VerifyOptions,
            #[clap(short = 'r', long, default_value = ".")]
            repository: PathBuf,
        },
    }
}

///
pub mod commitgraph {
    use std::path::PathBuf;

    use clap::AppSettings;

    #[derive(Debug, clap::Parser)]
    pub enum Subcommands {
        /// Verify the integrity of a commit graph
        #[clap(setting = AppSettings::DisableVersionFlag)]
        Verify {
            /// The path to '.git/objects/info/', '.git/objects/info/commit-graphs/', or '.git/objects/info/commit-graph' to validate.
            #[clap(parse(from_os_str))]
            path: PathBuf,
            /// output statistical information about the pack
            #[clap(long, short = 's')]
            statistics: bool,
        },
    }
}

///
#[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
pub mod remote {
    use clap::AppSettings;
    use gitoxide_core as core;

    #[derive(Debug, clap::Parser)]
    pub enum Subcommands {
        /// List remote references from a remote identified by a url.
        ///
        /// This is the plumbing equivalent of `git ls-remote`.
        /// Supported URLs are documented here: <https://www.git-scm.com/docs/git-clone#_git_urls>
        #[clap(setting = AppSettings::DisableVersionFlag)]
        RefList {
            /// The protocol version to use. Valid values are 1 and 2
            #[clap(long, short = 'p')]
            protocol: Option<core::net::Protocol>,

            /// the URLs or path from which to receive references
            ///
            /// See here for a list of supported URLs: <https://www.git-scm.com/docs/git-clone#_git_urls>
            url: String,
        },
    }
}
