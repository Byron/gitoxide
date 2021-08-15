use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use clap::Clap;
use git_features::progress::DoOrDiscard;
use gitoxide_core as core;

use crate::{
    porcelain::options::{Args, EstimateHours, Subcommands, ToolCommands},
    shared::pretty::prepare_and_run,
};

pub fn main() -> Result<()> {
    let args: Args = Args::parse();
    let should_interrupt = Arc::new(AtomicBool::new(false));
    git_repository::interrupt::init_handler({
        let should_interrupt = Arc::clone(&should_interrupt);
        move || should_interrupt.store(true, Ordering::SeqCst)
    })?;
    let verbose = !args.quiet;
    let progress = args.progress;
    let progress_keep_open = args.progress_keep_open;

    match args.cmd {
        #[cfg(debug_assertions)]
        Subcommands::Panic => prepare_and_run(
            "panic-behaviour",
            verbose,
            progress,
            progress_keep_open,
            crate::shared::STANDARD_RANGE,
            move |_progress, _out, _err| panic!("something went very wrong"),
        ),
        Subcommands::Init { directory } => core::repository::init(directory),
        Subcommands::Tools(tool) => match tool {
            ToolCommands::EstimateHours(EstimateHours {
                working_dir,
                refname,
                show_pii,
                omit_unify_identities,
            }) => {
                use gitoxide_core::hours;
                prepare_and_run(
                    "find",
                    verbose,
                    progress,
                    progress_keep_open,
                    crate::shared::STANDARD_RANGE,
                    move |progress, out, _err| {
                        hours::estimate(
                            &working_dir,
                            &refname,
                            DoOrDiscard::from(progress),
                            hours::Context {
                                show_pii,
                                omit_unify_identities,
                                out,
                            },
                        )
                    },
                )
            }
            ToolCommands::Find { root } => {
                use gitoxide_core::organize;
                prepare_and_run(
                    "find",
                    verbose,
                    progress,
                    progress_keep_open,
                    crate::shared::STANDARD_RANGE,
                    move |progress, out, _err| {
                        organize::discover(
                            root.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                            out,
                            DoOrDiscard::from(progress),
                        )
                    },
                )
            }
            ToolCommands::Organize {
                destination_directory,
                execute,
                repository_source,
            } => {
                use gitoxide_core::organize;
                prepare_and_run(
                    "organize",
                    verbose,
                    progress,
                    progress_keep_open,
                    crate::shared::STANDARD_RANGE,
                    move |progress, _out, _err| {
                        organize::run(
                            if execute {
                                organize::Mode::Execute
                            } else {
                                organize::Mode::Simulate
                            },
                            repository_source.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                            destination_directory.unwrap_or_else(|| [std::path::Component::CurDir].iter().collect()),
                            DoOrDiscard::from(progress),
                        )
                    },
                )
            }
        },
    }?;
    Ok(())
}
