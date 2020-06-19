use git_object as object;
use quick_error::quick_error;

use crate::{
    loose::{Object, HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use hex::FromHex;
use smallvec::SmallVec;
use std::{fs, io::Cursor, io::Read, os::unix::fs::MetadataExt, path::PathBuf};
use walkdir::WalkDir;

pub struct Db {
    pub path: PathBuf,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        WalkDir(err: walkdir::Error) {
            cause(err)
        }
        DecompressFile(err: zlib::Error, path: PathBuf) {
            display("decompression of loose object at '{}' failed", path.display())
            cause(err)
        }
        ParseIntegerError(msg: &'static str, number: Vec<u8>, err: btoi::ParseIntegerError) {
            display("{}: {:?}", msg, std::str::from_utf8(number))
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
        Io(err: std::io::Error, action: &'static str, path: PathBuf) {
            display("Could not {} file at '{}'", action, path.display())
            cause(err)
        }
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
            btoi::btoi(size).map_err(|e| {
                Error::ParseIntegerError(
                    "Object size in header could not be parsed",
                    size.to_owned(),
                    e,
                )
            })?,
            header_end + 1, // account for 0 byte
        )),
        _ => Err(Error::InvalidHeader("Expected '<type> <size>'")),
    }
}

fn sha1_path(id: &[u8; 20], mut root: PathBuf) -> PathBuf {
    let mut buf = [0u8; 40];
    hex::encode_to_slice(id, &mut buf).expect("no failure as everything is preset by now");
    let buf = std::str::from_utf8(&buf).expect("ascii only in hex");
    root.push(&buf[..2]);
    root.push(&buf[2..]);
    root
}

impl Db {
    pub fn at(path: impl Into<PathBuf>) -> Db {
        Db { path: path.into() }
    }
    pub fn iter(&self) -> impl Iterator<Item = Result<object::Id, Error>> {
        use std::path::Component::Normal;
        WalkDir::new(&self.path)
            .min_depth(2)
            .max_depth(3)
            .follow_links(false)
            .into_iter()
            .filter_map(|res| {
                let mut is_valid_path = false;
                let e = res.map_err(Error::WalkDir).map(|e| {
                    let p = e.path();
                    let (c1, c2) = p
                        .components()
                        .fold((None, None), |(_c1, c2), cn| (c2, Some(cn)));
                    if let (Some(Normal(c1)), Some(Normal(c2))) = (c1, c2) {
                        if c1.len() == 2 && c2.len() == 38 {
                            if let (Some(c1), Some(c2)) = (c1.to_str(), c2.to_str()) {
                                let mut buf = [0u8; 40];
                                {
                                    let (first_byte, rest) = buf.split_at_mut(2);
                                    first_byte.copy_from_slice(c1.as_bytes());
                                    rest.copy_from_slice(c2.as_bytes());
                                }
                                if let Ok(b) = <[u8; 20]>::from_hex(&buf[..]) {
                                    is_valid_path = true;
                                    return b;
                                }
                            }
                        }
                    }
                    [0u8; 20]
                });
                if is_valid_path {
                    Some(e)
                } else {
                    None
                }
            })
    }

    pub fn locate(&self, id: &object::Id) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut deflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream =
                fs::File::open(&path).map_err(|e| Error::Io(e, "open", path.to_owned()))?;
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

        let mut decompressed = SmallVec::from_buf(decompressed);
        decompressed.resize(consumed_out, 0);
        let mut compressed = SmallVec::from_buf(compressed);

        let path = match kind {
            object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
                // Read small objects right away and store them in memory while we
                // have a file handle available and 'hot'. Note that we don't decompress yet!
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

                    compressed.resize(fsize, 0);
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
            path,
            decompression_complete: deflate.is_done,
        })
    }
}
