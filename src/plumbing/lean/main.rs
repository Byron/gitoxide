use crate::{
    plumbing::lean::options::{self, Args, SubCommands},
    shared::lean::prepare,
};
use anyhow::Result;
use git_features::progress::DoOrDiscard;
use gitoxide_core::{self as core, OutputFormat};
use std::{
    io::{self, stderr, stdin, stdout},
    path::PathBuf,
    sync::atomic::AtomicBool,
    sync::Arc,
};
#[cfg(all(feature = "gitoxide-core-blocking-client", feature = "gitoxide-core-async-client"))]
compile_error!("Please set only one of the client networking options.");

pub fn main() -> Result<()> {
    let cli: Args = crate::shared::from_env();
    let should_interrupt = Arc::new(AtomicBool::new(false));
    git_repository::interrupt::init_handler(Arc::clone(&should_interrupt))?;
    let thread_limit = cli.threads;
    let verbose = cli.verbose;
    match cli.subcommand {
        SubCommands::PackCreate(options::PackCreate {
            repository,
            expansion,
            nondeterministic_count,
            statistics,
            tips,
            output_directory,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-create", Some(core::pack::create::PROGRESS_RANGE));
            let has_tips = !tips.is_empty();
            let input = if has_tips {
                None
            } else {
                #[cfg(feature = "atty")]
                if atty::is(atty::Stream::Stdin) {
                    anyhow::bail!("Refusing to read from standard input as no path is given, but it's a terminal.")
                }
                Some(io::BufReader::new(stdin()))
            };
            let expansion = expansion.unwrap_or_else(|| {
                if has_tips {
                    core::pack::create::ObjectExpansion::TreeTraversal
                } else {
                    core::pack::create::ObjectExpansion::None
                }
            });
            core::pack::create(
                repository.unwrap_or_else(|| PathBuf::from(".")),
                tips,
                input,
                output_directory,
                DoOrDiscard::from(progress),
                core::pack::create::Context {
                    expansion,
                    nondeterministic_count,
                    statistics: if statistics { Some(OutputFormat::Human) } else { None },
                    out: stdout(),
                    thread_limit,
                },
            )
        }
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        SubCommands::RemoteRefList(options::RemoteRefList { protocol, url }) => {
            let (_handle, progress) = prepare(verbose, "remote-ref-list", Some(core::remote::refs::PROGRESS_RANGE));
            let res = core::remote::refs::list(
                protocol,
                &url,
                DoOrDiscard::from(progress),
                core::remote::refs::Context {
                    thread_limit,
                    format: OutputFormat::Human,
                    out: io::stdout(),
                },
            );
            #[cfg(feature = "gitoxide-core-blocking-client")]
            return res;
            #[cfg(feature = "gitoxide-core-async-client")]
            return futures_lite::future::block_on(res);
        }
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        SubCommands::PackReceive(options::PackReceive {
            protocol,
            url,
            directory,
            refs_directory,
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-receive", core::pack::receive::PROGRESS_RANGE);
            let res = core::pack::receive(
                protocol,
                &url,
                directory,
                refs_directory,
                DoOrDiscard::from(progress),
                core::pack::receive::Context {
                    thread_limit,
                    format: OutputFormat::Human,
                    out: io::stdout(),
                    should_interrupt,
                },
            );
            #[cfg(feature = "gitoxide-core-blocking-client")]
            return res;
            #[cfg(feature = "gitoxide-core-async-client")]
            return futures_lite::future::block_on(res);
        }
        SubCommands::IndexFromPack(options::IndexFromPack {
            iteration_mode,
            pack_path,
            directory,
        }) => {
            use gitoxide_core::pack::index::PathOrRead;
            let (_handle, progress) = prepare(verbose, "pack-explode", core::pack::index::PROGRESS_RANGE);
            let input = if let Some(path) = pack_path {
                PathOrRead::Path(path)
            } else {
                #[cfg(feature = "atty")]
                if atty::is(atty::Stream::Stdin) {
                    anyhow::bail!("Refusing to read from standard input as no path is given, but it's a terminal.")
                }
                PathOrRead::Read(Box::new(std::io::stdin()))
            };
            core::pack::index::from_pack(
                input,
                directory,
                DoOrDiscard::from(progress),
                core::pack::index::Context {
                    thread_limit,
                    iteration_mode: iteration_mode.unwrap_or_default(),
                    format: OutputFormat::Human,
                    out: io::stdout(),
                    should_interrupt: &git_repository::interrupt::IS_INTERRUPTED,
                },
            )
        }
        SubCommands::PackExplode(options::PackExplode {
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
                    should_interrupt,
                },
            )
        }
        SubCommands::PackVerify(options::PackVerify {
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
                        (true, false) => verify::Mode::Sha1Crc32Decode,
                        (true, true) | (false, true) => verify::Mode::Sha1Crc32DecodeEncode,
                        (false, false) => verify::Mode::Sha1Crc32,
                    },
                    out: stdout(),
                    err: stderr(),
                    should_interrupt,
                },
            )
            .map(|_| ())
        }
        SubCommands::CommitGraphVerify(options::CommitGraphVerify { path, statistics }) => {
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
