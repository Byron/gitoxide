use crate::stream::{Entry, Error};
use crate::{stream, Stream};
use gix_object::bstr::{BStr, BString};
use std::io::{ErrorKind, Read, Write};

impl Stream {
    /// Access the next entry of the stream or `None` if there is nothing more to read.
    pub fn next_entry(&mut self) -> Result<Option<Entry<'_>>, Error> {
        assert!(
            self.path_buf.is_some(),
            "BUG: must consume and drop entry before getting the next one"
        );
        self.extra_entries.take();
        let res = read_entry_info(
            &mut self.read,
            self.path_buf.as_mut().expect("set while producing an entry"),
        );
        match res {
            Ok((remaining, mode, id)) => {
                if let Some(err) = self.err.lock().take() {
                    return Err(err);
                }
                Ok(Some(Entry {
                    path_buf: self.path_buf.take(),
                    parent: self,
                    id,
                    mode,
                    remaining,
                }))
            }
            Err(err) => {
                if let Some(err) = self.err.lock().take() {
                    return Err(err);
                }
                // unexpected EOF means the other side dropped. We handled potential errors already.
                if err.kind() == ErrorKind::UnexpectedEof {
                    return Ok(None);
                }
                Err(err.into())
            }
        }
    }
}

// Format: [usize-LE][usize-LE][byte][byte][hash][relative_path_bytes][object_stream]
// Note that stream_len can be usize::MAX to indicate the stream size is unknown
fn read_entry_info(
    read: &mut stream::utils::Read,
    path_buf: &mut BString,
) -> std::io::Result<(Option<usize>, gix_object::tree::EntryMode, gix_hash::ObjectId)> {
    let mut buf = [0; std::mem::size_of::<usize>() * 2 + 2];

    read.read_exact(&mut buf)?;
    let (path_len, rest) = buf.split_at(std::mem::size_of::<usize>());
    let (stream_len, bytes) = rest.split_at(std::mem::size_of::<usize>());
    let path_len = usize::from_le_bytes(path_len.try_into().expect("valid"));
    let stream_size = usize::from_le_bytes(stream_len.try_into().expect("valid"));
    let mode = byte_to_mode(bytes[0]);
    let hash_kind = byte_to_hash(bytes[1]);

    let mut hash = hash_kind.null();
    read.read_exact(hash.as_mut_slice())?;

    clear_and_set_len(path_buf, path_len);
    read.read_exact(path_buf)?;

    Ok(((stream_size != usize::MAX).then_some(stream_size), mode, hash))
}

/// This function must match the read-count of `read_entry_info` for max efficiency.
pub(crate) fn write_entry_header_and_path(
    path: &BStr,
    oid: &gix_hash::oid,
    mode: gix_object::tree::EntryMode,
    stream_len: Option<usize>,
    out: &mut gix_features::io::pipe::Writer,
) -> std::io::Result<()> {
    let mut buf = [0u8; std::mem::size_of::<usize>() * 2 + 2];
    let (path_len_buf, rest) = buf.split_at_mut(std::mem::size_of::<usize>());
    let (stream_len_buf, bytes) = rest.split_at_mut(std::mem::size_of::<usize>());

    path_len_buf.copy_from_slice(&path.len().to_le_bytes());
    stream_len_buf.copy_from_slice(&stream_len.unwrap_or(usize::MAX).to_le_bytes());
    bytes[0] = mode_to_byte(mode);
    bytes[1] = hash_to_byte(oid.kind());

    // We know how `out` works in a pipe writer, it's always writing everything.
    #[allow(clippy::unused_io_amount)]
    {
        out.write(&buf)?;
        out.write(oid.as_bytes())?;
        out.write(path)?;
    }
    Ok(())
}

/// This writes everything in `input` in such way that the receiver knows exactly how much to read.
/// The format is similar to the packetline format, but in binary.
pub(crate) fn write_stream(
    buf: &mut Vec<u8>,
    mut input: impl std::io::Read,
    out: &mut gix_features::io::pipe::Writer,
) -> std::io::Result<()> {
    const BUF_LEN: usize = u16::MAX as usize;
    clear_and_set_len(buf, BUF_LEN);

    // We know how `out` works in a pipe writer, it's always writing everything.
    #[allow(clippy::unused_io_amount)]
    loop {
        match input.read(buf) {
            Ok(0) => {
                // terminator
                out.write(&0_u16.to_le_bytes())?;
                break;
            }
            Ok(n) => {
                out.write(&(n as u16).to_le_bytes())?;
                out.write(&buf[..n])?;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn byte_to_hash(b: u8) -> gix_hash::Kind {
    match b {
        0 => gix_hash::Kind::Sha1,
        _ => unreachable!("BUG: we control the protocol"),
    }
}

fn byte_to_mode(b: u8) -> gix_object::tree::EntryMode {
    use gix_object::tree::EntryMode::*;
    match b {
        0 => Tree,
        1 => Blob,
        2 => BlobExecutable,
        3 => Link,
        4 => Commit,
        _ => unreachable!("BUG: we control the protocol"),
    }
}

fn hash_to_byte(h: gix_hash::Kind) -> u8 {
    match h {
        gix_hash::Kind::Sha1 => 0,
    }
}

fn mode_to_byte(m: gix_object::tree::EntryMode) -> u8 {
    use gix_object::tree::EntryMode::*;
    match m {
        Tree => 0,
        Blob => 1,
        BlobExecutable => 2,
        Link => 3,
        Commit => 4,
    }
}

fn clear_and_set_len(buf: &mut Vec<u8>, len: usize) {
    buf.clear();
    if buf.capacity() < len {
        buf.reserve(len);
        assert!(buf.capacity() >= len, "{} >= {}", buf.capacity(), len);
    }
    // SAFETY: we just assured that `buf` has the right capacity to hold `cap`
    #[allow(unsafe_code)]
    unsafe {
        buf.set_len(len);
    }
}
