#[cfg(any(feature = "prodash-render-line", feature = "prodash-render-tui"))]
pub const DEFAULT_FRAME_RATE: f32 = 6.0;

#[allow(unused)]
pub type ProgressRange = std::ops::RangeInclusive<prodash::progress::key::Level>;
#[allow(unused)]
pub const STANDARD_RANGE: ProgressRange = 2..=2;

/// If verbose is true, the env logger will be forcibly set to 'info' logging level. Otherwise env logging facilities
/// will just be initialized.
#[cfg(feature = "env_logger")]
#[allow(unused)] // Squelch warning because it's used in porcelain as well and we can't know that at compile time
pub fn init_env_logger(verbose: bool) {
    if verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .format_module_path(false)
            .init();
    } else {
        env_logger::init();
    }
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
pub mod lean {
    use crate::shared::ProgressRange;

    #[cfg(not(any(feature = "prodash-render-line-crossterm", feature = "prodash-render-line-termion")))]
    pub fn prepare(
        verbose: bool,
        name: &str,
        _: impl Into<Option<ProgressRange>>,
    ) -> ((), Option<prodash::progress::Log>) {
        super::init_env_logger(verbose);
        ((), Some(prodash::progress::Log::new(name, Some(1))))
    }

    #[cfg(any(feature = "prodash-render-line-crossterm", feature = "prodash-render-line-termion"))]
    pub fn prepare(
        verbose: bool,
        name: &str,
        range: impl Into<Option<ProgressRange>>,
    ) -> (Option<prodash::render::line::JoinHandle>, Option<prodash::tree::Item>) {
        use crate::shared::{self, STANDARD_RANGE};
        super::init_env_logger(false);

        if verbose {
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let ui_handle = shared::setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE), true);
            (Some(ui_handle), Some(sub_progress))
        } else {
            (None, None)
        }
    }
}

#[cfg(feature = "pretty-cli")]
pub mod pretty {
    use crate::shared::ProgressRange;
    use anyhow::{anyhow, Result};
    use std::io::{stderr, stdout, Write};

    pub fn prepare_and_run<T: Send + 'static>(
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
        crate::shared::init_env_logger(false);
        use git_features::interrupt;

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
                let ui_handle =
                    shared::setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE), true);
                std::thread::spawn({
                    let tx = tx.clone();
                    move || loop {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        if interrupt::is_triggered() {
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
                        frames_per_second: shared::DEFAULT_FRAME_RATE,
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
                            interrupt::trigger();
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
}

#[allow(unused)]
#[cfg(feature = "prodash-render-line")]
pub fn setup_line_renderer_range(
    progress: prodash::Tree,
    levels: std::ops::RangeInclusive<prodash::progress::key::Level>,
    hide_cursor: bool,
) -> prodash::render::line::JoinHandle {
    let output_is_terminal = atty::is(atty::Stream::Stderr);
    prodash::render::line(
        std::io::stderr(),
        progress,
        prodash::render::line::Options {
            level_filter: Some(levels),
            frames_per_second: DEFAULT_FRAME_RATE,
            initial_delay: Some(std::time::Duration::from_millis(1000)),
            output_is_terminal,
            colored: output_is_terminal && crosstermion::color::allowed(),
            terminal_dimensions: crosstermion::terminal::size().unwrap_or((80, 20)),
            timestamp: true,
            hide_cursor,
            throughput: true,
            ..prodash::render::line::Options::default()
        },
    )
}

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
pub fn from_env<T: argh::TopLevelCommand>() -> T {
    static VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
    let strings: Vec<String> = std::env::args().collect();
    let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    T::from_args(&[strs[0]], &strs[1..]).unwrap_or_else(|early_exit| {
        // This allows us to make subcommands mandatory,
        // and trigger a helpful message unless --version is specified
        if let Some(arg) = std::env::args().skip(1).next() {
            if arg == "--version" {
                println!("{}", VERSION);
                std::process::exit(0);
            }
        }
        println!("{}", early_exit.output);
        std::process::exit(match early_exit.status {
            Ok(()) => 0,
            Err(()) => 1,
        })
    })
}
