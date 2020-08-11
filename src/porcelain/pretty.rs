use anyhow::Result;
use clap::Clap;
use gitoxide_core as core;

mod options {
    use clap::{AppSettings, Clap};

    #[derive(Debug, Clap)]
    #[clap(about = "The rusty git", version = clap::crate_version!())]
    #[clap(setting = AppSettings::SubcommandRequired)]
    #[clap(setting = AppSettings::ColoredHelp)]
    pub struct Args {
        #[clap(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, Clap)]
    pub enum Subcommands {
        /// Initialize the repository in the current directory.
        #[clap(alias = "initialize")]
        #[clap(setting = AppSettings::ColoredHelp)]
        #[clap(setting = AppSettings::DisableVersion)]
        Init,
    }
}

pub fn main() -> Result<()> {
    use options::*;
    let args = Args::parse();
    git_features::interruptible::init_interrupt_handler();
    match args.cmd {
        Subcommands::Init => core::repository::init(),
    }?;
    Ok(())
}
