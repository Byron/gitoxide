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

    /// Move all repositories found in a given root directory into a structure matching their clone URLs.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "organize")]
    pub struct Organize {
        #[argh(positional)]
        /// the root directory to use when finding input repositories to move into position.
        ///
        /// Defaults to the current working directory.
        pub root: Option<PathBuf>,
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
        SubCommands::Organize(_cmd) => unimplemented!("organize"),
    }
}
