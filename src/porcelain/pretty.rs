use anyhow::Result;
use gitoxide_core as core;
use structopt::StructOpt;

mod options {
    use structopt::{clap::AppSettings, StructOpt};

    #[derive(Debug, StructOpt)]
    #[structopt(about = "The pretty git")]
    #[structopt(settings = &[AppSettings::SubcommandRequired,
                        AppSettings::ColoredHelp])]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Initialize the repository in the current directory.
        #[structopt(alias = "initialize")]
        #[structopt(setting = AppSettings::ColoredHelp)]
        Init,
    }
}

pub fn main() -> Result<()> {
    use options::*;
    let args = Args::from_args();
    match args.cmd {
        Subcommands::Init => core::init(),
    }?;
    Ok(())
}
