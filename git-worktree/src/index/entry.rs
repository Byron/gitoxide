use std::path::Path;
use std::{convert::TryInto, fs::OpenOptions, io::Write, time::Duration};

use bstr::BStr;
use git_hash::oid;
use git_index::Entry;

use crate::index::checkout::PathCache;
use crate::{index, os};

#[cfg_attr(not(unix), allow(unused_variables))]
pub fn checkout<Find, E>(
    entry: &mut Entry,
    entry_path: &BStr,
    find: &mut Find,
    cache: &mut PathCache,
    index::checkout::Options {
        fs: crate::fs::Capabilities {
            symlink,
            executable_bit,
            ..
        },
        destination_is_initially_empty,
        overwrite_existing,
        ..
    }: index::checkout::Options,
    buf: &mut Vec<u8>,
) -> Result<usize, index::checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<git_object::BlobRef<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let dest_relative =
        git_features::path::from_byte_slice(entry_path).map_err(|_| index::checkout::Error::IllformedUtf8 {
            path: entry_path.to_owned(),
        })?;
    let dest = cache.append_relative_path_assure_leading_dir(dest_relative, entry.mode)?;

    let object_size = match entry.mode {
        git_index::entry::Mode::FILE | git_index::entry::Mode::FILE_EXECUTABLE => {
            let obj = find(&entry.id, buf).map_err(|err| index::checkout::Error::Find {
                err,
                oid: entry.id,
                path: dest.to_path_buf(),
            })?;

            let mut options = open_options(dest, destination_is_initially_empty, overwrite_existing);
            let needs_executable_bit = executable_bit && entry.mode == git_index::entry::Mode::FILE_EXECUTABLE;
            #[cfg(unix)]
            if needs_executable_bit && destination_is_initially_empty {
                use std::os::unix::fs::OpenOptionsExt;
                // Note that these only work if the file was newly created, but won't if it's already
                // existing, possibly without the executable bit set. Thus we do this only if the file is new.
                options.mode(0o777);
            }

            let mut file = try_write_or_unlink(dest, overwrite_existing, |p| options.open(p))?;
            file.write_all(obj.data)?;

            // For possibly existing, overwritten files, we must change the file mode explicitly.
            #[cfg(unix)]
            if needs_executable_bit && !destination_is_initially_empty {
                use std::os::unix::fs::PermissionsExt;
                let mut perm = std::fs::symlink_metadata(&dest)?.permissions();
                perm.set_mode(0o777);
                std::fs::set_permissions(&dest, perm)?;
            }
            // NOTE: we don't call `file.sync_all()` here knowing that some filesystems don't handle this well.
            //       revisit this once there is a bug to fix.
            update_fstat(entry, file.metadata()?)?;
            obj.data.len()
        }
        git_index::entry::Mode::SYMLINK => {
            let obj = find(&entry.id, buf).map_err(|err| index::checkout::Error::Find {
                err,
                oid: entry.id,
                path: dest.to_path_buf(),
            })?;
            let symlink_destination = git_features::path::from_byte_slice(obj.data)
                .map_err(|_| index::checkout::Error::IllformedUtf8 { path: obj.data.into() })?;

            // TODO: how to deal with mode changes? Maybe this info can be passed once we check for whether
            // a checkout is needed at all.
            if symlink {
                // TODO: handle 'overwrite_existing' mode, which checks for 'exists' errors and unlinks existing files
                //       or directories or symlinks. Doing this shouldn't invalidate the cache as it's only valid till
                //       our parent anyway, but it may invalidate caches in other threads without their knowledge.
                //       collisions with overwrite mode must be handled with great care in parallel mode.
                try_write_or_unlink(dest, overwrite_existing, |p| {
                    crate::os::create_symlink(symlink_destination, p)
                })?;
            } else {
                try_write_or_unlink(dest, overwrite_existing, |p| {
                    open_options(p, destination_is_initially_empty, overwrite_existing).open(&dest)
                })?
                .write_all(obj.data)?;
            }

            update_fstat(entry, std::fs::symlink_metadata(&dest)?)?;
            obj.data.len()
        }
        git_index::entry::Mode::DIR => todo!(),
        git_index::entry::Mode::COMMIT => todo!(),
        _ => unreachable!(),
    };
    Ok(object_size)
}

/// Note that this works only because we assume to not race ourselves when symlinks are involved, and we do this by
/// delaying symlink creation to the end and will always do that sequentially.
/// It's still possible to fall for a race if other actors create symlinks in our path, but that's nothing to defend against.
fn try_write_or_unlink<T>(
    path: &Path,
    overwrite_existing: bool,
    op: impl Fn(&Path) -> std::io::Result<T>,
) -> std::io::Result<T> {
    if overwrite_existing {
        match op(path) {
            Ok(res) => Ok(res),
            Err(err) if os::indicates_collision(&err) => {
                try_unlink_path_recursively(path, &std::fs::symlink_metadata(path)?)?;
                op(path)
            }
            Err(err) => Err(err),
        }
    } else {
        op(path)
    }
}

fn try_unlink_path_recursively(path: &Path, path_meta: &std::fs::Metadata) -> std::io::Result<()> {
    if path_meta.is_dir() {
        std::fs::remove_dir_all(path)
    } else if path_meta.is_symlink() {
        symlink::remove_symlink_auto(path)
    } else {
        std::fs::remove_file(path)
    }
}

/// This is a debug assertion as we expect the machinery calling this to prevent this possibility in the first place
fn debug_assert_dest_is_no_symlink(path: &Path) {
    #[cfg(debug_assertions)]
    {
        if let Ok(meta) = path.metadata() {
            debug_assert!(
                !meta.is_symlink(),
                "BUG: should not ever allow to overwrite/write-into the target of a symbolic link: {}",
                path.display()
            );
        }
    }
}

fn open_options(path: &Path, destination_is_initially_empty: bool, overwrite_existing: bool) -> OpenOptions {
    if overwrite_existing || !destination_is_initially_empty {
        debug_assert_dest_is_no_symlink(path);
    }
    let mut options = OpenOptions::new();
    options
        .create_new(destination_is_initially_empty && !overwrite_existing)
        .create(!destination_is_initially_empty || overwrite_existing)
        .write(true);
    #[cfg(unix)]
    {
        /// Make sure that it's impossible to follow through to the target of symlinks.
        /// Note that this will still follow symlinks in the path, which is what we assume
        /// has been checked separately.
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    options
}

fn update_fstat<E>(entry: &mut Entry, meta: std::fs::Metadata) -> Result<(), index::checkout::Error<E>>
where
    E: std::error::Error + Send + Sync + 'static,
{
    let ctime = meta
        .created()
        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;
    let mtime = meta
        .modified()
        .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;

    let stat = &mut entry.stat;
    stat.mtime.secs = mtime
        .as_secs()
        .try_into()
        .expect("by 2038 we found a solution for this");
    stat.mtime.nsecs = mtime.subsec_nanos();
    stat.ctime.secs = ctime
        .as_secs()
        .try_into()
        .expect("by 2038 we found a solution for this");
    stat.ctime.nsecs = ctime.subsec_nanos();
    Ok(())
}
