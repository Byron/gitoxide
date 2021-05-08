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
    Init {
        /// The directory in which to initialize a new git repository.
        ///
        /// Defaults to the current working directory.
        directory: Option<PathBuf>,
    },
    /// A selection of useful tools
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersion)]
    Tools(ToolCommands),
}

#[derive(Debug, Clap)]
pub enum ToolCommands {
    /// Find all repositories in a given directory.
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(setting = AppSettings::DisableVersion)]
    Find {
        /// The directory in which to find all git repositories.
        ///
        /// Defaults to the current working directory.
        root: Option<PathBuf>,
    },
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
