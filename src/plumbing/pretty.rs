use anyhow::Result;
use gitoxide_core as core;
use std::io::{stderr, stdout, Write};
use structopt::StructOpt;

use options::*;

mod options {
    use gitoxide_core as core;
    use std::path::PathBuf;
    use structopt::{clap::AppSettings, StructOpt};

    #[derive(Debug, StructOpt)]
    #[structopt(name = "gio-plumbing", about = "The git underworld")]
    #[structopt(settings = &[AppSettings::SubcommandRequired,
                        AppSettings::ColoredHelp])]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Verify the integrity of a pack or index file
        #[structopt(setting = AppSettings::ColoredHelp)]
        VerifyPack {
            /// output statistical information about the pack
            #[structopt(long, short = "s")]
            statistics: bool,
            /// Determine the format to use when outputting statistics.
            #[structopt(
                long,
                short = "f",
                default_value = "human",
                possible_values(core::OutputFormat::variants())
            )]
            format: core::OutputFormat,

            /// verbose progress messages are printed line by line
            #[structopt(long, short = "v")]
            verbose: bool,

            /// bring up a terminal user interface displaying progress visually
            #[structopt(long, conflicts_with("verbose"))]
            progress: bool,

            /// the progress TUI will stay up even though the work is already completed.
            ///
            /// Use this to be able to read progress messages or additional information visible in the TUI log pane.
            #[structopt(long, conflicts_with("verbose"), requires("progress"))]
            progress_keep_open: bool,

            /// The '.pack' or '.idx' file whose checksum to validate.
            #[structopt(parse(from_os_str))]
            path: PathBuf,
        },
    }
}

fn prepare_and_run<T>(
    name: &str,
    verbose: bool,
    progress: bool,
    progress_keep_open: bool,
    run: impl FnOnce(Option<prodash::tree::Item>, &mut dyn std::io::Write, &mut dyn std::io::Write) -> Result<T>,
) -> Result<T> {
    super::init_env_logger(false);
    match (verbose, progress) {
        (false, false) => run(None, &mut stdout(), &mut stderr()),
        (true, false) => {
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let _handle = crate::shared::setup_line_renderer(progress, 2);
            run(Some(sub_progress), &mut stdout(), &mut stderr())
        }
        (true, true) | (false, true) => {
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let render_tui = prodash::tui::render(
                stdout(),
                progress,
                prodash::tui::Options {
                    title: "gitoxide".into(),
                    frames_per_second: crate::shared::DEFAULT_FRAME_RATE,
                    stop_if_empty_progress: !progress_keep_open,
                    ..Default::default()
                },
            )
            .expect("tui to come up without io error");
            let _handle = JoinThreadOnDrop(Some(std::thread::spawn(move || smol::run(render_tui))), None);
            let mut out = Vec::new();
            let mut err = Vec::new();
            let res = run(Some(sub_progress), &mut out, &mut err);
            // We might have something interesting to show, which would be hidden by the alternate screen if there is a progress TUI
            // We know that the printing happens at the end, so this is fine.
            drop(_handle);
            stdout().write_all(&out)?;
            stderr().write_all(&err)?;
            res
        }
    }
}

struct JoinThreadOnDrop(Option<std::thread::JoinHandle<()>>, Option<prodash::line::JoinHandle>);
impl Drop for JoinThreadOnDrop {
    fn drop(&mut self) {
        self.0.take().and_then(|handle| handle.join().ok());
    }
}

pub fn main() -> Result<()> {
    let args = Args::from_args();
    match args.cmd {
        Subcommands::VerifyPack {
            path,
            verbose,
            progress,
            format,
            progress_keep_open,
            statistics,
        } => prepare_and_run(
            "verify-pack",
            verbose,
            progress,
            progress_keep_open,
            |progress, out, err| {
                core::verify_pack_or_pack_index(path, progress, if statistics { Some(format) } else { None }, out, err)
            },
        )
        .map(|_| ()),
    }?;
    Ok(())
}
