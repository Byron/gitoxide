use git_repository as git;
use git_repository::bstr::ByteSlice;
use std::io::Write;
use std::path::Path;

pub mod entries {
    use git_repository as git;

    pub struct Options {
        pub object_hash: git::hash::Kind,
        pub format: crate::OutputFormat,
    }
}

#[allow(unused)]
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
            Human => human_entry(&mut out, &file, entry)?,
            #[cfg(feature = "serde1")]
            Json => json_entry_oneline(&mut out, &file, entry, entries.peek().is_none())?,
        }
    }

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"]\n")?;
    }
    Ok(())
}

#[cfg(feature = "serde1")]
fn json_entry_oneline(
    mut out: &mut impl Write,
    file: &git::index::File,
    entry: &git::index::Entry,
    is_last: bool,
) -> anyhow::Result<()> {
    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    struct Entry<'a> {
        stat: &'a git::index::entry::Stat,
        hex_id: String,
        flags: u32,
        mode: u32,
        path: std::borrow::Cow<'a, str>,
    }

    serde_json::to_writer(
        &mut out,
        &Entry {
            stat: &entry.stat,
            hex_id: entry.id.to_hex().to_string(),
            flags: entry.flags.bits(),
            mode: entry.mode.bits(),
            path: entry.path(&file.state).to_str_lossy(),
        },
    )?;

    if is_last {
        out.write_all(b"\n")?;
    } else {
        out.write_all(b",\n")?;
    }
    Ok(())
}

fn human_entry(out: &mut impl Write, file: &git::index::File, entry: &git::index::Entry) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {}{:?} {} {}",
        match entry.flags.stage() {
            0 => "BASE   ",
            1 => "OURS   ",
            2 => "THEIRS ",
            _ => "UNKNOWN",
        },
        if entry.flags.is_empty() {
            "".to_string()
        } else {
            format!("{:?} ", entry.flags)
        },
        entry.mode,
        entry.id,
        entry.path(&file.state)
    )
}
