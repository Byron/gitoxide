mod options;
use options::{Args, ChangeLog, SmartRelease, SubCommands};

mod command;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    init_logging();

    match args.subcommands {
        SubCommands::Changelog(ChangeLog { write, crates }) => command::release(
            command::release::Options {
                dry_run: !write,
                verbose: false,
                conservative_pre_release_version_handling: true,
                bump_when_needed: true,
                multi_crate_release: true,
                isolate_dependencies_from_breaking_changes: true,
                allow_dirty: false,
                ignore_instability: false,
                skip_publish: true,
                skip_tag: true,
                skip_push: true,
                skip_dependencies: true,
                dry_run_cargo_publish: false,
                no_verify: false,
                allow_auto_publish_of_stable_crates: true,
                update_crates_index: false,
            },
            crates,
            "keep".into(),
            "keep".into(),
        )?,
        SubCommands::SmartRelease(SmartRelease {
            execute,
            verbose,
            bump,
            bump_dependencies,
            crates,
            allow_dirty,
            ignore_instability,
            skip_publish,
            skip_tag,
            skip_push,
            dangerously_pass_no_verify,
            no_auto_publish_of_stable_crates,
            no_conservative_pre_release_version_handling,
            dry_run_cargo_publish,
            update_crates_index,
            no_bump_on_demand,
            skip_dependencies,
            no_multi_crate_release,
            no_isolate_dependencies_from_breaking_changes,
        }) => command::release(
            command::release::Options {
                dry_run: !execute,
                verbose: execute || verbose,
                conservative_pre_release_version_handling: !no_conservative_pre_release_version_handling,
                bump_when_needed: !no_bump_on_demand,
                multi_crate_release: !no_multi_crate_release,
                isolate_dependencies_from_breaking_changes: !no_isolate_dependencies_from_breaking_changes,
                allow_dirty,
                ignore_instability,
                skip_publish,
                skip_tag,
                skip_push,
                skip_dependencies,
                dry_run_cargo_publish,
                no_verify: dangerously_pass_no_verify,
                allow_auto_publish_of_stable_crates: !no_auto_publish_of_stable_crates,
                update_crates_index,
            },
            crates,
            bump.unwrap_or_else(|| "keep".into()),
            bump_dependencies.unwrap_or_else(|| "keep".into()),
        )?,
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
