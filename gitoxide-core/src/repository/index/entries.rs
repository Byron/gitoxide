pub fn entries(repo: gix::Repository, mut out: impl std::io::Write, format: crate::OutputFormat) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    let index = repo.index()?;

    #[cfg(feature = "serde")]
    if let Json = format {
        out.write_all(b"[\n")?;
    }

    let mut entries = index.entries().iter().peekable();
    while let Some(entry) = entries.next() {
        match format {
            Human => to_human(&mut out, &index, entry)?,
            #[cfg(feature = "serde")]
            Json => to_json(&mut out, &index, entry, entries.peek().is_none())?,
        }
    }

    #[cfg(feature = "serde")]
    if let Json = format {
        out.write_all(b"]\n")?;
    }
    Ok(())
}

#[cfg(feature = "serde")]
pub(crate) fn to_json(
    mut out: &mut impl std::io::Write,
    index: &gix::index::File,
    entry: &gix::index::Entry,
    is_last: bool,
) -> anyhow::Result<()> {
    use gix::bstr::ByteSlice;

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
            path: entry.path(index).to_str_lossy(),
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
