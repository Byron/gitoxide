use std::io;

#[cfg(feature = "serde")]
use gix::mailmap::Entry;

use crate::OutputFormat;

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
struct JsonEntry {
    new_name: Option<String>,
    new_email: Option<String>,
    old_name: Option<String>,
    old_email: String,
}

#[cfg(feature = "serde")]
impl<'a> From<Entry<'a>> for JsonEntry {
    fn from(v: Entry<'a>) -> Self {
        use gix::bstr::ByteSlice;
        JsonEntry {
            new_name: v.new_name().map(|s| s.to_str_lossy().into_owned()),
            new_email: v.new_email().map(|s| s.to_str_lossy().into_owned()),
            old_name: v.old_name().map(|s| s.to_str_lossy().into_owned()),
            old_email: v.old_email().to_str_lossy().into_owned(),
        }
    }
}

pub fn entries(
    repo: gix::Repository,
    format: OutputFormat,
    #[cfg_attr(not(feature = "serde"), allow(unused_variables))] out: impl io::Write,
    mut err: impl io::Write,
) -> anyhow::Result<()> {
    if format == OutputFormat::Human {
        writeln!(err, "Defaulting to JSON as human format isn't implemented").ok();
    }

    let mut mailmap = gix::mailmap::Snapshot::default();
    if let Err(e) = repo.open_mailmap_into(&mut mailmap) {
        writeln!(err, "Error while loading mailmap, the first error is: {e}").ok();
    }

    #[cfg(feature = "serde")]
    serde_json::to_writer_pretty(
        out,
        &mailmap.entries().into_iter().map(JsonEntry::from).collect::<Vec<_>>(),
    )?;

    Ok(())
}
