use anyhow::bail;
use std::path::Path;

use git_repository as git;
use git_repository::Progress;

pub struct Options {
    pub object_hash: git::hash::Kind,
    pub format: crate::OutputFormat,
}

mod entries;

pub mod information;

pub fn verify(
    index_path: impl AsRef<Path>,
    mut out: impl std::io::Write,
    Options { object_hash, format }: Options,
) -> anyhow::Result<()> {
    let file = parse_file(index_path, object_hash)?;
    file.verify_integrity()?;
    file.verify_entries()?;
    file.verify_extensions(false, git::index::verify::extensions::no_find)?;
    #[cfg_attr(not(feature = "serde1"), allow(irrefutable_let_patterns))]
    if let crate::OutputFormat::Human = format {
        writeln!(out, "OK").ok();
    }
    Ok(())
}

#[cfg_attr(not(feature = "serde1"), allow(unused_variables, unused_mut))]
pub fn information(
    index_path: impl AsRef<Path>,
    out: impl std::io::Write,
    mut err: impl std::io::Write,
    information::Options {
        index: Options {
            object_hash,
            mut format,
        },
        extension_details,
    }: information::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    #[cfg(feature = "serde1")]
    if let Human = format {
        writeln!(err, "Defaulting to JSON printing as nothing else will be implemented.").ok();
        format = Json;
    }
    match format {
        Human => {
            anyhow::bail!("Cannot print information using 'human' format.")
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

pub fn checkout_exclusive(
    index_path: impl AsRef<Path>,
    dest_directory: impl AsRef<Path>,
    mut progress: impl Progress,
    Options { object_hash, .. }: Options,
) -> anyhow::Result<()> {
    let dest_directory = dest_directory.as_ref();
    if dest_directory.exists() {
        bail!(
            "Refusing to checkout index into existing directory '{}' - remove it and try again",
            dest_directory.display()
        )
    }
    std::fs::create_dir_all(dest_directory)?;

    let mut index = parse_file(index_path, object_hash)?;

    let mut num_skipped = 0;
    for entry in index.entries_mut().iter_mut().filter(|e| {
        e.mode
            .contains(git::index::entry::Mode::DIR | git::index::entry::Mode::SYMLINK | git::index::entry::Mode::COMMIT)
    }) {
        entry.flags.insert(git::index::entry::Flags::SKIP_WORKTREE);
        num_skipped += 1;
    }
    if num_skipped > 0 {
        progress.info(format!("Skipping {} DIR/SYMLINK/COMMIT entries", num_skipped));
    }

    let opts = git::worktree::index::checkout::Options {
        fs: git::worktree::fs::Capabilities::probe(dest_directory),

        // TODO: turn the two following flags into an enum
        destination_is_initially_empty: true,
        overwrite_existing: false,
        ..Default::default()
    };

    let mut files = progress.add_child("checkout");
    let mut bytes = progress.add_child("writing");

    let entries_for_checkout = index.entries().len() - num_skipped;
    files.init(Some(entries_for_checkout), git::progress::count("files"));
    bytes.init(Some(entries_for_checkout), git::progress::bytes());

    let start = std::time::Instant::now();
    git::worktree::index::checkout(
        &mut index,
        dest_directory,
        |_, buf| {
            buf.clear();
            Some(git::objs::BlobRef { data: buf })
        },
        &mut files,
        &mut bytes,
        opts,
    )?;

    files.show_throughput(start);
    bytes.show_throughput(start);

    progress.done(format!("Created {} empty files", entries_for_checkout));
    Ok(())
}
