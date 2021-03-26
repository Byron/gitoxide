use anyhow::Result;
use clap::Clap;
use gitoxide_core as core;

use crate::{
    plumbing::pretty::options::{Args, Subcommands},
    shared::pretty::prepare_and_run,
};
use gitoxide_core::pack::verify;

pub fn main() -> Result<()> {
    let Args {
        threads: thread_limit,
        verbose,
        progress,
        progress_keep_open,
        format,
        cmd,
    } = Args::parse();
    git_features::interrupt::init_handler(std::io::stderr());

    match cmd {
        Subcommands::PackReceive {
            protocol,
            url,
            directory,
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
                    git_features::progress::DoOrDiscard::from(progress),
                    core::pack::receive::Context {
                        thread_limit,
                        format,
                        out,
                    },
                )
            },
        ),
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
                core::pack::index::from_pack(
                    pack_path,
                    directory,
                    git_features::progress::DoOrDiscard::from(progress),
                    core::pack::index::Context {
                        thread_limit,
                        iteration_mode,
                        format,
                        out,
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
                        thread_limit,
                        algorithm,
                        mode,
                        out,
                        err,
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
