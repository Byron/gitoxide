mod options {
    use argh::FromArgs;
    use std::path::PathBuf;

    #[derive(FromArgs)]
    #[argh(name = "gio-plumbing")]
    /// The lean git underworld
    pub struct Args {
        #[argh(switch)]
        /// print the program version.
        pub version: bool,

        #[argh(option, short = 't')]
        /// the amount of threads to use for some operations.
        ///
        /// If unset, or the value is 0, there is no limit and all logical cores can be used.
        pub threads: Option<usize>,

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
        /// output statistical information about the pack
        #[argh(switch, short = 's')]
        pub statistics: bool,
        /// verbose progress messages are printed line by line
        #[argh(switch, short = 'v')]
        pub verbose: bool,
        /// the '.pack' or '.idx' data whose checksum to validate.
        #[argh(positional)]
        pub path: PathBuf,
    }
}

use anyhow::Result;
use gitoxide_core as core;
use std::io::{stderr, stdout};

#[cfg(not(any(
    feature = "prodash-line-renderer-crossterm",
    feature = "prodash-line-renderer-termion"
)))]
fn prepare(verbose: bool, name: &str) -> ((), Option<git_features::progress::Log>) {
    super::init_env_logger(verbose);
    ((), Some(git_features::progress::Log::new(name, Some(1))))
}

#[cfg(any(
    feature = "prodash-line-renderer-crossterm",
    feature = "prodash-line-renderer-termion"
))]
fn prepare(verbose: bool, name: &str) -> (Option<prodash::line::JoinHandle>, Option<prodash::tree::Item>) {
    super::init_env_logger(false);

    if verbose {
        let progress = prodash::Tree::new();
        let sub_progress = progress.add_child(name);
        let handle = crate::shared::setup_line_renderer(progress, 2, false);
        (Some(handle), Some(sub_progress))
    } else {
        (None, None)
    }
}

pub fn main() -> Result<()> {
    pub use options::*;
    let cli: Args = crate::shared::from_env();
    let thread_limit = cli.threads;
    match cli.subcommand {
        SubCommands::VerifyPack(VerifyPack {
            path,
            verbose,
            statistics,
        }) => {
            let (_handle, progress) = prepare(verbose, "verify-pack");
            core::verify_pack_or_pack_index(
                path,
                progress,
                core::Context {
                    output_statistics: if statistics {
                        Some(core::OutputFormat::Human)
                    } else {
                        None
                    },
                    thread_limit,
                    out: stdout(),
                    err: stderr(),
                },
            )
            .map(|_| ())
        }
    }
}
