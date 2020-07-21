mod options {
    use argh::FromArgs;
    use gitoxide_core as core;
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
        #[argh(switch)]
        /// decode and parse tags, commits and trees to validate their correctness beyond hashing correctly.
        ///
        /// Malformed objects should not usually occur, but could be injected on purpose or accident.
        /// This will reduce overall performance.
        pub decode: bool,

        #[argh(switch)]
        /// decode and parse tags, commits and trees to validate their correctness, and re-encode them.
        ///
        /// This flag is primarily to test the implementation of encoding, and requires to decode the object first.
        /// Encoding an object after decoding it should yield exactly the same bytes.
        /// This will reduce overall performance even more, as re-encoding requires to transform zero-copy objects into
        /// owned objects, causing plenty of allocation to occour.
        pub re_encode: bool,

        #[argh(option)]
        /// the algorithm used to verify the pack. They differ in costs.
        ///
        /// Possible values are "less-time" and "less-memory". Default is "less-memory".
        pub algorithm: Option<core::VerifyAlgorithm>,

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
            algorithm,
            decode,
            re_encode,
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
                    algorithm: algorithm.unwrap_or(core::VerifyAlgorithm::LessTime),
                    thread_limit,
                    mode: match (decode, re_encode) {
                        (true, false) => core::VerifyMode::Sha1CRC32Decode,
                        (true, true) | (false, true) => core::VerifyMode::Sha1CRC32DecodeEncode,
                        (false, false) => core::VerifyMode::Sha1CRC32,
                    },
                    out: stdout(),
                    err: stderr(),
                },
            )
            .map(|_| ())
        }
    }
}
