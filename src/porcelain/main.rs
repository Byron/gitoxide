use crate::porcelain::options::{Args, Subcommands, ToolCommands};
use crate::shared::pretty::prepare_and_run;
use anyhow::Result;
use clap::Clap;
use git_features::progress::DoOrDiscard;
use gitoxide_core as core;

pub fn main() -> Result<()> {
    let args = Args::parse();
    git_features::interrupt::init_handler(std::io::stderr());
    let verbose = true;

    match args.cmd {
        Subcommands::Init { directory } => core::repository::init(directory),
        Subcommands::Tools(tool) => match tool {
            ToolCommands::Find { root } => {
                use gitoxide_core::organize;
                // force verbose only, being the line renderer.
                let progress = false;
                let progress_keep_open = false;
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
                // force verbose only, being the line renderer.
                let progress = false;
                let progress_keep_open = false;

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
