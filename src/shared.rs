use std::{
    io::{stderr, stdout, Write},
    panic::UnwindSafe,
};

use anyhow::{anyhow, Result};

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

#[cfg(any(feature = "prodash-render-line-crossterm", feature = "prodash-render-line-termion"))]
fn progress_tree() -> prodash::Tree {
    prodash::TreeOptions {
        message_buffer_capacity: 200,
        ..Default::default()
    }
    .into()
}

pub fn prepare_and_run<T: Send + 'static>(
    name: &str,
    verbose: bool,
    progress: bool,
    progress_keep_open: bool,
    range: impl Into<Option<ProgressRange>>,
    run: impl FnOnce(Option<prodash::tree::Item>, &mut dyn std::io::Write, &mut dyn std::io::Write) -> Result<T>
        + Send
        + UnwindSafe
        + 'static,
) -> Result<T> {
    init_env_logger(false);
    use git_repository::interrupt;

    match (verbose, progress) {
        (false, false) => run(None, &mut stdout(), &mut stderr()),
        (true, false) => {
            enum Event<T> {
                UiDone,
                ComputationFailed,
                ComputationDone(Result<T>),
            }
            let progress = progress_tree();
            let sub_progress = progress.add_child(name);
            let (tx, rx) = std::sync::mpsc::sync_channel::<Event<T>>(1);
            let ui_handle = setup_line_renderer_range(progress, range.into().unwrap_or(STANDARD_RANGE));
            std::thread::spawn({
                let tx = tx.clone();
                move || loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if interrupt::is_triggered() {
                        tx.send(Event::UiDone).ok();
                        break;
                    }
                }
            });
            // LIMITATION: This will hang if the thread panics as no message is send and the renderer thread will wait forever.
            // `catch_unwind` can't be used as a parking lot mutex is not unwind safe, coming from prodash.
            let join_handle = std::thread::spawn(move || {
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    run(Some(sub_progress), &mut stdout(), &mut stderr())
                }));
                match res {
                    Ok(res) => tx.send(Event::ComputationDone(res)).ok(),
                    Err(err) => {
                        tx.send(Event::ComputationFailed).ok();
                        std::panic::resume_unwind(err)
                    }
                }
            });
            match rx.recv()? {
                Event::UiDone => {
                    ui_handle.shutdown_and_wait();
                    drop(join_handle);
                    Err(anyhow!("Operation cancelled by user"))
                }
                Event::ComputationDone(res) => {
                    ui_handle.shutdown_and_wait();
                    join_handle.join().ok();
                    res
                }
                Event::ComputationFailed => match join_handle.join() {
                    Ok(_) => unreachable!("The thread has panicked and unwrap is expected to show it"),
                    Err(err) => std::panic::resume_unwind(err),
                },
            }
        }
        (true, true) | (false, true) => {
            enum Event<T> {
                UiDone,
                ComputationDone(Result<T>, Vec<u8>),
            }
            let progress = prodash::Tree::new();
            let sub_progress = progress.add_child(name);
            let render_tui = prodash::render::tui(
                stdout(),
                progress,
                prodash::render::tui::Options {
                    title: "gitoxide".into(),
                    frames_per_second: DEFAULT_FRAME_RATE,
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
                    tx.send(Event::UiDone).ok();
                }
            });
            std::thread::spawn(move || {
                // We might have something interesting to show, which would be hidden by the alternate screen if there is a progress TUI
                // We know that the printing happens at the end, so this is fine.
                let mut out = Vec::new();
                let res = run(Some(sub_progress), &mut out, &mut stderr());
                tx.send(Event::ComputationDone(res, out)).ok();
            });
            loop {
                match rx.recv()? {
                    Event::UiDone => {
                        // We don't know why the UI is done, usually it's the user aborting.
                        // We need the computation to stop as well so let's wait for that to happen
                        interrupt::trigger();
                        continue;
                    }
                    Event::ComputationDone(res, out) => {
                        ui_handle.join().ok();
                        stdout().write_all(&out)?;
                        break res;
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
) -> prodash::render::line::JoinHandle {
    prodash::render::line(
        std::io::stderr(),
        progress,
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
