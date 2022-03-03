use git_features::progress::Progress;
use git_hash::oid;

use crate::{index, index::checkout::Collision};

pub mod checkout;
pub(crate) mod entry;

pub fn checkout<Find>(
    index: &mut git_index::State,
    path: impl AsRef<std::path::Path>,
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
    let root = path.as_ref();
    let mut buf = Vec::new();
    let mut collisions = Vec::new();

    for (entry, entry_path) in index.entries_mut_with_paths() {
        // TODO: write test for that
        if entry.flags.contains(git_index::entry::Flags::SKIP_WORKTREE) {
            files.inc();
            continue;
        }

        let res = entry::checkout(entry, entry_path, &mut find, root, options, &mut buf);
        files.inc();
        match res {
            Ok(object_size) => bytes.inc_by(object_size),
            #[cfg(windows)]
            Err(index::checkout::Error::Io(err))
                if err.kind() == AlreadyExists || err.kind() == std::io::ErrorKind::PermissionDenied =>
            {
                collisions.push(Collision {
                    path: entry_path.into(),
                    error_kind: err.kind(),
                });
            }
            // TODO: use ::IsDirectory as well when stabilized instead of raw_os_error()
            #[cfg(not(windows))]
            Err(index::checkout::Error::Io(err)) if err.kind() == AlreadyExists || err.raw_os_error() == Some(21) => {
                // We are here because a file existed or was blocked by a directory which shouldn't be possible unless
                // we are on a file insensitive file system.
                collisions.push(Collision {
                    path: entry_path.into(),
                    error_kind: err.kind(),
                });
            }
            Err(err) => {
                if options.keep_going {
                    todo!("keep going")
                } else {
                    return Err(err);
                }
            }
        }
    }
    Ok(checkout::Outcome { collisions })
}
