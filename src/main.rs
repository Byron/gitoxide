#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use structopt::StructOpt;

mod options {
    use std::path::PathBuf;
    use structopt::clap::AppSettings;
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(about = "The git, simply swift")]
    #[structopt(settings = &[AppSettings::SubcommandRequired,
                            AppSettings::ColoredHelp])]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    /// Low-level commands that are not used every day
    #[derive(Debug, StructOpt)]
    pub enum Plumbing {
        /// Verify the integrity of a pack or index file
        #[structopt(setting = AppSettings::ColoredHelp)]
        VerifyPack {
            /// The '.pack' file whose checksum to validate.
            ///
            /// '.pack' files have a 20 byte trailer representing the Sha1 over all the bytes that
            /// preceded it.
            #[structopt(parse(from_os_str))]
            path: PathBuf,
        },
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Initialize the repository in the current directory.
        #[structopt(alias = "initialize")]
        #[structopt(setting = AppSettings::ColoredHelp)]
        Init,

        #[structopt(alias = "p")]
        #[structopt(setting = AppSettings::ColoredHelp)]
        Plumbing(Plumbing),
    }
}

fn main() -> Result<()> {
    let args = options::Args::from_args();
    match args.cmd {
        options::Subcommands::Init => {
            git_repository::init::repository().with_context(|| "Repository initialization failed")
        }
        options::Subcommands::Plumbing(cmd) => match cmd {
            options::Plumbing::VerifyPack { path } => {
                let pack =
                    git_odb::pack::File::at(path).with_context(|| "Could not open pack file")?;
                pack.verify_checksum().with_context(|| "Failed")?;
                println!("OK");
                Ok(())
            }
        },
    }?;
    Ok(())
}
