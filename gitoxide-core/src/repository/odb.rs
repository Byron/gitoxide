use std::{io, path::PathBuf};

use anyhow::bail;
use git_repository as git;

use crate::OutputFormat;

mod info {
    use std::path::PathBuf;

    use git_repository::odb::store;

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub struct Statistics {
        pub path: PathBuf,
        pub object_hash: String,
        pub use_multi_pack_index: bool,
        pub structure: Vec<store::structure::Record>,
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
        structure: store.structure()?,
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
