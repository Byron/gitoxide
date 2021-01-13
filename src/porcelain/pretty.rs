use crate::shared::pretty::prepare_and_run;
use anyhow::Result;
use clap::Clap;
use git_features::progress::DoOrDiscard;
use gitoxide_core as core;

mod options {
    use clap::{AppSettings, Clap};
    use std::path::PathBuf;

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
        /// Move all repositories found in a directory into a structure matching their clone URLs.
        #[clap(setting = AppSettings::ColoredHelp)]
        #[clap(setting = AppSettings::DisableVersion)]
        Organize {
            #[clap(long)]
            /// The operation will be in dry-run mode unless this flag is set.
            execute: bool,

            #[clap(long, short = 'f')]
            /// The directory to use when finding input repositories to move into position.
            ///
            /// Defaults to the current working directory.
            repository_source: Option<PathBuf>,

            #[clap(long, short = 't')]
            /// The directory to which to move repositories found in the repository-source.
            ///
            /// Defaults to the current working directory.
            destination_directory: Option<PathBuf>,
        },
    }
}

pub fn main() -> Result<()> {
    use options::*;
    let args = Args::parse();
    git_features::interrupt::init_handler(std::io::stderr());
    let verbose = true;

    match args.cmd {
        Subcommands::Init => core::repository::init(),
        Subcommands::Organize {
            execute,
            repository_source,
            destination_directory,
        } => {
            use gitoxide_core::util::organize;
            // force verbose only, being the line renderer.
            let progress = false;
            let progress_keep_open = false;

            prepare_and_run(
                "organize",
                verbose,
                progress,
                progress_keep_open,
                crate::shared::STANDARD_RANGE,
                move |progress, _out, _err| {
                    core::util::organize(
                        if execute {
                            organize::Mode::Execute
                        } else {
                            organize::Mode::Simulate
                        },
                        repository_source.unwrap_or_else(|| std::env::current_dir().expect("CWD as default source")),
                        destination_directory
                            .unwrap_or_else(|| std::env::current_dir().expect("CWD as default destination")),
                        DoOrDiscard::from(progress),
                    )
                },
            )
        }
    }?;
    Ok(())
}
