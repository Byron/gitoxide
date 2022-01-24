use std::path::Path;

use git_repository as git;

pub struct Options {
    pub object_hash: git::hash::Kind,
    pub format: crate::OutputFormat,
}

mod entries;

pub mod information;

#[cfg_attr(not(feature = "serde1"), allow(unused_variables))]
pub fn information(
    index_path: impl AsRef<Path>,
    out: impl std::io::Write,
    information::Options {
        index: Options { object_hash, format },
        extension_details,
    }: information::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    match format {
        Human => {
            anyhow::bail!("Only JSON output is implemented");
        }
        #[cfg(feature = "serde1")]
        Json => {
            let info = information::Collection::try_from_file(parse_file(index_path, object_hash)?, extension_details)?;
            serde_json::to_writer_pretty(out, &info)?;
            Ok(())
        }
    }
}

pub fn entries(
    index_path: impl AsRef<Path>,
    mut out: impl std::io::Write,
    Options { object_hash, format }: Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    let file = parse_file(index_path, object_hash)?;

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"[\n")?;
    }

    let mut entries = file.entries().iter().peekable();
    while let Some(entry) = entries.next() {
        match format {
            Human => entries::to_human(&mut out, &file, entry)?,
            #[cfg(feature = "serde1")]
            Json => entries::to_json(&mut out, &file, entry, entries.peek().is_none())?,
        }
    }

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"]\n")?;
    }
    Ok(())
}

fn parse_file(index_path: impl AsRef<Path>, object_hash: git::hash::Kind) -> anyhow::Result<git::index::File> {
    git::index::File::at(
        index_path.as_ref(),
        git::index::decode::Options {
            object_hash,
            ..Default::default()
        },
    )
    .map_err(Into::into)
}
