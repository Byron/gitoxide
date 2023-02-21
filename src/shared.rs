#[cfg(any(feature = "prodash-render-line", feature = "prodash-render-tui"))]
pub const DEFAULT_FRAME_RATE: f32 = 6.0;

#[allow(unused)]
pub type ProgressRange = std::ops::RangeInclusive<prodash::progress::key::Level>;
#[allow(unused)]
pub const STANDARD_RANGE: ProgressRange = 2..=2;

/// If verbose is true, the env logger will be forcibly set to 'info' logging level. Otherwise env logging facilities
/// will just be initialized.
#[allow(unused)] // Squelch warning because it's used in porcelain as well and we can't know that at compile time
pub fn init_env_logger() {
    if cfg!(feature = "small") {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .format_module_path(false)
            .init();
    } else {
        env_logger::init();
    }
}

#[cfg(feature = "prodash-render-line")]
pub fn progress_tree() -> std::sync::Arc<prodash::tree::Root> {
    prodash::tree::root::Options {
        message_buffer_capacity: 200,
        ..Default::default()
    }
    .into()
}

#[cfg(not(feature = "prodash-render-line"))]
pub struct LogCreator;

#[cfg(not(feature = "prodash-render-line"))]
impl LogCreator {
    pub fn add_child(&self, name: &str) -> prodash::progress::Log {
        prodash::progress::Log::new(name, Some(1))
    }
}

#[cfg(not(any(feature = "prodash-render-tui", feature = "prodash-render-line")))]
fn progress_tree() -> LogCreator {
    LogCreator
}

#[cfg(feature = "pretty-cli")]
pub mod pretty {
    use std::io::{stderr, stdout};

    use anyhow::Result;
    use gix_features::progress;

    use crate::shared::ProgressRange;

    #[cfg(feature = "small")]
    pub fn prepare_and_run<T>(
        name: &str,
        verbose: bool,
        progress: bool,
        #[cfg_attr(not(feature = "prodash-render-tui"), allow(unused_variables))] progress_keep_open: bool,
        range: impl Into<Option<ProgressRange>>,
        run: impl FnOnce(
            progress::DoOrDiscard<prodash::tree::Item>,
            &mut dyn std::io::Write,
            &mut dyn std::io::Write,
        ) -> Result<T>,
    ) -> Result<T> {
        crate::shared::init_env_logger();

        match (verbose, progress) {
            (false, false) => {
                let stdout = stdout();
                let mut stdout_lock = stdout.lock();
                let stderr = stderr();
                let mut stderr_lock = stderr.lock();
                run(progress::DoOrDiscard::from(None), &mut stdout_lock, &mut stderr_lock)
            }
            (true, false) => {
                let progress = crate::shared::progress_tree();
                let sub_progress = progress.add_child(name);

                use crate::shared::{self, STANDARD_RANGE};
                let handle = shared::setup_line_renderer_range(&progress, range.into().unwrap_or(STANDARD_RANGE));

                let mut out = Vec::<u8>::new();
                let res = run(progress::DoOrDiscard::from(Some(sub_progress)), &mut out, &mut stderr());
                handle.shutdown_and_wait();
                std::io::Write::write_all(&mut stdout(), &out)?;
                res
            }
            #[cfg(not(feature = "prodash-render-tui"))]
            (true, true) | (false, true) => {
                unreachable!("BUG: This branch can't be run without a TUI built-in")
            }
        }
    }

