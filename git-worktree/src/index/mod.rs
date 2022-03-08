use git_features::interrupt;
use git_features::progress::Progress;
use git_hash::oid;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::index::checkout::PathCache;

pub mod checkout;
pub(crate) mod entry;

/// Note that interruption still produce an `Ok(…)` value, so the caller should look at `should_interrupt` to communicate the outcome.
pub fn checkout<Find, E>(
    index: &mut git_index::State,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    should_interrupt: &AtomicBool,
    options: checkout::Options,
) -> Result<checkout::Outcome, checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let num_files = AtomicUsize::default();

    let mut ctx = chunk::Context {
        buf: Vec::new(),
        path_cache: {
            let mut cache = PathCache::new(dir.into());
            cache.unlink_on_collision = options.overwrite_existing;
            cache
        },
        files,
        bytes,
        find,
        options,
        num_files: &num_files,
    };
    let (_chunk_size, _, _num_threads) = git_features::parallel::optimize_chunk_size_and_thread_limit(
        100,
        index.entries().len().into(),
        options.thread_limit,
        None,
    );

    let chunk::Outcome {
        mut collisions,
        mut errors,
        delayed,
    } = chunk::process(
        interrupt::Iter::new(index.entries_mut_with_paths(), should_interrupt),
        &mut ctx,
    )?;

    for (entry, entry_path) in delayed {
        chunk::checkout_entry_handle_result(entry, entry_path, &mut errors, &mut collisions, &mut ctx)?;
    }

    Ok(checkout::Outcome {
        files_updated: ctx.num_files.load(Ordering::Relaxed),
        collisions,
        errors,
    })
}

mod chunk {
    use bstr::BStr;
    use git_features::progress::Progress;
    use git_hash::oid;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::index::{checkout, checkout::PathCache, entry};
    use crate::{index, os};

    pub struct Outcome<'a> {
        pub collisions: Vec<checkout::Collision>,
        pub errors: Vec<checkout::ErrorRecord>,
        pub delayed: Vec<(&'a mut git_index::Entry, &'a BStr)>,
    }

    pub struct Context<'a, P1, P2, Find> {
        pub bytes: &'a mut P1,
        pub files: &'a mut P2,
        pub find: Find,
        pub path_cache: PathCache,
        pub buf: Vec<u8>,
        pub options: checkout::Options,
        /// We keep these shared so that there is the chance for printing numbers that aren't looking like
        /// multiple of chunk sizes. Purely cosmetic. Otherwise it's the same as `files`.
        pub num_files: &'a AtomicUsize,
    }

    pub fn process<'entry, Find, E, P1, P2>(
        entries_with_paths: impl Iterator<Item = (&'entry mut git_index::Entry, &'entry BStr)>,
        ctx: &mut Context<'_, P1, P2, Find>,
    ) -> Result<Outcome<'entry>, checkout::Error<E>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
        P1: Progress,
        P2: Progress,
    {
        let mut delayed = Vec::new();

        let mut collisions = Vec::new();
        let mut errors = Vec::new();

        for (entry, entry_path) in entries_with_paths {
            // TODO: write test for that
            if entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
                ctx.files.inc();
                continue;
            }

            // Symlinks always have to be delayed on windows as they have to point to something that exists on creation.
            // And even if not, there is a distinction between file and directory symlinks, hence we have to check what the target is
            // before creating it.
            // And to keep things sane, we just do the same on non-windows as well which is similar to what git does and adds some safety
            // around writing through symlinks (even though we handle this).
            // This also means that we prefer content in files over symlinks in case of collisions, which probably is for the better, too.
            if entry.mode == git_index::entry::Mode::SYMLINK {
                delayed.push((entry, entry_path));
                continue;
            }

            checkout_entry_handle_result(entry, entry_path, &mut errors, &mut collisions, ctx)?;
        }
        Ok(Outcome {
            errors,
            collisions,
            delayed,
        })
    }

    pub fn checkout_entry_handle_result<Find, E, P1, P2>(
        entry: &mut git_index::Entry,
        entry_path: &BStr,
        errors: &mut Vec<checkout::ErrorRecord>,
        collisions: &mut Vec<checkout::Collision>,
        Context {
            bytes,
            files,
            find,
            path_cache,
            buf,
            options,
            num_files,
        }: &mut Context<'_, P1, P2, Find>,
    ) -> Result<(), checkout::Error<E>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
        P1: Progress,
        P2: Progress,
    {
        let res = entry::checkout(entry, entry_path, find, path_cache, *options, buf);
        files.inc();
        num_files.fetch_add(1, Ordering::SeqCst);
        match res {
            Ok(object_size) => bytes.inc_by(object_size),
            Err(index::checkout::Error::Io(err)) if os::indicates_collision(&err) => {
                // We are here because a file existed or was blocked by a directory which shouldn't be possible unless
                // we are on a file insensitive file system.
                files.fail(format!("{}: collided ({:?})", entry_path, err.kind()));
                collisions.push(checkout::Collision {
                    path: entry_path.into(),
                    error_kind: err.kind(),
                });
            }
            Err(err) => {
                if options.keep_going {
                    errors.push(checkout::ErrorRecord {
                        path: entry_path.into(),
                        error: Box::new(err),
                    });
                } else {
                    return Err(err);
                }
            }
        };
        Ok(())
    }
}
