mod options {
    use argh::FromArgs;
    #[cfg(feature = "gitoxide-core-organize")]
    use std::path::PathBuf;

    #[derive(FromArgs)]
    /// The lean git
    pub struct Args {
        #[argh(switch)]
        /// print the program version.
        pub version: bool,

        #[argh(subcommand)]
        pub subcommand: SubCommands,
    }

    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum SubCommands {
        Init(Init),
        #[cfg(feature = "gitoxide-core-organize")]
        Find(Find),
        #[cfg(feature = "gitoxide-core-organize")]
        Organize(Organize),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {
        #[argh(option)]
        pub directory: Option<PathBuf>,
    }

    /// find all repositories in a given directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "find")]
    #[cfg(feature = "gitoxide-core-organize")]
    pub struct Find {
        /// the directory in which to find all git repositories.
        ///
        /// Defaults to the current working directory.
        #[argh(positional)]
        pub root: Option<PathBuf>,
    }

    /// Move all repositories found in a directory into a structure matching their clone URLs.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "organize")]
    #[cfg(feature = "gitoxide-core-organize")]
    pub struct Organize {
        #[argh(switch)]
        /// the operation will be in dry-run mode unless this flag is set.
        pub execute: bool,

        #[argh(option, short = 'f')]
        /// the directory to use when finding input repositories to move into position.
        ///
        /// Defaults to the current working directory.
        pub repository_source: Option<PathBuf>,

        #[argh(option, short = 't')]
        /// the directory to which to move repositories found in the repository-source.
        ///
        /// Defaults to the current working directory.
        pub destination_directory: Option<PathBuf>,
    }
}

use anyhow::Result;
use gitoxide_core as core;

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = crate::shared::from_env();
    git_features::interrupt::init_handler(std::io::stderr());

    match cli.subcommand {
        SubCommands::Init(Init {
            directory
        }) => core::repository::init(),
        #[cfg(feature = "gitoxide-core-organize")]
        SubCommands::Find(Find { root }) => {
            use crate::shared::lean::prepare;
            use git_features::progress::DoOrDiscard;
            use gitoxide_core::organize;
            let verbose = true;
            let (_handle, progress) = prepare(verbose, "find", None);
            organize::discover(
                root.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                std::io::stdout(),
                DoOrDiscard::from(progress),
            )
        }
        #[cfg(feature = "gitoxide-core-organize")]
        SubCommands::Organize(Organize {
            execute,
            repository_source,
            destination_directory,
        }) => {
            use crate::shared::lean::prepare;
            use git_features::progress::DoOrDiscard;
            use gitoxide_core::organize;
            let verbose = true;
            let (_handle, progress) = prepare(verbose, "organize", None);
            organize::run(
                if execute {
                    organize::Mode::Execute
                } else {
                    organize::Mode::Simulate
                },
                repository_source.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                destination_directory.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                DoOrDiscard::from(progress),
            )
        }
    }
}