    #[cfg(not(feature = "small"))]
    pub fn prepare_and_run<T: Send + 'static>(
        name: &str,
        verbose: bool,
        progress: bool,
        #[cfg_attr(not(feature = "prodash-render-tui"), allow(unused_variables))] progress_keep_open: bool,
        range: impl Into<Option<ProgressRange>>,
        run: impl FnOnce(
                progress::DoOrDiscard<prodash::tree::Item>,
                &mut dyn std::io::Write,
                &mut dyn std::io::Write,
            ) -> Result<T>
            + Send
            + 'static,
    ) -> Result<T> {
        crate::shared::init_env_logger();

        match (verbose, progress) {
            (false, false) => run(progress::DoOrDiscard::from(None), &mut stdout(), &mut stderr()),
            (true, false) => {
                use crate::shared::{self, STANDARD_RANGE};
                let progress = shared::progress_tree();
                let sub_progress = progress.add_child(name);

                let handle = shared::setup_line_renderer_range(&progress, range.into().unwrap_or(STANDARD_RANGE));

                let mut out = Vec::<u8>::new();
                let mut err = Vec::<u8>::new();
                let res = run(progress::DoOrDiscard::from(Some(sub_progress)), &mut out, &mut err);
                handle.shutdown_and_wait();
                std::io::Write::write_all(&mut stdout(), &out)?;
                std::io::Write::write_all(&mut stderr(), &err)?;
                res
            }
            #[cfg(not(feature = "prodash-render-tui"))]
            (true, true) | (false, true) => {
                unreachable!("BUG: This branch can't be run without a TUI built-in")
            }
            #[cfg(feature = "prodash-render-tui")]
            (true, true) | (false, true) => {
                use std::io::Write;

                use crate::shared;

                enum Event<T> {
                    UiDone,
                    ComputationDone(Result<T>, Vec<u8>),
                }
                let progress = prodash::tree::Root::new();
                let sub_progress = progress.add_child(name);
                let render_tui = prodash::render::tui(
                    stdout(),
                    std::sync::Arc::downgrade(&progress),
                    prodash::render::tui::Options {
                        title: "gitoxide".into(),
                        frames_per_second: shared::DEFAULT_FRAME_RATE,
                        stop_if_progress_missing: !progress_keep_open,
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
                        tx.send(Event::UiDone).ok();
                    }
                });
                let thread = std::thread::spawn(move || {
                    // We might have something interesting to show, which would be hidden by the alternate screen if there is a progress TUI
                    // We know that the printing happens at the end, so this is fine.
                    let mut out = Vec::new();
                    let res = run(progress::DoOrDiscard::from(Some(sub_progress)), &mut out, &mut stderr());
                    tx.send(Event::ComputationDone(res, out)).ok();
                });
                loop {
                    match rx.recv() {
                        Ok(Event::UiDone) => {
                            // We don't know why the UI is done, usually it's the user aborting.
                            // We need the computation to stop as well so let's wait for that to happen
                            gix::interrupt::trigger();
                            continue;
                        }
                        Ok(Event::ComputationDone(res, out)) => {
                            ui_handle.join().ok();
                            stdout().write_all(&out)?;
                            break res;
                        }
                        Err(_err) => match thread.join() {
                            Ok(()) => unreachable!("BUG: We shouldn't fail to receive unless the thread has panicked"),
                            Err(panic) => std::panic::resume_unwind(panic),
                        },
                    }
                }
            }
        }
    }
}

#[allow(unused)]
#[cfg(feature = "prodash-render-line")]
pub fn setup_line_renderer_range(
    progress: &std::sync::Arc<prodash::tree::Root>,
    levels: std::ops::RangeInclusive<prodash::progress::key::Level>,
) -> prodash::render::line::JoinHandle {
    prodash::render::line(
        std::io::stderr(),
        std::sync::Arc::downgrade(progress),
        prodash::render::line::Options {
            level_filter: Some(levels),
            frames_per_second: DEFAULT_FRAME_RATE,
            initial_delay: Some(std::time::Duration::from_millis(1000)),
            timestamp: true,
            throughput: true,
            hide_cursor: true,
            ..prodash::render::line::Options::default()
        }
        .auto_configure(prodash::render::line::StreamKind::Stderr),
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
        if let Some(arg) = std::env::args().nth(1) {
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

mod clap {
    use std::{ffi::OsStr, str::FromStr};

    use clap::{builder, builder::PossibleValue, error::ErrorKind, Arg, Command, Error};
    use gitoxide_core as core;
    use gix::bstr::BString;

    #[derive(Clone)]
    pub struct AsBString;

    impl builder::TypedValueParser for AsBString {
        type Value = BString;

        fn parse_ref(&self, _cmd: &Command, _arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
            gix::env::os_str_to_bstring(value).ok_or_else(|| Error::new(ErrorKind::InvalidUtf8))
        }
    }

    #[derive(Clone)]
    pub struct AsOutputFormat;

    impl builder::TypedValueParser for AsOutputFormat {
        type Value = core::OutputFormat;

        fn parse_ref(&self, cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
            builder::StringValueParser::new()
                .try_map(|arg| core::OutputFormat::from_str(&arg))
                .parse_ref(cmd, arg, value)
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
            Some(Box::new(core::OutputFormat::variants().iter().map(PossibleValue::new)))
        }
    }

    #[derive(Clone)]
    pub struct AsHashKind;

    impl builder::TypedValueParser for AsHashKind {
        type Value = gix::hash::Kind;

        fn parse_ref(&self, cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
            builder::StringValueParser::new()
                .try_map(|arg| gix::hash::Kind::from_str(&arg))
                .parse_ref(cmd, arg, value)
        }

        fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
            Some(Box::new([PossibleValue::new("SHA1")].into_iter()))
        }
    }

    use clap::builder::{OsStringValueParser, TypedValueParser};

    #[derive(Clone)]
    pub struct AsPathSpec;

    impl TypedValueParser for AsPathSpec {
        type Value = gix::path::Spec;

        fn parse_ref(&self, cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
            OsStringValueParser::new()
                .try_map(|arg| gix::path::Spec::try_from(arg.as_os_str()))
                .parse_ref(cmd, arg, value)
        }
    }
}
pub use self::clap::{AsBString, AsHashKind, AsOutputFormat, AsPathSpec};
