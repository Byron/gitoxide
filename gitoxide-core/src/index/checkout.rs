use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::bail;
use git::{odb::FindExt, worktree::index::checkout, Progress};
use git_repository as git;

use crate::{
    index,
    index::{parse_file, Options},
};

pub fn checkout_exclusive(
    index_path: impl AsRef<Path>,
    dest_directory: impl AsRef<Path>,
    repo: Option<PathBuf>,
    mut err: impl std::io::Write,
    mut progress: impl Progress,
    should_interrupt: &AtomicBool,
    index::checkout_exclusive::Options {
        index: Options { object_hash, .. },
        empty_files,
        keep_going,
        thread_limit,
    }: index::checkout_exclusive::Options,
) -> anyhow::Result<()> {
    let repo = repo.map(git_repository::discover).transpose()?;

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
    let maybe_symlink_mode = if !empty_files && repo.is_some() {
        git::index::entry::Mode::DIR
    } else {
        git::index::entry::Mode::SYMLINK
    };
    for entry in index.entries_mut().iter_mut().filter(|e| {
        e.mode
            .contains(maybe_symlink_mode | git::index::entry::Mode::DIR | git::index::entry::Mode::COMMIT)
    }) {
        entry.flags.insert(git::index::entry::Flags::SKIP_WORKTREE);
        num_skipped += 1;
    }
    if num_skipped > 0 {
        progress.info(format!("Skipping {} DIR/SYMLINK/COMMIT entries", num_skipped));
    }

    let opts = git::worktree::index::checkout::Options {
        fs: git::worktree::fs::Capabilities::probe(dest_directory),

        destination_is_initially_empty: true,
        overwrite_existing: false,
        keep_going,
        thread_limit,
        ..Default::default()
    };

    let mut files = progress.add_child("checkout");
    let mut bytes = progress.add_child("writing");

    let entries_for_checkout = index.entries().len() - num_skipped;
    files.init(Some(entries_for_checkout), git::progress::count("files"));
    bytes.init(None, git::progress::bytes());

    let start = std::time::Instant::now();
    let no_repo = repo.is_none();
    let checkout::Outcome {
        errors,
        collisions,
        files_updated,
        bytes_written,
    } = match repo {
        Some(repo) => git::worktree::index::checkout(
            &mut index,
            dest_directory,
            {
                let objects = repo.objects.into_arc()?;
                move |oid, buf| {
                    objects.find_blob(oid, buf).ok();
                    if empty_files {
                        // We always want to query the ODB here…
                        objects.find_blob(oid, buf)?;
                        buf.clear();
                        // …but write nothing
                        Ok(git::objs::BlobRef { data: buf })
                    } else {
                        objects.find_blob(oid, buf)
                    }
                }
            },
            &mut files,
            &mut bytes,
            should_interrupt,
            opts,
        ),
        None => git::worktree::index::checkout(
            &mut index,
            dest_directory,
            |_, buf| {
                buf.clear();
                Ok(git::objs::BlobRef { data: buf })
            },
            &mut files,
            &mut bytes,
            should_interrupt,
            opts,
        ),
    }?;

    files.show_throughput(start);
    bytes.show_throughput(start);

    progress.done(format!(
        "Created {} {} files{} ({})",
        files_updated,
        no_repo.then(|| "empty").unwrap_or_default(),
        should_interrupt
            .load(Ordering::Relaxed)
            .then(|| {
                format!(
                    " of {}",
                    entries_for_checkout.saturating_sub(errors.len() + collisions.len())
                )
            })
            .unwrap_or_default(),
        git::progress::bytes()
            .unwrap()
            .display(bytes_written as usize, None, None)
    ));

    if !(collisions.is_empty() && errors.is_empty()) {
        let mut messages = Vec::new();
        if !errors.is_empty() {
            messages.push(format!("kept going through {} errors(s)", errors.len()));
            for record in errors {
                writeln!(err, "{}: {}", record.path, record.error).ok();
            }
        }
        if !collisions.is_empty() {
            messages.push(format!("encountered {} collision(s)", collisions.len()));
            for col in collisions {
                writeln!(err, "{}: collision ({:?})", col.path, col.error_kind).ok();
            }
        }
        bail!(
            "One or more errors occurred - checkout is incomplete: {}",
            messages.join(", ")
        );
    }
    Ok(())
}
