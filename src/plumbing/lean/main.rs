use anyhow::Result;
use git_features::progress::DoOrDiscard;
use gitoxide_core::{self as core, OutputFormat};
use std::io::{self, stderr, stdout};

use crate::shared::lean::prepare;

pub fn main() -> Result<()> {
    pub use crate::plumbing::lean::options::*;
    let cli: Args = crate::shared::from_env();
    git_features::interrupt::init_handler(std::io::stderr());
    let thread_limit = cli.threads;
    let verbose = cli.verbose;
    match cli.subcommand {
        SubCommands::RemoteRefList(RemoteRefList { protocol, url }) => {
            let (_handle, progress) = prepare(verbose, "remote-ref-list", Some(core::remote::refs::PROGRESS_RANGE));
            core::remote::refs::list(
                protocol,
                &url,
                DoOrDiscard::from(progress),
                core::remote::refs::Context {
                    thread_limit,
                    format: OutputFormat::Human,
                    out: io::stdout(),
                },
            )
        }
        SubCommands::PackReceive(PackReceive {
            protocol,
            url,
            directory,
            refs_directory,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-receive", core::pack::receive::PROGRESS_RANGE);
            core::pack::receive(
                protocol,
                &url,
                directory,
                refs_directory,
                DoOrDiscard::from(progress),
                core::pack::receive::Context {
                    thread_limit,
                    format: OutputFormat::Human,
                    out: io::stdout(),
                },
            )
        }
        SubCommands::IndexFromPack(IndexFromPack {
            iteration_mode,
            pack_path,
            directory,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-explode", core::pack::index::PROGRESS_RANGE);
            core::pack::index::from_pack(
                pack_path,
                directory,
                DoOrDiscard::from(progress),
                core::pack::index::Context {
                    thread_limit,
                    iteration_mode: iteration_mode.unwrap_or_default(),
                    format: OutputFormat::Human,
                    out: io::stdout(),
                },
            )
        }
        SubCommands::PackExplode(PackExplode {
            pack_path,
            sink_compress,
            object_path,
            verify,
            check,
            delete_pack,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-explode", None);
            core::pack::explode::pack_or_pack_index(
                pack_path,
                object_path,
                check.unwrap_or_default(),
                progress,
                core::pack::explode::Context {
                    thread_limit,
                    delete_pack,
                    sink_compress,
                    verify,
                },
            )
        }
        SubCommands::PackVerify(PackVerify {
            path,
            statistics,
            algorithm,
            decode,
            re_encode,
        }) => {
            use self::core::pack::verify;
            let (_handle, progress) = prepare(verbose, "pack-verify", None);
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
        SubCommands::CommitGraphVerify(CommitGraphVerify { path, statistics }) => {
            use self::core::commitgraph::verify;

            verify::graph_or_file(
                path,
                verify::Context {
                    err: stderr(),
                    out: stdout(),
                    output_statistics: if statistics {
                        Some(core::OutputFormat::Human)
                    } else {
                        None
                    },
                },
            )
            .map(|_| ())
        }
    }
}
