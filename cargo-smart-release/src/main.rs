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
            allow_dirty,
            ignore_instability,
            skip_publish,
            dangerously_pass_no_verify,
        }) => command::release(
            command::release::Options {
                dry_run: !execute,
                allow_dirty,
                ignore_instability,
                skip_publish,
                no_verify: dangerously_pass_no_verify,
            },
            version_bump_spec,
            crates,
        )?,
    }
    Ok(())
}

fn init_logging() {
    env_logger::Builder::new()
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();
}
