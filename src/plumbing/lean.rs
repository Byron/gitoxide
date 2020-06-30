mod options {
    use argh::FromArgs;
    use std::path::PathBuf;

    #[derive(FromArgs)]
    /// A simple calculation tool
    pub struct Args {
        #[argh(subcommand)]
        pub subcommand: SubCommands,
    }

    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand)]
    pub enum SubCommands {
        VerifyPack(VerifyPack),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "verify-pack")]
    pub struct VerifyPack {
        /// the '.pack' or '.idx' file whose checksum to validate.
        #[argh(positional)]
        pub path: PathBuf,
    }
}

use anyhow::Result;
use git_features::progress;
use gitoxide_core as core;
use std::io::{stderr, stdout};

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = argh::from_env();
    env_logger::init();
    match cli.subcommand {
        SubCommands::VerifyPack(VerifyPack { path }) => core::verify_pack_or_pack_index(
            path,
            progress::Log::new("verify").into(),
            stdout(),
            stderr(),
        ),
    }
}
