mod options;
use options::{Args, Release, SubCommands};

mod command;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    init_logging();

    match args.subcommand {
        SubCommands::Release(Release {
            execute,
            version_bump_spec,
            crates,
        }) => command::release(!execute, version_bump_spec, crates)?,
    }
    Ok(())
}

fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_module_path(false)
        .init();
}
