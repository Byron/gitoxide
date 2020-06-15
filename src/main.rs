use anyhow::{Context, Result};
use git_core;
use structopt::StructOpt;

mod options {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(about = "The git, simply swift")]
    #[structopt(setting = structopt::clap::AppSettings::SubcommandRequired)]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Initialize the repository in the current directory.
        #[structopt(alias = "initialize")]
        Init,
    }
}

fn main() -> Result<()> {
    let args = options::Args::from_args();
    match args.cmd {
        options::Subcommands::Init => {
            git_core::init::repository().with_context(|| "Repository initialization failed")
        }
    }?;
    Ok(())
}
