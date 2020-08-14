mod options {
    use argh::FromArgs;

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
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {}
}

use anyhow::Result;
use gitoxide_core as core;

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = crate::shared::from_env();
    git_features::interrupt::init_handler(std::io::stderr());

    match cli.subcommand {
        SubCommands::Init(_) => core::repository::init(),
    }
}
