use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use gix_features::{interrupt, parallel::in_parallel, progress, progress::Progress};
use gix_hash::oid;

use crate::{cache, checkout::chunk, Cache};

/// Note that interruption still produce an `Ok(…)` value, so the caller should look at `should_interrupt` to communicate the outcome.
/// `dir` is the directory into which to checkout the `index`.
/// `git_dir` is the `.git` directory for reading additional per-repository configuration files.
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
    options: crate::checkout::Options,
) -> Result<crate::checkout::Outcome, crate::checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Send + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let num_files = AtomicUsize::default();
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

    let state = cache::State::for_checkout(options.overwrite_existing, options.attributes.clone());
    let attribute_files = state.id_mappings_from_index(index, paths, Default::default(), case);
    let mut ctx = chunk::Context {
        buf: Vec::new(),
        path_cache: Cache::new(dir, state, case, Vec::with_capacity(512), attribute_files),
        find,
        options,
        num_files: &num_files,
    };

    let chunk::Outcome {
        mut collisions,
        mut errors,
        mut bytes_written,
        delayed,
    } = if num_threads == 1 {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        chunk::process(entries_with_paths, files, bytes, &mut ctx)?
    } else {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        in_parallel(
            gix_features::iter::Chunks {
                inner: entries_with_paths,
                size: chunk_size,
            },
            thread_limit,
            {
                let ctx = ctx.clone();
                move |_| (progress::Discard, progress::Discard, ctx.clone())
            },
            |chunk, (files, bytes, ctx)| chunk::process(chunk.into_iter(), files, bytes, ctx),
            chunk::Reduce {
                files,
                bytes,
                num_files: &num_files,
                aggregate: Default::default(),
                marker: Default::default(),
            },
        )?
    };

    for (entry, entry_path) in delayed {
        bytes_written += chunk::checkout_entry_handle_result(
            entry,
            entry_path,
            &mut errors,
            &mut collisions,
            files,
            bytes,
            &mut ctx,
        )? as u64;
    }

    Ok(crate::checkout::Outcome {
        files_updated: num_files.load(Ordering::Relaxed),
        collisions,
        errors,
        bytes_written,
    })
}
