use std::path::Path;

use git_repository as git;

pub mod entries;

pub fn entries(
    index_path: impl AsRef<Path>,
    mut out: impl std::io::Write,
    entries::Options { object_hash, format }: entries::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    let file = git::index::File::at(
        index_path.as_ref(),
        git::index::decode::Options {
            object_hash,
            ..Default::default()
        },
    )?;

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
