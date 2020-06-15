use crate::{
    loose::{Db, HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use git_object as object;
use hex::ToHex;
use object::parsed;
use quick_error::quick_error;
use smallvec::SmallVec;
use std::{
    fs::File, io::Cursor, io::Read, os::unix::fs::MetadataExt, path::PathBuf, str, str::FromStr,
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decompress(err: zlib::Error) {
            display("decompression of object data failed")
            from()
            cause(err)
        }
        DecompressFile(err: zlib::Error, path: PathBuf) {
            display("decompression of loose object at '{}' failed", path.display())
            cause(err)
        }
        ParseTag(err: parsed::Error) {
            display("Could not parse tag object")
            from()
            cause(err)
        }
        ObjectHeader(err: object::Error) {
            display("Could not parse object kind")
            from()
            cause(err)
        }
        InvalidHeader(msg: &'static str) {
            display("{}", msg)
        }
        ParseUsize(number: String, err: std::num::ParseIntError) {
            display("Number '{}' could not be parsed", number)
            cause(err)
        }
        Io(err: std::io::Error, action: &'static str, path: PathBuf) {
            display("Could not {} file at '{}'", action, path.display())
            cause(err)
        }
    }
}

pub struct Object {
    pub kind: object::Kind,
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
            object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
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
                    object::Kind::Tag => parsed::Object::Tag(parsed::Tag::from_bytes(bytes)?),
                    _ => unimplemented!(),
                }
            }
            object::Kind::Blob => unimplemented!(),
        })
    }
}

pub fn parse_header(input: &[u8]) -> Result<(object::Kind, usize, usize), Error> {
    let header_end = input
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| Error::InvalidHeader("Did not find 0 byte in header"))?;
    let header = &input[..header_end];
    let mut split = header.split(|&b| b == b' ');
    match (split.next(), split.next()) {
        (Some(kind), Some(size)) => Ok((
            object::Kind::from_bytes(kind)?,
            {
                let size = str::from_utf8(size).map_err(|_| {
                    parsed::Error::ParseError(
                        "Object size was not valid UTF-8 or ascii for that matter",
                        size.to_owned(),
                    )
                })?;
                usize::from_str(size).map_err(|e| Error::ParseUsize(size.to_owned(), e))?
            },
            header_end + 1, // account for 0 byte
        )),
        _ => Err(Error::InvalidHeader("Expected '<type> <size>'")),
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
    pub fn find(&self, id: &object::Id) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut deflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream =
                File::open(&path).map_err(|e| Error::Io(e, "open", path.to_owned()))?;
            let bytes_read = istream
                .read(&mut compressed[..])
                .map_err(|e| Error::Io(e, "read", path.to_owned()))?;
            let mut out = Cursor::new(&mut decompressed[..]);

            (
                deflate
                    .once(&compressed[..bytes_read], &mut out)
                    .map_err(|e| Error::DecompressFile(e, path.to_owned()))?,
                bytes_read,
                istream,
            )
        };

        let (kind, size, header_size) = parse_header(&decompressed[..consumed_out])?;

        let decompressed = SmallVec::from_buf(decompressed);
        let mut compressed = SmallVec::from_buf(compressed);

        let path = match kind {
            object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
                let fsize = input_stream
                    .metadata()
                    .map_err(|e| Error::Io(e, "read metadata", path.to_owned()))?
                    .size();
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
                    input_stream
                        .read_exact(&mut compressed[bytes_read..])
                        .map_err(|e| Error::Io(e, "read", path.to_owned()))?;
                    None
                }
            }
            object::Kind::Blob => Some(path), // we will open the file again when needed. Maybe we can load small sized objects anyway
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
