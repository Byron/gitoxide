use gix_worktree_stream::{Entry, Stream};

use crate::{Error, Format, Options};

/// Write all stream entries in `stream` as provided by `next_entry(stream)` to `out` configured according to `opts` which
/// also includes the streaming format.
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
    #[cfg(any(feature = "tar", feature = "tar_gz"))]
    {
        enum State<W: std::io::Write> {
            #[cfg(feature = "tar")]
            Tar((tar::Builder<W>, Vec<u8>)),
            #[cfg(feature = "tar_gz")]
            TarGz((tar::Builder<flate2::write::GzEncoder<W>>, Vec<u8>)),
        }

        impl<W: std::io::Write> State<W> {
            pub fn new(format: Format, mtime: gix_date::SecondsSinceUnixEpoch, out: W) -> Result<Self, Error> {
                Ok(match format {
                    Format::InternalTransientNonPersistable => unreachable!("handled earlier"),
                    Format::Zip { .. } => return Err(Error::ZipWithoutSeek),
                    #[cfg(feature = "tar")]
                    Format::Tar => {
                        #[cfg(feature = "tar")]
                        {
                            State::Tar((
                                {
                                    let mut ar = tar::Builder::new(out);
                                    ar.mode(tar::HeaderMode::Deterministic);
                                    ar
                                },
                                Vec::with_capacity(64 * 1024),
                            ))
                        }
                        #[cfg(not(feature = "tar"))]
                        {
                            Err(Error::SupportNotCompiledIn { wanted: Format::Tar })
                        }
                    }
                    Format::TarGz { compression_level } => {
                        #[cfg(feature = "tar_gz")]
                        {
                            State::TarGz((
                                {
                                    let gz = flate2::GzBuilder::new().mtime(mtime as u32).write(
                                        out,
                                        match compression_level {
                                            None => flate2::Compression::default(),
                                            Some(level) => flate2::Compression::new(level as u32),
                                        },
                                    );
                                    let mut ar = tar::Builder::new(gz);
                                    ar.mode(tar::HeaderMode::Deterministic);
                                    ar
                                },
                                Vec::with_capacity(64 * 1024),
                            ))
                        }
                        #[cfg(not(feature = "tar_gz"))]
                        {
                            Err(Error::SupportNotCompiledIn { wanted: Format::TarGz })
                        }
                    }
                })
            }
        }

        let mut state = State::new(opts.format, opts.modification_time, out)?;
        while let Some(entry) = next_entry(stream)? {
            match &mut state {
                #[cfg(feature = "tar")]
                State::Tar((ar, buf)) => {
                    append_tar_entry(ar, buf, entry, opts.modification_time, &opts)?;
                }
                #[cfg(feature = "tar_gz")]
                State::TarGz((ar, buf)) => {
                    append_tar_entry(ar, buf, entry, opts.modification_time, &opts)?;
                }
            }
        }

        match state {
            #[cfg(feature = "tar")]
            State::Tar((mut ar, _)) => {
                ar.finish()?;
            }
            #[cfg(feature = "tar_gz")]
            State::TarGz((ar, _)) => {
                ar.into_inner()?.finish()?;
            }
        }
    }
    Ok(())
}

/// Like [`write_stream()`], but requires [`std::io::Seek`] for `out`.
///
/// Note that `zip` is able to stream big files, which our `tar` implementation is not able to do, which makes it the
/// only suitable container to support huge files from `git-lfs` without consuming excessive amounts of memory.
#[cfg_attr(not(feature = "zip"), allow(unused_mut, unused_variables))]
pub fn write_stream_seek<NextFn>(
    stream: &mut Stream,
    mut next_entry: NextFn,
    out: impl std::io::Write + std::io::Seek,
    opts: Options,
) -> Result<(), Error>
where
    NextFn: FnMut(&mut Stream) -> Result<Option<Entry<'_>>, gix_worktree_stream::entry::Error>,
{
    let compression_level = match opts.format {
        Format::Zip { compression_level } => compression_level.map(|lvl| lvl as i32),
        _other => return write_stream(stream, next_entry, out, opts),
    };

    #[cfg(feature = "zip")]
    {
        let mut ar = zip::write::ZipWriter::new(out);
        let mut buf = Vec::new();
        let mtime = time::OffsetDateTime::from_unix_timestamp(opts.modification_time)
            .map_err(|err| Error::InvalidModificationTime(Box::new(err)))?
            .try_into()
            .map_err(|err| Error::InvalidModificationTime(Box::new(err)))?;
        while let Some(entry) = next_entry(stream)? {
            append_zip_entry(
                &mut ar,
                entry,
                &mut buf,
                mtime,
                compression_level,
                opts.tree_prefix.as_ref(),
            )?;
        }
        ar.finish()
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    }

    Ok(())
}

#[cfg(feature = "zip")]
fn append_zip_entry<W: std::io::Write + std::io::Seek>(
    ar: &mut zip::write::ZipWriter<W>,
    mut entry: gix_worktree_stream::Entry<'_>,
    buf: &mut Vec<u8>,
    mtime: zip::DateTime,
    compression_level: Option<i32>,
    tree_prefix: Option<&bstr::BString>,
) -> Result<(), Error> {
    let file_opts = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(compression_level)
        .large_file(entry.bytes_remaining().map_or(true, |len| len > u32::MAX as usize))
        .last_modified_time(mtime)
        .unix_permissions(if matches!(entry.mode, gix_object::tree::EntryMode::BlobExecutable) {
            0o755
        } else {
            0o644
        });
    let path = add_prefix(entry.relative_path(), tree_prefix).into_owned();
    match entry.mode {
        gix_object::tree::EntryMode::Blob | gix_object::tree::EntryMode::BlobExecutable => {
            ar.start_file(path.to_string(), file_opts)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            std::io::copy(&mut entry, ar)?;
        }
        gix_object::tree::EntryMode::Tree | gix_object::tree::EntryMode::Commit => {
            ar.add_directory(path.to_string(), file_opts)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }
        gix_object::tree::EntryMode::Link => {
            use bstr::ByteSlice;
            std::io::copy(&mut entry, buf)?;
            ar.add_symlink(path.to_string(), buf.as_bstr().to_string(), file_opts)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }
    }
    Ok(())
}

#[cfg(any(feature = "tar", feature = "tar_gz"))]
fn append_tar_entry<W: std::io::Write>(
    ar: &mut tar::Builder<W>,
    buf: &mut Vec<u8>,
    mut entry: gix_worktree_stream::Entry<'_>,
    mtime_seconds_since_epoch: i64,
    opts: &Options,
) -> Result<(), Error> {
    let mut header = tar::Header::new_gnu();
    header.set_mtime(mtime_seconds_since_epoch as u64);
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
    Ok(())
}

#[cfg(any(feature = "tar", feature = "tar_gz"))]
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

#[cfg(any(feature = "tar", feature = "tar_gz"))]
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
