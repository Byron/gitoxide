mod options {
    use argh::FromArgs;
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
        Organize(Organize),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {}

    /// Move all repositories found in a directory into a structure matching their clone URLs.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "organize")]
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
        SubCommands::Init(_) => core::repository::init(),
        SubCommands::Organize(Organize {
            execute,
            repository_source,
            destination_directory,
        }) => {
            use gitoxide_core::util::organize;
            core::util::organize(
                if execute {
                    organize::Mode::Execute
                } else {
                    organize::Mode::Simulate
                },
                repository_source.unwrap_or_else(|| std::env::current_dir().expect("CWD as default source")),
                destination_directory.unwrap_or_else(|| std::env::current_dir().expect("CWD as default destination")),
                git_features::progress::Discard,
            )
        }
    }
}
