use crate::shared::ProgressRange;
use anyhow::Result;
use git_features::progress;
use gitoxide_core::{self as core, OutputFormat};
use std::io::{self, stderr, stdout};

#[cfg(not(any(feature = "prodash-render-line-crossterm", feature = "prodash-render-line-termion")))]
fn prepare(verbose: bool, name: &str, _: impl Into<Option<ProgressRange>>) -> ((), Option<prodash::progress::Log>) {
    crate::plumbing::init_env_logger(verbose);
    ((), Some(prodash::progress::Log::new(name, Some(1))))
}

#[cfg(any(feature = "prodash-render-line-crossterm", feature = "prodash-render-line-termion"))]
fn prepare(
    verbose: bool,
    name: &str,
    range: impl Into<Option<ProgressRange>>,
) -> (Option<prodash::render::line::JoinHandle>, Option<prodash::tree::Item>) {
    use crate::shared::{self, STANDARD_RANGE};
    crate::plumbing::init_env_logger(false);

    if verbose {
        let progress = prodash::Tree::new();
        let sub_progress = progress.add_child(name);
        let handle = shared::setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE), true);
        (Some(handle), Some(sub_progress))
    } else {
        (None, None)
    }
}

pub fn main() -> Result<()> {
    pub use crate::plumbing::lean::options::*;
    let cli: Args = crate::shared::from_env();
    git_features::interrupt::init_handler(std::io::stderr());
    let thread_limit = cli.threads;
    let verbose = cli.verbose;
    match cli.subcommand {
        SubCommands::RemoteRefList(RemoteRefList { protocol, url }) => {
            let (_handle, progress) = prepare(verbose, "remote-ref-list", None);
            core::remote::refs::list(
                protocol,
                &url,
                progress::DoOrDiscard::from(progress),
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
        }) => {
            let (_handle, progress) = prepare(verbose, "pack-receive", None);
            core::pack::receive(
                protocol,
                &url,
                directory,
                progress::DoOrDiscard::from(progress),
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
                progress::DoOrDiscard::from(progress),
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
    }
}
