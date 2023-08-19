use std::sync::atomic::AtomicBool;

use gix_features::{interrupt, parallel::in_parallel_with_finalize, progress::Progress};
use gix_hash::oid;

use crate::checkout::chunk;
use gix_worktree::{cache, Cache};

/// Checkout the entire `index` into `dir`, and resolve objects found in index entries with `find` to write their content to their
/// respective path in `dir`.
/// Use `files` to count each fully checked out file, and count the amount written `bytes`. If `should_interrupt` is `true`, the
/// operation will abort.
/// `options` provide a lot of context on how to perform the operation.
///
/// ### Handling the return value
///
/// Note that interruption still produce an `Ok(…)` value, so the caller should look at `should_interrupt` to communicate the outcome.
///
#[allow(clippy::too_many_arguments)]
pub fn checkout<Find, E>(
    index: &mut gix_index::State,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    should_interrupt: &AtomicBool,
    options: crate::checkout::Options,
) -> Result<crate::checkout::Outcome, crate::checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Send + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let paths = index.take_path_backing();
    let res = checkout_inner(index, &paths, dir, find, files, bytes, should_interrupt, options);
    index.return_path_backing(paths);
    res
}

#[allow(clippy::too_many_arguments)]
fn checkout_inner<Find, E>(
    index: &mut gix_index::State,
    paths: &gix_index::PathStorage,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    should_interrupt: &AtomicBool,
    mut options: crate::checkout::Options,
) -> Result<crate::checkout::Outcome, crate::checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Send + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let num_files = files.counter();
    let num_bytes = bytes.counter();
    let dir = dir.into();
    let case = if options.fs.ignore_case {
        gix_glob::pattern::Case::Fold
    } else {
        gix_glob::pattern::Case::Sensitive
    };
    let (chunk_size, thread_limit, num_threads) = gix_features::parallel::optimize_chunk_size_and_thread_limit(
        100,
        index.entries().len().into(),
        options.thread_limit,
        None,
    );

    let state = cache::State::for_checkout(options.overwrite_existing, std::mem::take(&mut options.attributes));
    let attribute_files = state.id_mappings_from_index(index, paths, case);
    let mut ctx = chunk::Context {
        buf: Vec::new(),
        options: (&options).into(),
        path_cache: Cache::new(dir, state, case, Vec::with_capacity(512), attribute_files),
        filters: options.filters,
        find,
    };

    let chunk::Outcome {
        mut collisions,
        mut errors,
        mut bytes_written,
        files: files_updated,
        delayed_symlinks,
        delayed_paths_unknown,
        delayed_paths_unprocessed,
    } = if num_threads == 1 {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        let mut delayed_filter_results = Vec::new();
        let mut out = chunk::process(
            entries_with_paths,
            num_files.as_deref(),
            num_bytes.as_deref(),
            &mut delayed_filter_results,
            &mut ctx,
        )?;
        chunk::process_delayed_filter_results(
            delayed_filter_results,
            num_files.as_deref(),
            num_bytes.as_deref(),
            &mut out,
            &mut ctx,
        )?;
        out
    } else {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        in_parallel_with_finalize(
            gix_features::iter::Chunks {
                inner: entries_with_paths,
                size: chunk_size,
            },
            thread_limit,
            {
                let ctx = ctx.clone();
                move |_| (Vec::new(), ctx)
            },
            |chunk, (delayed_filter_results, ctx)| {
                chunk::process(
                    chunk.into_iter(),
                    num_files.as_deref(),
                    num_bytes.as_deref(),
                    delayed_filter_results,
                    ctx,
                )
            },
            |(delayed_filter_results, mut ctx)| {
                let mut out = chunk::Outcome::default();
                chunk::process_delayed_filter_results(
                    delayed_filter_results,
                    num_files.as_deref(),
                    num_bytes.as_deref(),
                    &mut out,
                    &mut ctx,
                )?;
                Ok(out)
            },
            chunk::Reduce {
                files: num_files.is_none().then_some(files),
                bytes: num_bytes.is_none().then_some(bytes),
                aggregate: Default::default(),
                marker: Default::default(),
            },
        )?
    };

    for (entry, entry_path) in delayed_symlinks {
        bytes_written += chunk::checkout_entry_handle_result(
            entry,
            entry_path,
            &mut errors,
            &mut collisions,
            num_files.as_deref(),
            num_bytes.as_deref(),
            &mut ctx,
        )?
        .as_bytes()
        .expect("only symlinks are delayed here, they are never filtered (or delayed again)")
            as u64;
    }

    Ok(crate::checkout::Outcome {
        files_updated,
        collisions,
        errors,
        bytes_written,
        delayed_paths_unknown,
        delayed_paths_unprocessed,
    })
}
