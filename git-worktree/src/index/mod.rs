use bstr::BStr;
use git_features::progress::Progress;
use git_hash::oid;

use crate::index;
use crate::index::checkout::{ErrorRecord, PathCache};

pub mod checkout;
pub(crate) mod entry;

pub fn checkout<Find, E>(
    index: &mut git_index::State,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    options: checkout::Options,
) -> Result<checkout::Outcome, checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    if !options.destination_is_initially_empty {
        todo!("deal with non-clone checkouts")
    }

    let mut path_cache = PathCache::new(dir.into());
    path_cache.unlink_on_collision = options.overwrite_existing;

    let mut delayed = Vec::new();

    let mut ctx = Context {
        buf: Vec::new(),
        collisions: Vec::new(),
        errors: Vec::new(),
        path_cache,
        files,
        bytes,
        find,
        options,
    };

    for (entry, entry_path) in index.entries_mut_with_paths() {
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
            // if entry.mode == git_index::entry::Mode::SYMLINK {
            delayed.push((entry, entry_path));
            continue;
        }

        checkout_entry_handle_result(entry, entry_path, &mut ctx)?;
    }

    for (entry, entry_path) in delayed {
        checkout_entry_handle_result(entry, entry_path, &mut ctx)?;
    }

    Ok(checkout::Outcome {
        collisions: ctx.collisions,
        errors: ctx.errors,
    })
}

struct Context<'a, P1, P2, Find> {
    bytes: &'a mut P1,
    files: &'a mut P2,
    find: Find,
    path_cache: PathCache,
    buf: Vec<u8>,
    options: checkout::Options,
    errors: Vec<ErrorRecord>,
    collisions: Vec<checkout::Collision>,
}

fn checkout_entry_handle_result<Find, E, P1, P2>(
    entry: &mut git_index::Entry,
    entry_path: &BStr,
    Context {
        bytes,
        files,
        find,
        path_cache,
        buf,
        options,
        errors,
        collisions,
    }: &mut Context<'_, P1, P2, Find>,
) -> Result<(), checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
    P1: Progress,
    P2: Progress,
{
    use std::io::ErrorKind::AlreadyExists;

    let res = entry::checkout(entry, entry_path, find, path_cache, *options, buf);
    files.inc();
    match res {
        Ok(object_size) => bytes.inc_by(object_size),
        #[cfg(windows)]
        Err(index::checkout::Error::Io(err))
            if err.kind() == AlreadyExists || err.kind() == std::io::ErrorKind::PermissionDenied =>
        {
            collisions.push(checkout::Collision {
                path: entry_path.into(),
                error_kind: err.kind(),
            });
        }
        // TODO: use ::IsDirectory as well when stabilized instead of raw_os_error()
        #[cfg(not(windows))]
        Err(index::checkout::Error::Io(err)) if err.kind() == AlreadyExists || err.raw_os_error() == Some(21) => {
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
                errors.push(ErrorRecord {
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
