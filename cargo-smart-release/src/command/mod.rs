pub mod release {
    use crate::changelog::section::segment;

    #[derive(Debug, Clone, Copy)]
    pub struct Options {
        pub dry_run: bool,
        pub allow_dirty: bool,
        pub ignore_instability: bool,
        pub skip_publish: bool,
        pub dry_run_cargo_publish: bool,
        pub conservative_pre_release_version_handling: bool,
        /// Pass --no-verify unconditionally to cargo publish. Really just for fixing things
        pub no_verify: bool,
        pub skip_tag: bool,
        pub allow_auto_publish_of_stable_crates: bool,
        pub update_crates_index: bool,
        pub bump_when_needed: bool,
        pub verbose: bool,
        pub skip_push: bool,
        pub dependencies: bool,
        pub isolate_dependencies_from_breaking_changes: bool,
        pub changelog: bool,
        pub preview: bool,
        pub generator_segments: segment::Selection,
        pub allow_fully_generated_changelogs: bool,
        pub changelog_links: bool,
        pub allow_changelog_github_release: bool,
        pub capitalize_commit: bool,
    }
}
#[path = "release/mod.rs"]
mod release_impl;
pub use release_impl::release;

pub mod changelog {
    use crate::changelog::section::segment;

    #[derive(Debug, Clone, Copy)]
    pub struct Options {
        pub dry_run: bool,
        pub dependencies: bool,
        pub allow_dirty: bool,
        pub preview: bool,
        // All the segments to generate
        pub generator_segments: segment::Selection,
        pub no_links: bool,
        pub capitalize_commit: bool,
    }
}
#[path = "changelog.rs"]
mod changelog_impl;
pub use changelog_impl::changelog;
