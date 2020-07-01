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
        /// if set, verbose progress messages are printed line by line
        #[argh(switch, short = 'v')]
        pub verbose: bool,
        /// the '.pack' or '.idx' file whose checksum to validate.
        #[argh(positional)]
        pub path: PathBuf,
    }
}

use anyhow::Result;
use git_features::progress;
use gitoxide_core as core;
use std::io::{stderr, stdout};

fn init_log(verbose: bool) {
    if verbose {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    } else {
        env_logger::init();
    }
}

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = argh::from_env();
    match cli.subcommand {
        SubCommands::VerifyPack(VerifyPack { path, verbose }) => {
            init_log(verbose);
            core::verify_pack_or_pack_index(
                path,
                progress::Log::new("verify-pack").into(),
                stdout(),
                stderr(),
            )
        }
    }
}
