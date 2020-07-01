use anyhow::Result;
use git_features::progress;
use gitoxide_core as core;
use std::io::{stderr, stdout};
use structopt::StructOpt;

mod options {
    use std::path::PathBuf;
    use structopt::{clap::AppSettings, StructOpt};

    #[derive(Debug, StructOpt)]
    #[structopt(about = "The git, simply swift")]
    #[structopt(settings = &[AppSettings::SubcommandRequired,
                        AppSettings::ColoredHelp])]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Verify the integrity of a pack or index file
        #[structopt(setting = AppSettings::ColoredHelp)]
        VerifyPack {
            /// if set, verbose progress messages are printed line by line
            #[structopt(long, short = "v")]
            verbose: bool,
            /// The '.pack' or '.idx' file whose checksum to validate.
            #[structopt(parse(from_os_str))]
            path: PathBuf,
        },
    }
}

fn init_progress(name: &str, verbose: bool) -> progress::DoOrDiscard<progress::Log> {
    super::init_env_logger(verbose);
    progress::DoOrDiscard::from(if verbose {
        Some(progress::Log::new(name))
    } else {
        None
    })
}

pub fn main() -> Result<()> {
    use options::*;
    let args = Args::from_args();
    match args.cmd {
        Subcommands::VerifyPack { path, verbose } => core::verify_pack_or_pack_index(
            path,
            init_progress("verify-pack", verbose).into(),
            stdout(),
            stderr(),
        ),
    }?;
    Ok(())
}
