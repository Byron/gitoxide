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

    /// display verbose messages and progress information
    #[argh(switch, short = 'v')]
    pub verbose: bool,

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
    IndexFromPack(IndexFromPack),
}

/// Create an index from a packfile.
///
/// This command can also be used to stream packs to standard input or to repair partial packs.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pack-index-from-data")]
pub struct IndexFromPack {
    /// specify how to iterate the pack, defaults to 'verify'
    ///
    /// Valid values are
    ///
    ///  **as-is** do not do anything and expect the pack file to be valid as per the trailing hash,
    ///  **verify** the input ourselves and validate that it matches with the hash provided in the pack,
    ///  **restore** hash the input ourselves and ignore failing entries, instead finish the pack with the hash we computed
    #[argh(option, short = 'i')]
    pub iteration_mode: Option<core::pack::index::IterationMode>,

    /// path to the pack file to read (with .pack extension).
    ///
    /// If unset, the pack file is expected on stdin.
    #[argh(option, short = 'p')]
    pub pack_path: Option<PathBuf>,

    /// the folder into which to place the pack and the generated index file
    ///
    /// If unset, only informational output will be provided to standard output.
    #[argh(positional)]
    pub directory: Option<PathBuf>,
}

/// Explode a pack into loose objects.
///
/// This can be useful in case of partially invalidated packs to extract as much information as possible,
/// or because working with loose objects is easier with custom tooling.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pack-explode")]
pub struct PackExplode {
    #[argh(switch)]
    /// read written objects back and assert they match their source. Fail the operation otherwise.
    ///
    /// Only relevant if an object directory is set.
    pub verify: bool,

    /// delete the pack and index file after the operation is successful
    #[argh(switch)]
    pub delete_pack: bool,

    /// compress bytes even when using the sink, i.e. no object directory is specified
    ///
    /// This helps to determine overhead related to compression. If unset, the sink will
    /// only create hashes from bytes, which is usually limited by the speed at which input
    /// can be obtained.
    #[argh(switch)]
    pub sink_compress: bool,

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
    /// the '.pack' or '.idx' file whose checksum to validate.
    #[argh(positional)]
    pub path: PathBuf,
}
