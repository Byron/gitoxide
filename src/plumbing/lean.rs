mod options {
    use argh::FromArgs;
    use gitoxide_core as core;
    use std::path::PathBuf;

    #[derive(FromArgs)]
    #[argh(name = "gix-plumbing")]
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
        PackVerify(PackVerify),
        PackExplode(PackExplode),
    }
    /// Explode a pack into loose objects.
    ///
    /// This can be useful in case of partially invalidated packs to extract as much information as possible,
    /// or because working with loose objects is easier with custom tooling.
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "pack-explode")]
    pub struct PackExplode {
        /// delete the pack and index file after the operation is successful
        #[argh(switch)]
        pub delete_pack: bool,

        /// display verbose messages and progress information
        #[argh(switch, short = 'v')]
        pub verbose: bool,

        /// the amount of checks to run. Defaults to 'all'.
        ///
        /// Allowed values:
        /// all
        /// skip-file-checksum
        /// skip-file-and-object-checksum
        /// skip-file-and-object-checksum-and-no-abort-on-decode
        #[argh(option, short = 'c')]
        pub check: Option<core::pack::explode::SafetyCheck>,

        /// the '.pack' or '.idx' file to explode into loose objects
        #[argh(positional)]
        pub pack_path: PathBuf,

        /// the path into which all objects should be written. Commonly '.git/objects'
        #[argh(positional)]
        pub object_path: Option<PathBuf>,
    }

    /// Verify a pack
    #[derive(FromArgs, PartialEq, Debug)]
    #[argh(subcommand, name = "pack-verify")]
    pub struct PackVerify {
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
        pub algorithm: Option<core::pack::verify::Algorithm>,

        /// output statistical information about the pack
        #[argh(switch, short = 's')]
        pub statistics: bool,
        /// display verbose messages and progress information
        #[argh(switch, short = 'v')]
        pub verbose: bool,
        /// the '.pack' or '.idx' file whose checksum to validate.
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
        SubCommands::PackExplode(PackExplode {
            pack_path,
            object_path,
            verbose,
            check,
            delete_pack,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-explode");
            core::pack::explode::pack_or_pack_index(
                pack_path,
                object_path,
                check.unwrap_or(core::pack::explode::SafetyCheck::All),
                progress,
                delete_pack,
            )
        }
        SubCommands::PackVerify(PackVerify {
            path,
            verbose,
            statistics,
            algorithm,
            decode,
            re_encode,
        }) => {
            use self::core::pack::verify;
            let (_handle, progress) = prepare(verbose, "pack-verify");
            core::pack::verify::pack_or_pack_index(
                path,
                progress,
                core::pack::verify::Context {
                    output_statistics: if statistics {
                        Some(core::OutputFormat::Human)
                    } else {
                        None
                    },
                    algorithm: algorithm.unwrap_or(verify::Algorithm::LessTime),
                    thread_limit,
                    mode: match (decode, re_encode) {
                        (true, false) => verify::Mode::Sha1CRC32Decode,
                        (true, true) | (false, true) => verify::Mode::Sha1CRC32DecodeEncode,
                        (false, false) => verify::Mode::Sha1CRC32,
                    },
                    out: stdout(),
                    err: stderr(),
                },
            )
            .map(|_| ())
        }
    }
}
