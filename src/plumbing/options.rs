use std::path::PathBuf;

use gitoxide_core as core;

#[derive(Debug, clap::Parser)]
#[clap(name = "gix-plumbing", about = "The git underworld", version = clap::crate_version!())]
#[clap(subcommand_required = true)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// The repository to access.
    #[clap(short = 'r', long, default_value = ".")]
    pub repository: PathBuf,

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
    #[clap(long, default_value_t = git_repository::hash::Kind::default(), possible_values(&["SHA1"]))]
    pub object_hash: git_repository::hash::Kind,

    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Interact with the object database.
    #[clap(subcommand)]
    Odb(odb::Subcommands),
    /// Interact with tree objects.
    #[clap(subcommand)]
    Tree(tree::Subcommands),
    /// Interact with commit objects.
    #[clap(subcommand)]
    Commit(commit::Subcommands),
    /// Verify the integrity of the entire repository
    Verify {
        #[clap(flatten)]
        args: free::pack::VerifyOptions,
    },
    /// Query and obtain information about revisions.
    #[clap(subcommand)]
    Revision(revision::Subcommands),
    /// Interact with the mailmap.
    #[clap(subcommand)]
    Mailmap(mailmap::Subcommands),
    /// Interact with the remote hosts.
    Remote(remote::Platform),
    /// Interact with the exclude files like .gitignore.
    #[clap(subcommand)]
    Exclude(exclude::Subcommands),
    Config(config::Platform),
    /// Subcommands that need no git repository to run.
    #[clap(subcommand)]
    Free(free::Subcommands),
}

pub mod config {
    /// Print all entries in a configuration file or access other sub-commands
    #[derive(Debug, clap::Parser)]
    #[clap(subcommand_required(false))]
    pub struct Platform {
        /// The filter terms to limit the output to matching sections and subsections only.
        ///
        /// Typical filters are `branch` or `remote.origin` or `remote.or*` - git-style globs are supported
        /// and comparisons are case-insensitive.
        pub filter: Vec<String>,
    }
}

pub mod remote {
    #[derive(Debug, clap::Parser)]
    pub struct Platform {
        /// The name of the remote to connect to.
        #[clap(long, short = 'n', default_value = "origin")]
        pub name: String,

        /// Subcommands
        #[clap(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, clap::Subcommand)]
    #[clap(visible_alias = "remotes")]
    pub enum Subcommands {
        /// Print all references available on the remote
        Refs,
    }
}

pub mod mailmap {
    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Print all entries in configured mailmaps, inform about errors as well.
        Entries,
    }
}

pub mod odb {
    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Print all object names.
        Entries,
        /// Provide general information about the object database.
        Info,
    }
}

pub mod tree {
    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Print entries in a given tree
        Entries {
            /// Traverse the entire tree and its subtrees respectively, not only this tree.
            #[clap(long, short = 'r')]
            recursive: bool,

            /// Provide files size as well. This is expensive as the object is decoded entirely.
            #[clap(long, short = 'e')]
            extended: bool,

            /// The tree to traverse, or the tree at `HEAD` if unspecified.
            treeish: Option<String>,
        },
        /// Provide information about a tree.
        Info {
            /// Provide files size as well. This is expensive as the object is decoded entirely.
            #[clap(long, short = 'e')]
            extended: bool,
            /// The tree to traverse, or the tree at `HEAD` if unspecified.
            treeish: Option<String>,
        },
    }
}

pub mod commit {
    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Describe the current commit or the given one using the name of the closest annotated tag in its ancestry.
        Describe {
            /// Use annotated tag references only, not all tags.
            #[clap(long, short = 't', conflicts_with("all-refs"))]
            annotated_tags: bool,

            /// Use all references under the `ref/` namespaces, which includes tag references, local and remote branches.
            #[clap(long, short = 'a', conflicts_with("annotated-tags"))]
            all_refs: bool,

            /// Only follow the first parent when traversing the commit graph.
            #[clap(long, short = 'f')]
            first_parent: bool,

            /// Always display the long format, even if that would not be necessary as the id is located directly on a reference.
            #[clap(long, short = 'l')]
            long: bool,

            /// Consider only the given `n` candidates. This can take longer, but potentially produces more accurate results.
            #[clap(long, short = 'c', default_value = "10")]
            max_candidates: usize,

            /// Print information on stderr to inform about performance statistics
            #[clap(long, short = 's')]
            statistics: bool,

            #[clap(long)]
            /// If there was no way to describe the commit, fallback to using the abbreviated input revision.
            always: bool,

            /// A specification of the revision to use, or the current `HEAD` if unset.
            rev_spec: Option<String>,
        },
    }
}

