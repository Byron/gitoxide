use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};

pub fn init(directory: Option<PathBuf>) -> Result<git_repository::Path> {
    git_repository::path::create::into(directory.unwrap_or_default(), git_repository::Kind::WorkTree)
        .with_context(|| "Repository initialization failed")
}

pub mod verify {
    use crate::pack;
    use crate::OutputFormat;
    use std::{path::PathBuf, sync::atomic::AtomicBool};

    use git_repository::Progress;

    /// A general purpose context for many operations provided here
    pub struct Context {
        /// If set, provide statistics to `out` in the given format
        pub output_statistics: Option<OutputFormat>,
        /// If set, don't use more than this amount of threads.
        /// Otherwise, usually use as many threads as there are logical cores.
        /// A value of 0 is interpreted as no-limit
        pub thread_limit: Option<usize>,
        pub verify_mode: pack::verify::Mode,
        pub algorithm: pack::verify::Algorithm,
    }

    pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

    pub fn integrity(
        repo: PathBuf,
        mut out: impl std::io::Write,
        progress: impl Progress,
        should_interrupt: &AtomicBool,
        Context {
            output_statistics,
            thread_limit,
            verify_mode,
            algorithm,
        }: Context,
    ) -> anyhow::Result<()> {
        let repo = git_repository::open(repo)?;
        #[cfg_attr(not(feature = "serde1"), allow(unused))]
        let outcome = repo.objects.verify_integrity(
            progress,
            should_interrupt,
            git_repository::odb::pack::index::verify::integrity::Options {
                verify_mode,
                traversal: algorithm.into(),
                thread_limit,
                // TODO: a way to get the pack cache from a handle
                make_pack_lookup_cache: || git_repository::odb::pack::cache::Never,
            },
        )?;
        match output_statistics {
            Some(OutputFormat::Human) => writeln!(out, "Human output is currently unsupported, use JSON instead")?,
            #[cfg(feature = "serde1")]
            Some(OutputFormat::Json) => {
                serde_json::to_writer_pretty(
                    out,
                    &serde_json::json!({
                        "index_statistics" : outcome.index_statistics,
                        "loose_object-stores" : outcome.loose_object_stores
                    }),
                )?;
            }
            None => {}
        }
        Ok(())
    }
}
