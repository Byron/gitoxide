mod options;
use options::{Args, ChangeLog, SmartRelease, SubCommands};

use cargo_smart_release::command;

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();
    init_logging();

    match args.subcommands {
        SubCommands::Changelog(ChangeLog {
            write,
            crates,
            no_dependencies,
            no_preview,
            without,
            allow_dirty,
        }) => command::changelog(
            command::changelog::Options {
                dry_run: !write,
                allow_dirty,
                preview: !no_preview,
                dependencies: !no_dependencies,
                generator_segments: names_to_segment_selection(&without)?,
            },
            crates,
        )?,
        SubCommands::SmartRelease(SmartRelease {
            execute,
            verbose,
            bump,
            bump_dependencies,
            crates,
            allow_dirty,
            ignore_instability,
            no_publish,
            no_tag,
            no_push,
            changelog_without,
            dangerously_pass_no_verify,
            no_auto_publish_of_stable_crates,
            no_conservative_pre_release_version_handling,
            dry_run_cargo_publish,
            update_crates_index,
            no_bump_on_demand,
            no_changelog,
            no_changelog_preview,
            no_dependencies,
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
                skip_publish: no_publish,
                changelog: !no_changelog,
                skip_tag: no_tag,
                skip_push: no_push,
                skip_dependencies: no_dependencies,
                dry_run_cargo_publish,
                no_verify: dangerously_pass_no_verify,
                allow_auto_publish_of_stable_crates: !no_auto_publish_of_stable_crates,
                update_crates_index,
                preview: !no_changelog_preview,
                generator_segments: names_to_segment_selection(&changelog_without)?,
            },
            crates,
            bump.unwrap_or_else(|| "keep".into()),
            bump_dependencies.unwrap_or_else(|| "keep".into()),
        )?,
    };

    Ok(())
}

fn names_to_segment_selection(
    names: &[String],
) -> anyhow::Result<cargo_smart_release::changelog::section::segment::Selection> {
    use cargo_smart_release::changelog::section::segment::Selection;
    Ok(if names.is_empty() {
        Selection::all()
    } else {
        let mut deselected = Selection::empty();
        for name in names {
            deselected |= match name.as_str() {
                "clippy" => Selection::CLIPPY,
                "commit-details" => Selection::COMMIT_DETAILS,
                "commit-statistics" => Selection::COMMIT_STATISTICS,
                other => anyhow::bail!("Invalid changelog segment selector: {:?}", other),
            };
        }
        Selection::all().difference(deselected)
    })
}

fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .init();
}
