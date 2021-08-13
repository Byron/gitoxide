mod options;
use options::{Args, SmartRelease, SubCommands};

mod command;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    init_logging();

    match args.subcommands {
        SubCommands::SmartRelease(SmartRelease {
            execute,
            bump,
            bump_dependencies,
            crates,
            allow_dirty,
            ignore_instability,
            skip_publish,
            skip_tag,
            dangerously_pass_no_verify,
            allow_auto_publish_of_stable_crates,
            no_dry_run_cargo_publish,
            update_crates_index,
        }) => {
            let bump = bump.unwrap_or_else(|| "keep".into());
            command::release(
                command::release::Options {
                    dry_run: !execute,
                    allow_dirty,
                    ignore_instability,
                    skip_publish,
                    skip_tag,
                    no_dry_run_cargo_publish,
                    no_verify: dangerously_pass_no_verify,
                    allow_auto_publish_of_stable_crates,
                    update_crates_index,
                },
                crates,
                bump.clone(),
                bump_dependencies.unwrap_or(bump),
            )?
        }
    };

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
