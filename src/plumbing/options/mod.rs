use std::path::PathBuf;

use git_repository as git;
use git_repository::bstr::BString;
use gitoxide_core as core;

#[derive(Debug, clap::Parser)]
#[clap(name = "gix-plumbing", about = "The git underworld", version = clap::crate_version!())]
#[clap(subcommand_required = true)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// The repository to access.
    #[clap(short = 'r', long, default_value = ".")]
    pub repository: PathBuf,

    /// Add these values to the configuration in the form of `key=value` or `key`.
    ///
    /// For example, if `key` is `core.abbrev`, set configuration like `[core] abbrev = key`,
    /// or `remote.origin.url = foo` to set `[remote "origin"] url = foo`.
    #[clap(long, short = 'c', parse(try_from_os_str = git::env::os_str_to_bstring))]
    pub config: Vec<BString>,

    #[clap(long, short = 't')]
    /// The amount of threads to use for some operations.
    ///
    /// If unset, or the value is 0, there is no limit and all logical cores can be used.
    pub threads: Option<usize>,

    /// Display verbose messages and progress information
    #[clap(long, short = 'v')]
    pub verbose: bool,

    /// Turn off verbose message display for commands where these are shown by default.
    #[clap(long, conflicts_with("verbose"))]
    pub no_verbose: bool,

    /// Bring up a terminal user interface displaying progress visually
    #[cfg(feature = "prodash-render-tui")]
    #[clap(long, conflicts_with("verbose"))]
    pub progress: bool,

    /// Don't default malformed configuration flags, but show an error instead.
    ///
    /// Note that some subcommands use strict mode by default.
    // TODO: needs a 'lenient' mutually exclusive counterpart. Opens the gate to auto-verbose some commands, and add --no-verbose
    //       for these.
    #[clap(long, short = 's')]
    pub strict: bool,

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
    /// A program just like `git credential`.
    #[clap(subcommand)]
    Credential(credential::Subcommands),
    /// Fetch data from remotes and store it in the repository
    #[cfg(feature = "gitoxide-core-blocking-client")]
    Fetch(fetch::Platform),
    #[cfg(feature = "gitoxide-core-blocking-client")]
    Clone(clone::Platform),
    /// Interact with the mailmap.
    #[clap(subcommand)]
    Mailmap(mailmap::Subcommands),
    /// Interact with the remote hosts.
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    Remote(remote::Platform),
    /// Interact with the exclude files like .gitignore.
    #[clap(subcommand)]
    Exclude(exclude::Subcommands),
    #[clap(subcommand)]
    Index(index::Subcommands),
    /// Display overall progress of the gitoxide project as seen from the perspective of git-config.
    Progress,
    Config(config::Platform),
    /// Subcommands that need no git repository to run.
    #[clap(subcommand)]
    Free(free::Subcommands),
}

pub mod config {
    use git::bstr::BString;
    use git_repository as git;

    /// Print all entries in a configuration file or access other sub-commands
    #[derive(Debug, clap::Parser)]
    #[clap(subcommand_required(false))]
    pub struct Platform {
        /// The filter terms to limit the output to matching sections and subsections only.
        ///
        /// Typical filters are `branch` or `remote.origin` or `remote.or*` - git-style globs are supported
        /// and comparisons are case-insensitive.
        #[clap(parse(try_from_os_str = git::env::os_str_to_bstring))]
        pub filter: Vec<BString>,
    }
}

#[cfg(feature = "gitoxide-core-blocking-client")]
pub mod fetch {
    use git_repository as git;

    #[derive(Debug, clap::Parser)]
    pub struct Platform {
        /// Don't change the local repository, but otherwise try to be as accurate as possible.
        #[clap(long, short = 'n')]
        pub dry_run: bool,

        /// Output additional typically information provided by the server as part of the connection handshake.
        #[clap(long, short = 'H')]
        pub handshake_info: bool,

        /// The name of the remote to connect to, or the url of the remote to connect to directly.
        ///
        /// If unset, the current branch will determine the remote.
        #[clap(long, short = 'r')]
        pub remote: Option<String>,

        /// Override the built-in and configured ref-specs with one or more of the given ones.
        #[clap(parse(try_from_os_str = git::env::os_str_to_bstring))]
        pub ref_spec: Vec<git_repository::bstr::BString>,
    }
}

#[cfg(feature = "gitoxide-core-blocking-client")]
pub mod clone {
    use std::{ffi::OsString, path::PathBuf};

    #[derive(Debug, clap::Parser)]
    pub struct Platform {
        /// Output additional typically information provided by the server as part of the connection handshake.
        #[clap(long, short = 'H')]
        pub handshake_info: bool,

        /// If set, the clone will be bare and a working tree checkout won't be available.
        #[clap(long)]
        pub bare: bool,

        /// The url of the remote to connect to, like `https://github.com/byron/gitoxide`.
        pub remote: OsString,

        /// The directory to initialize with the new repository and to which all data should be written.
        pub directory: PathBuf,
    }
}

#[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
pub mod remote {
    use git_repository as git;

    #[derive(Debug, clap::Parser)]
    pub struct Platform {
        /// The name of the remote to connect to, or the URL of the remote to connect to directly.
        ///
        /// If unset, the current branch will determine the remote.
        #[clap(long, short = 'n')]
        pub name: Option<String>,

        /// Output additional typically information provided by the server as part of the connection handshake.
        #[clap(long, short = 'H')]
        pub handshake_info: bool,

        /// Subcommands
        #[clap(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, clap::Subcommand)]
    #[clap(visible_alias = "remotes")]
    pub enum Subcommands {
        /// Print all references available on the remote.
        Refs,
        /// Print all references available on the remote as filtered through ref-specs.
        RefMap {
            /// Also display remote references that were sent by the server, but filtered by the refspec locally.
            #[clap(long, short = 'u')]
            show_unmapped_remote_refs: bool,
            /// Override the built-in and configured ref-specs with one or more of the given ones.
            #[clap(parse(try_from_os_str = git::env::os_str_to_bstring))]
            ref_spec: Vec<git_repository::bstr::BString>,
        },
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

pub mod credential {
    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Get the credentials fed for `url=<url>` via STDIN.
        #[clap(visible_alias = "get")]
        Fill,
        /// Approve the information piped via STDIN as obtained with last call to `fill`
        #[clap(visible_alias = "store")]
        Approve,
        /// Try to resolve the given revspec and print the object names.
        #[clap(visible_alias = "erase")]
        Reject,
    }
}

pub mod revision {
    #[derive(Debug, clap::Subcommand)]
    #[clap(visible_alias = "rev", visible_alias = "r")]
    pub enum Subcommands {
        /// List all commits reachable from the given rev-spec.
        #[clap(visible_alias = "l")]
        List { spec: std::ffi::OsString },
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

pub mod index {
    use std::path::PathBuf;

    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommands {
        /// Create an index from a tree-ish.
        #[clap(visible_alias = "read-tree")]
        FromTree {
            /// Overwrite the specified index file if it already exists.
            #[clap(long, short = 'f')]
            force: bool,
            /// Path to the index file to be written.
            /// If none is given it will be kept in memory only as a way to measure performance. One day we will probably write the index
            /// back by default, but that requires us to write more of the index to work.
            #[clap(long, short = 'i')]
            index_output_path: Option<PathBuf>,
            /// A revspec that points to the to generate the index from.
            spec: std::ffi::OsString,
        },
    }
}

///
pub mod free;
