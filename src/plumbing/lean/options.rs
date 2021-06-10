use argh::FromArgs;
use gitoxide_core as core;
use std::ffi::OsString;
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
    PackCreate(PackCreate),
    PackVerify(PackVerify),
    PackExplode(PackExplode),
    IndexFromPack(IndexFromPack),
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    RemoteRefList(RemoteRefList),
    #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
    PackReceive(PackReceive),
    CommitGraphVerify(CommitGraphVerify),
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

/// List remote references from a remote identified by a url.
///
/// This is the plumbing equivalent of `git ls-remote`.
/// Supported URLs are documented here: https://www.git-scm.com/docs/git-clone#_git_urls
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "remote-ref-list")]
#[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
pub struct RemoteRefList {
    /// the protocol version to use. Valid values are 1 and 2
    #[argh(option, short = 'p')]
    pub protocol: Option<core::net::Protocol>,

    /// the URLs or path from which to receive references
    ///
    /// See here for a list of supported URLs: https://www.git-scm.com/docs/git-clone#_git_urls
    #[argh(positional)]
    pub url: String,
}

/// Receive a pack from a remote identified by a url.
///
/// This is the plumbing equivalent of `git clone` and `git-fetch`.
/// Supported URLs are documented here: https://www.git-scm.com/docs/git-clone#_git_urls
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pack-receive")]
#[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
pub struct PackReceive {
    /// the protocol version to use. Valid values are 1 and 2
    #[argh(option, short = 'p')]
    pub protocol: Option<core::net::Protocol>,

    /// the directory into which to write references. Existing files will be overwritten.
    ///
    /// Note that the directory will be created if needed.
    #[argh(option, short = 'r')]
    pub refs_directory: Option<PathBuf>,

    /// the URLs or path from which to receive the pack.
    ///
    /// See here for a list of supported URLs: https://www.git-scm.com/docs/git-clone#_git_urls
    #[argh(positional)]
    pub url: String,

    /// the directory into which to write the received pack and index.
    ///
    /// If unset, they will be discarded.
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
#[argh(subcommand, name = "pack-create")]
pub struct PackCreate {
    #[argh(option, short = 'r')]
    /// the directory containing the '.git' repository from which objects should be read.
    pub repository: Option<PathBuf>,

    #[argh(option, short = 'e')]
    /// the way objects are expanded. They differ in costs.
    ///
    /// Possible values are "none" and "tree-traversal". Default is "none".
    pub expansion: Option<core::pack::create::ObjectExpansion>,

    /// the tips from which to start the commit graph iteration.
    ///
    /// If empty, we expect to read objects on stdin and default to 'none' as expansion mode.
    /// Otherwise the expansion mode is 'tree-traversal' by default.
    #[argh(positional)]
    pub tips: Vec<OsString>,
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

/// Verify a commit graph
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "commit-graph-verify")]
pub struct CommitGraphVerify {
    /// the path to '.git/objects/info/', '.git/objects/info/commit-graphs/', or '.git/objects/info/commit-graph' to validate.
    #[argh(positional)]
    pub path: PathBuf,

    /// output statistical information about the pack
    #[argh(switch, short = 's')]
    pub statistics: bool,
}
