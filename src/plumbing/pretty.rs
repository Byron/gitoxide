use anyhow::{anyhow, Result};
use clap::Clap;
use gitoxide_core as core;
use std::io::{stderr, stdout, Write};

use gitoxide_core::pack::verify;
use options::*;

mod options {
    use clap::{AppSettings, Clap};
    use gitoxide_core as core;
    use std::path::PathBuf;

    #[derive(Debug, Clap)]
    #[clap(name = "gix-plumbing", about = "The git underworld", version = clap::crate_version!())]
    #[clap(setting = AppSettings::SubcommandRequired)]
    #[clap(setting = AppSettings::ColoredHelp)]
    pub struct Args {
        #[clap(long, short = "t")]
        /// The amount of threads to use for some operations.
        ///
        /// If unset, or the value is 0, there is no limit and all logical cores can be used.
        pub threads: Option<usize>,

        /// Display verbose messages and progress information
        #[clap(long, short = "v")]
        pub verbose: bool,

        /// Bring up a terminal user interface displaying progress visually
        #[clap(long, conflicts_with("verbose"))]
        pub progress: bool,

        /// The progress TUI will stay up even though the work is already completed.
        ///
        /// Use this to be able to read progress messages or additional information visible in the TUI log pane.
        #[clap(long, conflicts_with("verbose"), requires("progress"))]
        pub progress_keep_open: bool,

        /// Determine the format to use when outputting statistics.
        #[clap(
            long,
            short = "f",
            default_value = "human",
            possible_values(core::OutputFormat::variants())
        )]
        pub format: core::OutputFormat,

        #[clap(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, Clap)]
    pub enum Subcommands {
        /// Create an index from a packfile.
        ///
        /// This command can also be used to stream packs to standard input or to repair partial packs.
        #[clap(setting = AppSettings::ColoredHelp)]
        #[clap(setting = AppSettings::DisableVersion)]
        PackIndexFromData {
            /// Specify how to iterate the pack, defaults to 'verify'
            ///
            /// Valid values are
            ///
            ///  **as-is** do not do anything and expect the pack file to be valid as per the trailing hash,
            ///  **verify** the input ourselves and validate that it matches with the hash provided in the pack,
            ///  **restore** hash the input ourselves and ignore failing entries, instead finish the pack with the hash we computed
            #[clap(
                long,
                short = "i",
                default_value = "verify",
                possible_values(core::pack::index::IterationMode::variants())
            )]
            iteration_mode: core::pack::index::IterationMode,

            /// Path to the pack file to read (with .pack extension).
            ///
            /// If unset, the pack file is expected on stdin.
            #[clap(long, short = "p")]
            pack_path: Option<PathBuf>,

            /// The folder into which to place the pack and the generated index file
            ///
            /// If unset, only informational output will be provided to standard output.
            #[clap(parse(from_os_str))]
            directory: Option<PathBuf>,
        },
        /// Verify the integrity of a pack or index file
        #[clap(setting = AppSettings::ColoredHelp)]
        #[clap(setting = AppSettings::DisableVersion)]
        PackExplode {
            #[clap(long)]
            /// Read written objects back and assert they match their source. Fail the operation otherwise.
            ///
            /// Only relevant if an object directory is set.
            verify: bool,

            /// delete the pack and index file after the operation is successful
            #[clap(long)]
            delete_pack: bool,

            /// The amount of checks to run
            #[clap(
                long,
                short = "c",
                default_value = "all",
                possible_values(core::pack::explode::SafetyCheck::variants())
            )]
            check: core::pack::explode::SafetyCheck,

            /// Compress bytes even when using the sink, i.e. no object directory is specified
            ///
            /// This helps to determine overhead related to compression. If unset, the sink will
            /// only create hashes from bytes, which is usually limited by the speed at which input
            /// can be obtained.
            #[clap(long)]
            sink_compress: bool,

            /// The '.pack' or '.idx' file to explode into loose objects
            #[clap(parse(from_os_str))]
            pack_path: PathBuf,

            /// The path into which all objects should be written. Commonly '.git/objects'
            #[clap(parse(from_os_str))]
            object_path: Option<PathBuf>,
        },
        /// Verify the integrity of a pack or index file
        #[clap(setting = AppSettings::ColoredHelp)]
        #[clap(setting = AppSettings::DisableVersion)]
        PackVerify {
            /// output statistical information about the pack
            #[clap(long, short = "s")]
            statistics: bool,
            /// The algorithm used to verify the pack. They differ in costs.
            #[clap(
                long,
                short = "a",
                default_value = "less-time",
                possible_values(core::pack::verify::Algorithm::variants())
            )]
            algorithm: core::pack::verify::Algorithm,

            #[clap(long, conflicts_with("re-encode"))]
            /// Decode and parse tags, commits and trees to validate their correctness beyond hashing correctly.
            ///
            /// Malformed objects should not usually occur, but could be injected on purpose or accident.
            /// This will reduce overall performance.
            decode: bool,

            #[clap(long)]
            /// Decode and parse tags, commits and trees to validate their correctness, and re-encode them.
            ///
            /// This flag is primarily to test the implementation of encoding, and requires to decode the object first.
            /// Encoding an object after decoding it should yield exactly the same bytes.
            /// This will reduce overall performance even more, as re-encoding requires to transform zero-copy objects into
            /// owned objects, causing plenty of allocation to occour.
            re_encode: bool,

            /// The '.pack' or '.idx' file whose checksum to validate.
            #[clap(parse(from_os_str))]
            path: PathBuf,
        },
    }
}

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
    super::init_env_logger(false);
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
