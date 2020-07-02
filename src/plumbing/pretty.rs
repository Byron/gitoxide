use anyhow::Result;
use git_features::progress;
use gitoxide_core as core;
use std::io::{stderr, stdout, Write};
use structopt::StructOpt;

use options::*;

mod options {
    use std::path::PathBuf;
    use structopt::{clap::AppSettings, StructOpt};

    #[derive(Debug, StructOpt)]
    #[structopt(about = "The git, simply swift")]
    #[structopt(settings = &[AppSettings::SubcommandRequired,
                        AppSettings::ColoredHelp])]
    pub struct Args {
        #[structopt(subcommand)]
        pub cmd: Subcommands,
    }

    #[derive(Debug)]
    pub enum ProgressMode {
        Stop,
        KeepRunning,
    }

    impl ProgressMode {
        fn variants() -> &'static [&'static str] {
            &["stop", "keep-running"]
        }
    }
    impl std::str::FromStr for ProgressMode {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s_lc = s.to_ascii_lowercase();
            Ok(match s_lc.as_str() {
                "stop" => ProgressMode::Stop,
                "keep-running" => ProgressMode::KeepRunning,
                _ => return Err(format!("Invalid progress mode: {}", s)),
            })
        }
    }

    #[derive(Debug, StructOpt)]
    pub enum Subcommands {
        /// Verify the integrity of a pack or index file
        #[structopt(setting = AppSettings::ColoredHelp)]
        VerifyPack {
            /// if set, output statistical information about the pack
            #[structopt(long, short = "s")]
            statistics: bool,

            /// if set, verbose progress messages are printed line by line
            #[structopt(long, short = "v")]
            verbose: bool,

            /// if set, bring up a terminal user interface displaying progress visually
            #[structopt(long, conflicts_with("verbose"), possible_values(ProgressMode::variants()))]
            progress: Option<ProgressMode>,

            /// The '.pack' or '.idx' file whose checksum to validate.
            #[structopt(parse(from_os_str))]
            path: PathBuf,
        },
    }
}

fn init_progress(
    name: &str,
    verbose: bool,
    progress: Option<ProgressMode>,
) -> (
    Option<JoinThreadOnDrop>,
    Option<progress::Either<progress::Log, prodash::tree::Item>>,
) {
    super::init_env_logger(verbose);
    match (verbose, progress) {
        (false, None) => (None, None),
        (true, None) => (None, Some(progress::Either::Left(progress::Log::new(name)))),
        (true, Some(mode)) | (false, Some(mode)) => {
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let render_tui = prodash::tui::render(
                progress,
                prodash::tui::TuiOptions {
                    title: "gitoxide".into(),
                    frames_per_second: 6.0,
                    stop_if_empty_progress: match mode {
                        ProgressMode::KeepRunning => false,
                        ProgressMode::Stop => true,
                    },
                    ..Default::default()
                },
            )
            .expect("tui to come up without io error");
            let handle = std::thread::spawn(move || smol::run(render_tui));

            (
                Some(JoinThreadOnDrop(Some(handle))),
                Some(progress::Either::Right(sub_progress)),
            )
        }
    }
}

struct JoinThreadOnDrop(Option<std::thread::JoinHandle<()>>);
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
            statistics,
        } => {
            let (handle, progress) = init_progress("verify-pack", verbose, progress);
            let mut buf = Vec::new();
            let res = core::verify_pack_or_pack_index(path, progress, statistics, &mut buf, stderr()).map(|_| ());
            // We might have something interesting to show, which would be hidden by the alternate screen if there is a progress TUI
            // We know that the printing happens at the end, so this is fine.
            drop(handle);
            stdout().write_all(&buf)?;
            res
        }
    }?;
    Ok(())
}
