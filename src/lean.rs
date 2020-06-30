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
        Init(Init),
        VerifyPack(VerifyPack),
    }

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "init")]
    pub struct Init {}

    /// Initialize the repository in the current directory.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "verify-pack")]
    pub struct VerifyPack {
        /// the '.pack' or '.idx' file whose checksum to validate.
        #[argh(option)]
        pub path: PathBuf,
    }
}

use anyhow::Result;
use gitoxide_core as core;
use std::io::{stderr, stdout};

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = argh::from_env();
    match cli.subcommand {
        SubCommands::Init(_) => core::init(),
        SubCommands::VerifyPack(VerifyPack { path }) => {
            core::verify_pack_or_pack_index(path, stdout(), stderr())
        }
    }
}
