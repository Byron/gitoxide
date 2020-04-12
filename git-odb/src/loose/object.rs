use crate::{
    loose::Db,
    loose::{HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    object::Id,
    object::{parsed, Kind},
    zlib,
};
use failure::{err_msg, Error, ResultExt};
use hex::ToHex;
use smallvec::SmallVec;
use std::{fs::File, io::Cursor, io::Read, os::unix::fs::MetadataExt, path::PathBuf, str};

pub struct Object {
    pub kind: Kind,
    pub size: usize,
    decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    header_size: usize,
    _path: Option<PathBuf>,
    is_decompressed: bool,
}

impl Object {
    pub fn parsed(&mut self) -> Result<parsed::Object, Error> {
        Ok(match self.kind {
            Kind::Tag | Kind::Commit | Kind::Tree => {
                if !self.is_decompressed {
                    let total_size = self.header_size + self.size;
                    let cap = self.decompressed_data.capacity();
                    if cap < total_size {
                        self.decompressed_data.reserve_exact(total_size - cap);
                    }
                    unsafe {
                        debug_assert!(self.decompressed_data.capacity() >= total_size);
                        self.decompressed_data.set_len(total_size);
                    }
                    let mut cursor = Cursor::new(&mut self.decompressed_data[..]);
                    // TODO Performance opportunity
                    // here we do a lot of additional work, which could be saved if we
                    // could re-use the previous state. This doesn't work for some reason.
                    let mut deflate = zlib::Inflate::default();
                    deflate.all_till_done(&self.compressed_data[..], &mut cursor)?;
                    self.is_decompressed = deflate.is_done;
                    debug_assert!(deflate.is_done);
                    self.compressed_data = Default::default();
                }
                let bytes = &self.decompressed_data[self.header_size..];
                match self.kind {
                    Kind::Tag => parsed::Object::Tag(parsed::Tag::from_bytes(bytes)?),
                    _ => unimplemented!(),
                }
            }
            Kind::Blob => unimplemented!(),
        })
    }
}

pub fn parse_header(input: &[u8]) -> Result<(Kind, usize, usize), Error> {
    let header_end = input
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| err_msg("Invalid header, did not find 0 byte"))?;
    let header = &input[..header_end];
    let mut split = header.split(|&b| b == b' ');
    match (split.next(), split.next()) {
        (Some(kind), Some(size)) => Ok((
            Kind::from_bytes(kind)?,
            str::from_utf8(size)?.parse()?,
            header_end + 1, // account for 0 byte
        )),
        _ => bail!("expected '<type> <size>'"),
    }
}

fn sha1_path(id: &[u8; 20], mut root: PathBuf) -> PathBuf {
    let mut buf = String::with_capacity(40);
    id.write_hex(&mut buf)
        .expect("no failure as everything is preset by now");
    root.push(&buf[..2]);
    root.push(&buf[2..]);
    root
}

impl Db {
    pub fn find(&self, id: &Id) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut deflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream = File::open(&path)?;
            let bytes_read = istream.read(&mut compressed[..])?;
            let mut out = Cursor::new(&mut decompressed[..]);

            (
                deflate
                    .once(&compressed[..bytes_read], &mut out)
                    .with_context(|_| {
                        format!("ZIP inflating failed while reading '{}'", path.display())
                    })?,
                bytes_read,
                istream,
            )
        };

        let (kind, size, header_size) =
            parse_header(&decompressed[..consumed_out]).with_context(|_| {
                format!(
                    "Invalid header layout at '{}', expected '<type> <size>'",
                    path.display()
                )
            })?;

        let decompressed = SmallVec::from_buf(decompressed);
        let mut compressed = SmallVec::from_buf(compressed);

        let path = match kind {
            Kind::Tag | Kind::Commit | Kind::Tree => {
                let fsize = input_stream.metadata()?.size();
                assert!(fsize <= ::std::usize::MAX as u64);
                let fsize = fsize as usize;
                if bytes_read == fsize {
                    None
                } else {
                    let cap = compressed.capacity();
                    if cap < fsize {
                        compressed.reserve_exact(fsize - cap);
                        debug_assert!(fsize == compressed.capacity());
                    }
                    unsafe {
                        compressed.set_len(fsize);
                    }
                    input_stream.read_exact(&mut compressed[bytes_read..])?;
                    None
                }
            }
            Kind::Blob => Some(path), // we will open the file again when needed. Maybe we can load small sized objects anyway
        };

        Ok(Object {
            kind,
            size,
            decompressed_data: decompressed,
            compressed_data: compressed,
            header_size,
            _path: path,
            is_decompressed: deflate.is_done,
        })
    }
}
