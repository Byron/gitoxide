use anyhow::{anyhow, Result};
use clap::Clap;
use gitoxide_core as core;
use std::io::{stderr, stdout, Write};

use crate::plumbing::pretty::options::*;
use gitoxide_core::pack::verify;

use crate::shared::ProgressRange;

fn prepare_and_run<T: Send + 'static>(
    name: &str,
    verbose: bool,
    progress: bool,
    progress_keep_open: bool,
    range: impl Into<Option<ProgressRange>>,
    run: impl FnOnce(Option<prodash::tree::Item>, &mut dyn std::io::Write, &mut dyn std::io::Write) -> Result<T>
        + Send
        + 'static,
) -> Result<T> {
    use crate::shared::{self, STANDARD_RANGE};
    crate::plumbing::init_env_logger(false);
    use git_features::interrupt::{is_triggered, trigger};

    match (verbose, progress) {
        (false, false) => run(None, &mut stdout(), &mut stderr()),
        (true, false) => {
            enum Event<T> {
                UIDone,
                ComputationDone(Result<T>),
            };
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let (tx, rx) = std::sync::mpsc::sync_channel::<Event<T>>(1);
            let ui_handle = shared::setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE), true);
            std::thread::spawn({
                let tx = tx.clone();
                move || loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if is_triggered() {
                        tx.send(Event::UIDone).ok();
                        break;
                    }
                }
            });
            std::thread::spawn(move || {
                let res = run(Some(sub_progress), &mut stdout(), &mut stderr());
                tx.send(Event::ComputationDone(res)).ok();
            });
            match rx.recv()? {
                Event::UIDone => {
                    ui_handle.shutdown_and_wait();
                    Err(anyhow!("Operation cancelled by user"))
                }
                Event::ComputationDone(res) => {
                    ui_handle.shutdown_and_wait();
                    res
                }
            }
        }
        (true, true) | (false, true) => {
            enum Event<T> {
                UIDone,
                ComputationDone(Result<T>, Vec<u8>, Vec<u8>),
            };
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let render_tui = prodash::render::tui(
                stdout(),
                progress,
                prodash::render::tui::Options {
                    title: "gitoxide".into(),
                    frames_per_second: crate::shared::DEFAULT_FRAME_RATE,
                    stop_if_empty_progress: !progress_keep_open,
                    throughput: true,
                    ..Default::default()
                },
            )
            .expect("tui to come up without io error");
            let (tx, rx) = std::sync::mpsc::sync_channel::<Event<T>>(1);
            let ui_handle = std::thread::spawn({
                let tx = tx.clone();
                move || {
                    futures_lite::future::block_on(render_tui);
                    tx.send(Event::UIDone).ok();
                }
            });
            std::thread::spawn(move || {
                // We might have something interesting to show, which would be hidden by the alternate screen if there is a progress TUI
                // We know that the printing happens at the end, so this is fine.
                let mut out = Vec::new();
                let mut err = Vec::new();
                let res = run(Some(sub_progress), &mut out, &mut err);
                tx.send(Event::ComputationDone(res, out, err)).ok();
            });
            loop {
                match rx.recv()? {
                    Event::UIDone => {
                        // We don't know why the UI is done, usually it's the user aborting.
                        // We need the computation to stop as well so let's wait for that to happen
                        trigger();
                        continue;
                    }
                    Event::ComputationDone(res, out, err) => {
                        ui_handle.join().ok();
                        stdout().write_all(&out)?;
                        stderr().write_all(&err)?;
                        break res;
                    }
                }
            }
        }
    }
}

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
                    (true, false) => verify::Mode::Sha1CRC32Decode,
                    (true, true) | (false, true) => verify::Mode::Sha1CRC32DecodeEncode,
                    (false, false) => verify::Mode::Sha1CRC32,
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
    }?;
    Ok(())
}
