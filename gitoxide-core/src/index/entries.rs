use git_repository as git;

#[cfg(feature = "serde1")]
pub(crate) fn to_json(
    mut out: &mut impl std::io::Write,
    file: &git::index::File,
    entry: &git::index::Entry,
    is_last: bool,
) -> anyhow::Result<()> {
    use git_repository::bstr::ByteSlice;

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

pub(crate) fn to_human(
    out: &mut impl std::io::Write,
    file: &git::index::File,
    entry: &git::index::Entry,
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
        entry.path(&file.state)
    )
}
