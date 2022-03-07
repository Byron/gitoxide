use std::{convert::TryInto, fs::OpenOptions, io::Write, time::Duration};

use bstr::BStr;
use git_hash::oid;
use git_index::Entry;

use crate::index;
use crate::index::checkout::PathCache;

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
    let dest = cache.append_relative_path_assure_leading_dir(
        git_features::path::from_byte_slice(entry_path).map_err(|_| index::checkout::Error::IllformedUtf8 {
            path: entry_path.to_owned(),
        })?,
        entry.mode,
    )?;

    let object_size = match entry.mode {
        git_index::entry::Mode::FILE | git_index::entry::Mode::FILE_EXECUTABLE => {
            let obj = find(&entry.id, buf).map_err(|err| index::checkout::Error::Find {
                err,
                oid: entry.id,
                path: dest.to_path_buf(),
            })?;
            let mut options = open_options(destination_is_initially_empty, overwrite_existing);
            #[cfg(unix)]
            if executable_bit && entry.mode == git_index::entry::Mode::FILE_EXECUTABLE {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o777);
            }

            let mut file = options.open(&dest)?;
            file.write_all(obj.data)?;
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
                crate::os::create_symlink(symlink_destination, dest)?;
            } else {
                open_options(destination_is_initially_empty, overwrite_existing)
                    .open(&dest)?
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

fn open_options(destination_is_initially_empty: bool, overwrite_existing: bool) -> OpenOptions {
    let mut options = OpenOptions::new();
    options
        .create_new(destination_is_initially_empty && !overwrite_existing)
        .create(!destination_is_initially_empty || overwrite_existing)
        .write(true);
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
