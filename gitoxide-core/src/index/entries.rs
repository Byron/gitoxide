use std::path::Path;

use crate::index::{parse_file, Options};

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
            Human => to_human(&mut out, &file, entry)?,
            #[cfg(feature = "serde1")]
            Json => to_json(&mut out, &file, entry, entries.peek().is_none())?,
        }
    }

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"]\n")?;
    }
    Ok(())
}

#[cfg(feature = "serde1")]
pub(crate) fn to_json(
    mut out: &mut impl std::io::Write,
    file: &gix::index::File,
    entry: &gix::index::Entry,
    is_last: bool,
) -> anyhow::Result<()> {
    use gix::bstr::ByteSlice;

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    struct Entry<'a> {
        stat: &'a gix::index::entry::Stat,
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
            path: entry.path(file).to_str_lossy(),
        },
    )?;

    if is_last {
        out.write_all(b"\n")?;
    } else {
        out.write_all(b",\n")?;
    }
    Ok(())
}

pub(crate) fn to_human(
    out: &mut impl std::io::Write,
    file: &gix::index::File,
    entry: &gix::index::Entry,
) -> std::io::Result<()> {
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
        entry.path(file)
    )
}
