mod options;
use clap::Parser;
use options::{Args, SubCommands};

use cargo_smart_release::command;

fn main() -> anyhow::Result<()> {
    gix::interrupt::init_handler(|| {})?;
    unsafe {
        // SAFETY: we don't manipulate the environment from any thread
        time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Unsound);
    }
    let args: Args = Args::parse();
    match args.subcommands {
        SubCommands::Changelog {
            write,
            execute,
            crates,
            no_dependencies,
            no_preview,
            no_links,
            without,
            allow_dirty,
        } => {
            init_logging(false);
            command::changelog(
                command::changelog::Options {
                    dry_run: !(write || execute),
                    allow_dirty,
                    no_links,
                    preview: !no_preview,
                    dependencies: !no_dependencies,
                    generator_segments: names_to_segment_selection(&without)?,
                },
                crates,
            )?
        }
        SubCommands::SmartRelease {
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
            no_changelog_links,
            no_changelog_preview,
            no_changelog_github_release,
            allow_fully_generated_changelogs,
            no_dependencies,
            no_isolate_dependencies_from_breaking_changes,
        } => {
            let verbose = execute || verbose;
            init_logging(verbose);
            command::release(
                command::release::Options {
                    dry_run: !execute,
                    verbose,
                    conservative_pre_release_version_handling: !no_conservative_pre_release_version_handling,
                    bump_when_needed: !no_bump_on_demand,
                    isolate_dependencies_from_breaking_changes: !no_isolate_dependencies_from_breaking_changes,
                    allow_dirty,
                    ignore_instability,
                    skip_publish: no_publish,
                    changelog: !no_changelog,
                    skip_tag: no_tag,
                    skip_push: no_push,
                    dependencies: !no_dependencies,
                    dry_run_cargo_publish,
                    no_verify: dangerously_pass_no_verify,
                    allow_auto_publish_of_stable_crates: !no_auto_publish_of_stable_crates,
                    update_crates_index,
                    preview: !no_changelog_preview,
                    generator_segments: names_to_segment_selection(&changelog_without)?,
                    allow_fully_generated_changelogs,
                    changelog_links: !no_changelog_links,
                    allow_changelog_github_release: !no_changelog_github_release,
                },
                crates,
                to_bump_spec(bump.as_deref().unwrap_or(DEFAULT_BUMP_SPEC))?,
                to_bump_spec(bump_dependencies.as_deref().unwrap_or(DEFAULT_BUMP_SPEC))?,
            )?
        }
    };

    Ok(())
}

const DEFAULT_BUMP_SPEC: &str = "auto";

fn to_bump_spec(spec: &str) -> anyhow::Result<cargo_smart_release::version::BumpSpec> {
    use cargo_smart_release::version::BumpSpec::*;
    Ok(match spec {
        "patch" | "Patch" => Patch,
        "minor" | "Minor" => Minor,
        "major" | "Major" => Major,
        "keep" | "Keep" => Keep,
        "auto" | "Auto" => Auto,
        unknown_spec => anyhow::bail!("Unknown bump specification: {:?}", unknown_spec),
    })
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
                "git-conventional" => Selection::GIT_CONVENTIONAL,
                other => anyhow::bail!("Invalid changelog segment selector: {:?}", other),
            };
        }
        Selection::all().difference(deselected)
    })
}

fn init_logging(verbose: bool) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(if verbose { "trace" } else { "info" }))
        .format_module_path(false)
        .format_target(false)
        .format_timestamp(None)
        .init();
}
