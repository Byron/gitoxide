use git_features::progress::Progress;
use git_hash::oid;

use crate::index;
use crate::index::checkout::{ErrorRecord, PathCache};

pub mod checkout;
pub(crate) mod entry;

pub fn checkout<Find>(
    index: &mut git_index::State,
    dir: impl Into<std::path::PathBuf>,
    mut find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    options: checkout::Options,
) -> Result<checkout::Outcome, checkout::Error>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<git_object::BlobRef<'a>>,
{
    if !options.destination_is_initially_empty {
        todo!("deal with non-clone checkouts")
    }

    use std::io::ErrorKind::AlreadyExists;
    let mut path_cache = PathCache::new(dir.into());
    path_cache.unlink_on_collision = options.overwrite_existing;

    let mut buf = Vec::new();
    let mut collisions = Vec::new();
    let mut errors = Vec::new();

    for (entry, entry_path) in index.entries_mut_with_paths() {
        // TODO: write test for that
        if entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
            files.inc();
            continue;
        }

        let res = entry::checkout(entry, entry_path, &mut find, &mut path_cache, options, &mut buf);
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
                    files.fail(format!("{}: {}", entry_path, err));
                    errors.push(ErrorRecord {
                        path: entry_path.into(),
                        error: Box::new(err),
                    });
                } else {
                    return Err(err);
                }
            }
        }
    }
    Ok(checkout::Outcome { collisions, errors })
}
