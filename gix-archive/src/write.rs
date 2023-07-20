use crate::{Error, Format, Options};
use gix_worktree_stream::{Entry, Stream};

/// Write all stream entries in `stream` as provided by `next_entry(stream)` to `out` configured according to `opts`.
///
/// ### Performance
///
/// * The caller should be sure `out` is fast enough. If in doubt, wrap in [`std::io::BufWriter`].
/// * Further, big files aren't suitable for archival into `tar` archives as they require the size of the stream to be known
///   prior to writing the header of each entry.
#[cfg_attr(not(feature = "tar"), allow(unused_mut, unused_variables))]
pub fn write_stream<NextFn>(
    stream: &mut Stream,
    mut next_entry: NextFn,
    out: impl std::io::Write,
    opts: Options,
) -> Result<(), Error>
where
    NextFn: FnMut(&mut Stream) -> Result<Option<Entry<'_>>, gix_worktree_stream::entry::Error>,
{
    if opts.format == Format::InternalTransientNonPersistable {
        return Err(Error::InternalFormatMustNotPersist);
    }
    #[cfg(feature = "tar")]
    {
        enum State<W: std::io::Write> {
            #[cfg(feature = "tar")]
            Tar((tar::Builder<W>, Vec<u8>)),
        }

        impl<W: std::io::Write> State<W> {
            pub fn new(format: Format, out: W) -> Self {
                match format {
                    Format::InternalTransientNonPersistable => unreachable!("handled earlier"),
                    #[cfg(feature = "tar")]
                    Format::Tar => State::Tar((
                        {
                            let mut ar = tar::Builder::new(out);
                            ar.mode(tar::HeaderMode::Deterministic);
                            ar
                        },
                        Vec::with_capacity(64 * 1024),
                    )),
                }
            }
        }

        let mut state = State::new(opts.format, out);
        let mtime_seconds_since_epoch = opts
            .modification_time
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs());

        while let Some(mut entry) = next_entry(stream)? {
            match &mut state {
                #[cfg(feature = "tar")]
                State::Tar((ar, buf)) => {
                    let mut header = tar::Header::new_gnu();
                    if let Some(mtime) = mtime_seconds_since_epoch {
                        header.set_mtime(mtime);
                    }
                    header.set_entry_type(tar_entry_type(entry.mode));
                    header.set_mode(if matches!(entry.mode, gix_object::tree::EntryMode::BlobExecutable) {
                        0o755
                    } else {
                        0o644
                    });
                    buf.clear();
                    std::io::copy(&mut entry, buf)?;

                    let path = gix_path::from_bstr(add_prefix(entry.relative_path(), opts.tree_prefix.as_ref()));
                    header.set_size(buf.len() as u64);

                    if entry.mode == gix_object::tree::EntryMode::Link {
                        use bstr::ByteSlice;
                        let target = gix_path::from_bstr(buf.as_bstr());
                        header.set_entry_type(tar::EntryType::Symlink);
                        header.set_size(0);
                        ar.append_link(&mut header, path, target)?;
                    } else {
                        ar.append_data(&mut header, path, buf.as_slice())?;
                    }
                }
            }
        }

        match state {
            #[cfg(feature = "tar")]
            State::Tar((mut ar, _)) => {
                ar.finish()?;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "tar")]
fn tar_entry_type(mode: gix_object::tree::EntryMode) -> tar::EntryType {
    use gix_object::tree::EntryMode;
    use tar::EntryType;
    match mode {
        EntryMode::Tree | EntryMode::Commit => EntryType::Directory,
        EntryMode::Blob => EntryType::Regular,
        EntryMode::BlobExecutable => EntryType::Regular,
        EntryMode::Link => EntryType::Link,
    }
}

#[cfg(feature = "tar")]
fn add_prefix<'a>(relative_path: &'a bstr::BStr, prefix: Option<&bstr::BString>) -> std::borrow::Cow<'a, bstr::BStr> {
    use std::borrow::Cow;
    match prefix {
        None => Cow::Borrowed(relative_path),
        Some(prefix) => {
            use bstr::ByteVec;
            let mut buf = prefix.clone();
            buf.push_str(relative_path);
            Cow::Owned(buf)
        }
    }
}
