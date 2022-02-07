use std::{
    io::{stdin, BufReader},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use anyhow::Result;
use clap::Parser;
use gitoxide_core as core;
use gitoxide_core::pack::verify;

#[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
use crate::plumbing::options::remote;
use crate::{
    plumbing::options::{commitgraph, index, pack, repo, Args, Subcommands},
    shared::pretty::prepare_and_run,
};

#[cfg(feature = "gitoxide-core-async-client")]
pub mod async_util {
    use crate::shared::ProgressRange;

    #[cfg(not(feature = "prodash-render-line"))]
    compile_error!("BUG: Need at least a line renderer in async mode");

    pub fn prepare(
        verbose: bool,
        name: &str,
        range: impl Into<Option<ProgressRange>>,
    ) -> (Option<prodash::render::line::JoinHandle>, Option<prodash::tree::Item>) {
        use crate::shared::{self, STANDARD_RANGE};
        crate::shared::init_env_logger(false);

        if verbose {
            let progress = crate::shared::progress_tree();
            let sub_progress = progress.add_child(name);
            let ui_handle = shared::setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE));
            (Some(ui_handle), Some(sub_progress))
        } else {
            (None, None)
        }
    }
}

pub fn main() -> Result<()> {
    let args: Args = Args::parse();
    let thread_limit = args.threads;
    let verbose = args.verbose;
    let format = args.format;
    let cmd = args.cmd;
    let object_hash = args.object_hash;

    let progress;
    let progress_keep_open;
    #[cfg(feature = "prodash-render-tui")]
    {
        progress = args.progress;
        progress_keep_open = args.progress_keep_open;
    }
    #[cfg(not(feature = "prodash-render-tui"))]
    {
        progress = false;
        progress_keep_open = false;
    }

    let should_interrupt = Arc::new(AtomicBool::new(false));
    git_repository::interrupt::init_handler({
        let should_interrupt = Arc::clone(&should_interrupt);
        move || should_interrupt.store(true, Ordering::SeqCst)
    })?;

    match cmd {
        Subcommands::Index(index::Platform {
            object_hash,
            index_path,
            cmd,
        }) => match cmd {
            index::Subcommands::Info { no_details } => prepare_and_run(
                "index-entries",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::index::information(
                        index_path,
                        out,
                        err,
                        core::index::information::Options {
                            index: core::index::Options { object_hash, format },
                            extension_details: !no_details,
                        },
                    )
                },
            ),
            index::Subcommands::Entries => prepare_and_run(
                "index-entries",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::index::entries(index_path, out, core::index::Options { object_hash, format })
                },
            ),
            index::Subcommands::Verify => prepare_and_run(
                "index-verify",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::index::verify(index_path, out, core::index::Options { object_hash, format })
                },
            ),
        },
        Subcommands::Repository(subcommands) => match subcommands {
            repo::Subcommands::Verify {
                args:
                    pack::VerifyOptions {
                        statistics,
                        algorithm,
                        decode,
                        re_encode,
                    },
                repository,
            } => prepare_and_run(
                "repository-verify",
                verbose,
                progress,
                progress_keep_open,
                core::repository::verify::PROGRESS_RANGE,
                move |progress, out, _err| {
                    core::repository::verify::integrity(
                        repository,
                        out,
                        progress,
                        &should_interrupt,
                        core::repository::verify::Context {
                            output_statistics: statistics.then(|| format),
                            algorithm,
                            verify_mode: verify_mode(decode, re_encode),
                            thread_limit,
                        },
                    )
                },
            ),
        },
        Subcommands::Pack(subcommands) => match subcommands {
            pack::Subcommands::Create {
                repository,
                expansion,
                thin,
                statistics,
                nondeterministic_count,
                tips,
                pack_cache_size_mb,
                object_cache_size_mb,
                output_directory,
            } => {
                let has_tips = !tips.is_empty();
                prepare_and_run(
                    "pack-create",
                    verbose,
                    progress,
                    progress_keep_open,
                    core::pack::create::PROGRESS_RANGE,
                    move |progress, out, _err| {
                        let input = if has_tips {
                            None
                        } else {
                            if atty::is(atty::Stream::Stdin) {
                                anyhow::bail!(
                                    "Refusing to read from standard input as no path is given, but it's a terminal."
                                )
                            }
                            Some(BufReader::new(stdin()))
                        };
                        let repository = repository.unwrap_or_else(|| PathBuf::from("."));
                        let context = core::pack::create::Context {
                            thread_limit,
                            thin,
                            nondeterministic_count,
                            pack_cache_size_in_bytes: pack_cache_size_mb.unwrap_or(0) * 1_000_000,
                            object_cache_size_in_bytes: object_cache_size_mb.unwrap_or(0) * 1_000_000,
                            statistics: if statistics { Some(format) } else { None },
                            out,
                            expansion: expansion.unwrap_or(if has_tips {
                                core::pack::create::ObjectExpansion::TreeTraversal
                            } else {
                                core::pack::create::ObjectExpansion::None
                            }),
                        };
                        core::pack::create(repository, tips, input, output_directory, progress, context)
                    },
                )
            }
            #[cfg(feature = "gitoxide-core-async-client")]
            pack::Subcommands::Receive {
                protocol,
                url,
                directory,
                refs,
                refs_directory,
            } => {
                let (_handle, progress) =
                    async_util::prepare(verbose, "pack-receive", core::pack::receive::PROGRESS_RANGE);
                let fut = core::pack::receive(
                    protocol,
                    &url,
                    directory,
                    refs_directory,
                    refs.into_iter().map(|s| s.into()).collect(),
                    git_features::progress::DoOrDiscard::from(progress),
                    core::pack::receive::Context {
                        thread_limit,
                        format,
                        out: std::io::stdout(),
                        should_interrupt,
                        object_hash,
                    },
                );
                return futures_lite::future::block_on(fut);
            }
            #[cfg(feature = "gitoxide-core-blocking-client")]
            pack::Subcommands::Receive {
                protocol,
                url,
                directory,
                refs,
                refs_directory,
            } => prepare_and_run(
                "pack-receive",
                verbose,
                progress,
                progress_keep_open,
                core::pack::receive::PROGRESS_RANGE,
                move |progress, out, _err| {
                    core::pack::receive(
                        protocol,
                        &url,
                        directory,
                        refs_directory,
                        refs.into_iter().map(|r| r.into()).collect(),
                        git_features::progress::DoOrDiscard::from(progress),
                        core::pack::receive::Context {
                            thread_limit,
                            format,
                            should_interrupt,
                            out,
                            object_hash,
                        },
                    )
                },
            ),
            pack::Subcommands::Explode {
                check,
                sink_compress,
                delete_pack,
                pack_path,
                object_path,
                verify,
            } => prepare_and_run(
                "pack-explode",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |progress, _out, _err| {
                    core::pack::explode::pack_or_pack_index(
                        pack_path,
                        object_path,
                        check,
                        progress,
                        core::pack::explode::Context {
                            thread_limit,
                            delete_pack,
                            sink_compress,
                            verify,
                            should_interrupt,
                            object_hash,
                        },
                    )
                },
            ),
            pack::Subcommands::Verify {
                args:
                    pack::VerifyOptions {
                        algorithm,
                        decode,
                        re_encode,
                        statistics,
                    },
                path,
            } => prepare_and_run(
                "pack-verify",
                verbose,
                progress,
                progress_keep_open,
                verify::PROGRESS_RANGE,
                move |progress, out, err| {
                    let mode = verify_mode(decode, re_encode);
                    let output_statistics = if statistics { Some(format) } else { None };
                    verify::pack_or_pack_index(
                        path,
                        progress,
                        verify::Context {
                            output_statistics,
                            out,
                            err,
                            thread_limit,
                            mode,
                            algorithm,
                            should_interrupt: &should_interrupt,
                            object_hash,
                        },
                    )
                },
            )
            .map(|_| ()),
            pack::Subcommands::MultiIndex(subcommands) => match subcommands {
                pack::multi_index::Subcommands::Verify { multi_index_path } => prepare_and_run(
                    "pack-multi-index-verify",
                    verbose,
                    progress,
                    progress_keep_open,
                    core::pack::multi_index::PROGRESS_RANGE,
                    move |progress, _out, _err| {
                        core::pack::multi_index::verify(
                            multi_index_path,
                            progress,
                            &git_repository::interrupt::IS_INTERRUPTED,
                        )
                    },
                ),
                pack::multi_index::Subcommands::Create {
                    output_path,
                    index_paths,
                } => prepare_and_run(
                    "pack-multi-index-create",
                    verbose,
                    progress,
                    progress_keep_open,
                    core::pack::multi_index::PROGRESS_RANGE,
                    move |progress, _out, _err| {
                        core::pack::multi_index::create(
                            index_paths,
                            output_path,
                            progress,
                            &git_repository::interrupt::IS_INTERRUPTED,
                            object_hash,
                        )
                    },
                ),
            },
            pack::Subcommands::Index(subcommands) => match subcommands {
                pack::index::Subcommands::Create {
                    iteration_mode,
                    pack_path,
                    directory,
                } => prepare_and_run(
                    "pack-index-create",
                    verbose,
                    progress,
                    progress_keep_open,
                    core::pack::index::PROGRESS_RANGE,
                    move |progress, out, _err| {
                        use gitoxide_core::pack::index::PathOrRead;
                        let input = if let Some(path) = pack_path {
                            PathOrRead::Path(path)
                        } else {
                            if atty::is(atty::Stream::Stdin) {
                                anyhow::bail!(
                                    "Refusing to read from standard input as no path is given, but it's a terminal."
                                )
                            }
                            PathOrRead::Read(Box::new(std::io::stdin()))
                        };
                        core::pack::index::from_pack(
                            input,
                            directory,
                            progress,
                            core::pack::index::Context {
                                thread_limit,
                                iteration_mode,
                                format,
                                out,
                                object_hash,
                                should_interrupt: &git_repository::interrupt::IS_INTERRUPTED,
                            },
                        )
                    },
                ),
            },
        },
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Subcommands::Remote(subcommands) => match subcommands {
            #[cfg(feature = "gitoxide-core-async-client")]
            remote::Subcommands::RefList { protocol, url } => {
                let (_handle, progress) =
                    async_util::prepare(verbose, "remote-ref-list", Some(core::remote::refs::PROGRESS_RANGE));
                let fut = core::remote::refs::list(
                    protocol,
                    &url,
                    git_features::progress::DoOrDiscard::from(progress),
                    core::remote::refs::Context {
                        thread_limit,
                        format,
                        out: std::io::stdout(),
                    },
                );
                return futures_lite::future::block_on(fut);
            }
            #[cfg(feature = "gitoxide-core-blocking-client")]
            remote::Subcommands::RefList { protocol, url } => prepare_and_run(
                "remote-ref-list",
                verbose,
                progress,
                progress_keep_open,
                core::remote::refs::PROGRESS_RANGE,
                move |progress, out, _err| {
                    core::remote::refs::list(
                        protocol,
                        &url,
                        git_features::progress::DoOrDiscard::from(progress),
                        core::remote::refs::Context {
                            thread_limit,
                            format,
                            out,
                        },
                    )
                },
            ),
        },
        Subcommands::CommitGraph(subcommands) => match subcommands {
            commitgraph::Subcommands::Verify { path, statistics } => prepare_and_run(
                "commitgraph-verify",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    let output_statistics = if statistics { Some(format) } else { None };
                    core::commitgraph::verify::graph_or_file(
                        path,
                        core::commitgraph::verify::Context {
                            err,
                            out,
                            output_statistics,
                        },
                    )
                },
            )
            .map(|_| ()),
        },
    }?;
    Ok(())
}

fn verify_mode(decode: bool, re_encode: bool) -> verify::Mode {
    match (decode, re_encode) {
        (true, false) => verify::Mode::HashCrc32Decode,
        (true, true) | (false, true) => verify::Mode::HashCrc32DecodeEncode,
        (false, false) => verify::Mode::HashCrc32,
    }
}
