#[cfg(any(feature = "prodash-render-line", feature = "prodash-render-tui"))]
pub const DEFAULT_FRAME_RATE: f32 = 6.0;

#[allow(unused)]
pub type ProgressRange = std::ops::RangeInclusive<prodash::progress::key::Level>;
#[allow(unused)]
pub const STANDARD_RANGE: ProgressRange = 2..=2;

/// If verbose is true, the env logger will be forcibly set to 'info' logging level. Otherwise env logging facilities
/// will just be initialized.
#[cfg(feature = "env_logger")]
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