pub mod revision {
    #[derive(Debug, clap::Subcommand)]
    #[clap(visible_alias = "rev", visible_alias = "r")]
    pub enum Subcommands {
        /// Provide the revision specification like `@~1` to explain.
        #[clap(visible_alias = "e")]
        Explain { spec: std::ffi::OsString },
        /// Try to resolve the given revspec and print the object names.
        #[clap(visible_alias = "query", visible_alias = "parse", visible_alias = "p")]
        Resolve {
            /// Instead of resolving a rev-spec, explain what would be done for the first spec.
            ///
            /// Equivalent to the `explain` subcommand.
            #[clap(short = 'e', long)]
            explain: bool,
            /// Show the first resulting object similar to how `git cat-file` would, but don't show the resolved spec.
            #[clap(short = 'c', long, conflicts_with = "explain")]
            cat_file: bool,
            /// rev-specs like `@`, `@~1` or `HEAD^2`.
            #[clap(required = true)]
            specs: Vec<std::ffi::OsString>,
        },
        /// Return the names and hashes of all previously checked-out branches.
        #[clap(visible_alias = "prev")]
        PreviousBranches,
    }
}

///
pub mod free {
    #[derive(Debug, clap::Subcommand)]
    #[clap(visible_alias = "no-repo")]
    pub enum Subcommands {
        /// Subcommands for interacting with git remote server.
        #[clap(subcommand)]
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Remote(remote::Subcommands),
        /// Subcommands for interacting with commit-graphs
        #[clap(subcommand)]
        CommitGraph(commitgraph::Subcommands),
        /// Subcommands for interacting with mailmaps
        Mailmap {
            #[clap(flatten)]
            cmd: mailmap::Platform,
        },
        /// Subcommands for interacting with pack files and indices
        #[clap(subcommand)]
        Pack(pack::Subcommands),
        /// Subcommands for interacting with a worktree index, typically at .git/index
        Index(index::Platform),
    }

