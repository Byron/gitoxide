use std::{
    io::{stdin, BufReader},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use anyhow::Result;
use clap::Clap;
use gitoxide_core as core;
use gitoxide_core::pack::verify;

use crate::{
    plumbing::options::{Args, Subcommands},
    shared::prepare_and_run,
};

pub fn main() -> Result<()> {
    let Args {
        threads: thread_limit,
        verbose,
        progress,
        progress_keep_open,
        format,
        cmd,
    } = Args::parse();
    let should_interrupt = Arc::new(AtomicBool::new(false));
    git_repository::interrupt::init_handler({
        let should_interrupt = Arc::clone(&should_interrupt);
        move || should_interrupt.store(true, Ordering::SeqCst)
    })?;

    match cmd {
        Subcommands::PackCreate {
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
                        expansion: expansion.unwrap_or_else(|| {
                            if has_tips {
                                core::pack::create::ObjectExpansion::TreeTraversal
                            } else {
                                core::pack::create::ObjectExpansion::None
                            }
                        }),
                    };
                    let progress = git_features::progress::DoOrDiscard::from(progress);
                    core::pack::create(repository, tips, input, output_directory, progress, context)
                },
            )
        }
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Subcommands::PackReceive {
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
                    },
                )
            },
        ),
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Subcommands::RemoteRefList { protocol, url } => prepare_and_run(
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
        Subcommands::PackIndexFromData {
            iteration_mode,
            pack_path,
            directory,
        } => prepare_and_run(
            "pack-index-from-data",
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
                        anyhow::bail!("Refusing to read from standard input as no path is given, but it's a terminal.")
                    }
                    PathOrRead::Read(Box::new(std::io::stdin()))
                };
                core::pack::index::from_pack(
                    input,
                    directory,
                    git_features::progress::DoOrDiscard::from(progress),
                    core::pack::index::Context {
                        thread_limit,
                        iteration_mode,
                        format,
                        out,
                        should_interrupt: &git_repository::interrupt::IS_INTERRUPTED,
                    },
                )
            },
        ),
        Subcommands::PackExplode {
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
                    },
                )
            },
        ),
        Subcommands::PackVerify {
            path,
            algorithm,
            decode,
            re_encode,
            statistics,
        } => prepare_and_run(
            "pack-verify",
            verbose,
            progress,
            progress_keep_open,
            None,
            move |progress, out, err| {
                let mode = match (decode, re_encode) {
                    (true, false) => verify::Mode::Sha1Crc32Decode,
                    (true, true) | (false, true) => verify::Mode::Sha1Crc32DecodeEncode,
                    (false, false) => verify::Mode::Sha1Crc32,
                };
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
                        should_interrupt,
                    },
                )
            },
        )
        .map(|_| ()),
        Subcommands::CommitGraphVerify { path, statistics } => prepare_and_run(
            "commit-graph-verify",
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
    }?;
    Ok(())
}
