use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};

pub fn init(directory: Option<PathBuf>) -> Result<git_repository::Path> {
    git_repository::path::create::into(directory.unwrap_or_default(), git_repository::Kind::WorkTree)
        .with_context(|| "Repository initialization failed")
}

pub mod tree;

pub mod verify;

pub mod odb {
    use crate::OutputFormat;
    use anyhow::bail;
    use git_repository as git;
    use std::io;
    use std::path::PathBuf;

    mod info {
        use git_repository::odb::store;
        use std::path::PathBuf;

        #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
        pub struct Statistics {
            pub path: PathBuf,
            pub object_hash: String,
            pub use_multi_pack_index: bool,
            pub metrics: store::Metrics,
        }
    }

    #[cfg_attr(not(feature = "serde1"), allow(unused_variables))]
    pub fn info(
        repository: PathBuf,
        format: OutputFormat,
        out: impl io::Write,
        mut err: impl io::Write,
    ) -> anyhow::Result<()> {
        if format == OutputFormat::Human {
            writeln!(err, "Only JSON is implemented - using that instead")?;
        }

        let repo = git::open(repository)?.apply_environment();
        let store = repo.objects.store_ref();
        let stats = info::Statistics {
            path: store.path().into(),
            object_hash: store.object_hash().to_string(),
            use_multi_pack_index: store.use_multi_pack_index(),
            metrics: store.metrics(),
        };

        #[cfg(feature = "serde1")]
        {
            serde_json::to_writer_pretty(out, &stats)?;
        }

        Ok(())
    }

    pub fn entries(repository: PathBuf, format: OutputFormat, mut out: impl io::Write) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("Only human output format is supported at the moment");
        }

        let repo = git::open(repository)?.apply_environment();

        for object in repo.objects.iter()? {
            let object = object?;
            writeln!(out, "{}", object)?;
        }

        Ok(())
    }
}