    ///
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    pub mod remote {
        use gitoxide_core as core;

        #[derive(Debug, clap::Subcommand)]
        pub enum Subcommands {
            /// List remote references from a remote identified by a url.
            ///
            /// This is the plumbing equivalent of `git ls-remote`.
            /// Supported URLs are documented here: <https://www.git-scm.com/docs/git-clone#_git_urls>
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

    ///
    pub mod commitgraph {
        use std::path::PathBuf;

        #[derive(Debug, clap::Subcommand)]
        pub enum Subcommands {
            /// Verify the integrity of a commit graph
            Verify {
                /// The path to '.git/objects/info/', '.git/objects/info/commit-graphs/', or '.git/objects/info/commit-graph' to validate.
                path: PathBuf,
                /// output statistical information about the pack
                #[clap(long, short = 's')]
                statistics: bool,
            },
        }
    }

    pub mod index {
        use std::path::PathBuf;

        #[derive(Debug, clap::Parser)]
        pub struct Platform {
            /// The object format to assume when reading files that don't inherently know about it, or when writing files.
            #[clap(long, default_value_t = git_repository::hash::Kind::default(), possible_values(&["SHA1"]))]
            pub object_hash: git_repository::hash::Kind,

            /// The path to the index file.
            #[clap(short = 'i', long, default_value = ".git/index")]
            pub index_path: PathBuf,

            /// Subcommands
            #[clap(subcommand)]
            pub cmd: Subcommands,
        }

        #[derive(Debug, clap::Subcommand)]
        pub enum Subcommands {
            /// Validate constraints and assumptions of an index along with its integrity.
            Verify,
            /// Print all entries to standard output
            Entries,
            /// Print information about the index structure
            Info {
                /// Do not extract specific extension information to gain only a superficial idea of the index's composition.
                #[clap(long)]
                no_details: bool,
            },
            /// Checkout the index into a directory with exclusive write access, similar to what would happen during clone.
            CheckoutExclusive {
                /// The path to `.git` repository from which objects can be obtained to write the actual files referenced
                /// in the index. Use this measure the impact on extracting objects on overall performance.
                #[clap(long, short = 'r')]
                repository: Option<PathBuf>,
                /// Ignore errors and keep checking out as many files as possible, and report all errors at the end of the operation.
                #[clap(long, short = 'k')]
                keep_going: bool,
                /// Enable to query the object database yet write only empty files. This is useful to measure the overhead of ODB query
                /// compared to writing the bytes to disk.
                #[clap(long, short = 'e', requires = "repository")]
                empty_files: bool,
                /// The directory into which to write all index entries.
                directory: PathBuf,
            },
        }
    }

    ///
    pub mod pack {
        use std::{ffi::OsString, path::PathBuf};

        use gitoxide_core as core;

        #[derive(Debug, clap::Subcommand)]
        pub enum Subcommands {
            /// Subcommands for interacting with pack indices (.idx)
            #[clap(subcommand)]
            Index(index::Subcommands),
            /// Subcommands for interacting with multi-pack indices (named "multi-pack-index")
            MultiIndex(multi_index::Platform),
            /// Create a new pack with a set of objects.
            Create {
                #[clap(long, short = 'r')]
                /// the directory containing the '.git' repository from which objects should be read.
                repository: Option<PathBuf>,

                #[clap(long, short = 'e', possible_values(core::pack::create::ObjectExpansion::variants()))]
                /// the way objects are expanded. They differ in costs.
                ///
                /// Possible values are "none" and "tree-traversal". Default is "none".
                expansion: Option<core::pack::create::ObjectExpansion>,

                #[clap(long, default_value_t = 3, requires = "nondeterministic-count")]
                /// The amount of threads to use when counting and the `--nondeterminisitc-count` flag is set, defaulting
                /// to the globally configured threads.
                ///
                /// Use it to have different trade-offs between counting performance and cost in terms of CPU, as the scaling
                /// here is everything but linear. The effectiveness of each core seems to be no more than 30%.
                counting_threads: usize,

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
                pack_path: PathBuf,

                /// The path into which all objects should be written. Commonly '.git/objects'
                object_path: Option<PathBuf>,
            },
            /// Verify the integrity of a pack, index or multi-index file
            Verify {
                #[clap(flatten)]
                args: VerifyOptions,

                /// The '.pack', '.idx' or 'multi-pack-index' file to validate.
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
            /// owned objects, causing plenty of allocation to occur.
            pub re_encode: bool,
        }

        ///
        pub mod multi_index {
            use std::path::PathBuf;

            #[derive(Debug, clap::Parser)]
            pub struct Platform {
                /// The path to the index file.
                #[clap(short = 'i', long, default_value = ".git/objects/pack/multi-pack-index")]
                pub multi_index_path: PathBuf,

                /// Subcommands
                #[clap(subcommand)]
                pub cmd: Subcommands,
            }

            #[derive(Debug, clap::Subcommand)]
            pub enum Subcommands {
                /// Display all entries of a multi-index: <oid> <pack-id> <pack-offset>
                Entries,
                /// Print general information about a multi-index file
                Info,
                /// Verify a multi-index quickly without inspecting objects themselves
                Verify,
                /// Create a multi-pack index from one or more pack index files, overwriting possibloy existing files.
                Create {
                    /// Paths to the pack index files to read (with .idx extension).
                    ///
                    /// Note for the multi-index to be useful, it should be side-by-side with the supplied `.idx` files.
                    #[clap(required = true)]
                    index_paths: Vec<PathBuf>,
                },
            }
        }

        ///
        pub mod index {
            use std::path::PathBuf;

            use gitoxide_core as core;

            #[derive(Debug, clap::Subcommand)]
            pub enum Subcommands {
                /// create a pack index from a pack data file.
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
                    directory: Option<PathBuf>,
                },
            }
        }
    }

    ///
    pub mod mailmap {
        use std::path::PathBuf;

        #[derive(Debug, clap::Parser)]
        pub struct Platform {
            /// The path to the mailmap file.
            #[clap(short = 'p', long, default_value = ".mailmap")]
            pub path: PathBuf,

            /// Subcommands
            #[clap(subcommand)]
            pub cmd: Subcommands,
        }

        #[derive(Debug, clap::Subcommand)]
        pub enum Subcommands {
            /// Parse all entries in the mailmap and report malformed lines or collisions.
            Verify,
        }
    }
}

pub mod exclude {
    use std::ffi::OsString;

    use git_repository as git;

    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Check if path-specs are excluded and print the result similar to `git check-ignore`.
        Query {
            /// Show actual ignore patterns instead of un-excluding an entry.
            ///
            /// That way one can understand why an entry might not be excluded.
            #[clap(long, short = 'i')]
            show_ignore_patterns: bool,
            /// Additional patterns to use for exclusions. They have the highest priority.
            ///
            /// Useful for undoing previous patterns using the '!' prefix.
            #[clap(long, short = 'p')]
            patterns: Vec<OsString>,
            /// The git path specifications to check for exclusion, or unset to read from stdin one per line.
            #[clap(parse(try_from_os_str = std::convert::TryFrom::try_from))]
            pathspecs: Vec<git::path::Spec>,
        },
    }
}
