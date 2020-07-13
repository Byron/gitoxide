use git_object as object;
use quick_error::quick_error;

use crate::{
    loose::db::serde::parse_header,
    loose::{Object, HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use smallvec::SmallVec;
use std::{fs, io::Cursor, io::Read, path::PathBuf};

pub(crate) mod serde;

pub struct Db {
    pub path: PathBuf,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
            display("Could not {} data at '{}'", action, path.display())
            cause(err)
        }
    }
}

fn sha1_path(id: &object::Id, mut root: PathBuf) -> PathBuf {
    let mut buf = [0u8; 40];
    id.encode_to_40_bytes_slice(&mut buf)
        .expect("no failure as everything is preset by now");
    let buf = std::str::from_utf8(&buf).expect("ascii only in hex");
    root.push(&buf[..2]);
    root.push(&buf[2..]);
    root
}

pub mod iter {
    use crate::loose::Db;
    use git_object as object;
    use quick_error::quick_error;
    use walkdir::WalkDir;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            WalkDir(err: walkdir::Error) {
                cause(err)
            }
        }
    }

    impl Db {
        pub fn iter(&self) -> impl Iterator<Item = Result<object::Id, Error>> {
            use std::path::Component::Normal;
            // TODO: Put this behind a feature flag in git-features and allow iterting with jwalk
            WalkDir::new(&self.path)
                .min_depth(2)
                .max_depth(3)
                .follow_links(false)
                .into_iter()
                .filter_map(|res| {
                    let mut is_valid_path = false;
                    let e = res.map_err(Error::WalkDir).map(|e| {
                        let p = e.path();
                        let (c1, c2) = p.components().fold((None, None), |(_c1, c2), cn| (c2, Some(cn)));
                        if let (Some(Normal(c1)), Some(Normal(c2))) = (c1, c2) {
                            if c1.len() == 2 && c2.len() == 38 {
                                if let (Some(c1), Some(c2)) = (c1.to_str(), c2.to_str()) {
                                    let mut buf = [0u8; 40];
                                    {
                                        let (first_byte, rest) = buf.split_at_mut(2);
                                        first_byte.copy_from_slice(c1.as_bytes());
                                        rest.copy_from_slice(c2.as_bytes());
                                    }
                                    if let Ok(b) = object::Id::from_hex(&buf[..]) {
                                        is_valid_path = true;
                                        return b;
                                    }
                                }
                            }
                        }
                        object::Id::null()
                    });
                    if is_valid_path {
                        Some(e)
                    } else {
                        None
                    }
                })
        }
    }
}

impl Db {
    pub fn at(path: impl Into<PathBuf>) -> Db {
        Db { path: path.into() }
    }

    const OPEN_ACTION: &'static str = "open";

    pub fn locate(&self, id: &object::Id) -> Option<Result<Object, Error>> {
        match self.locate_inner(id) {
            Ok(obj) => Some(Ok(obj)),
            Err(err) => match err {
                Error::Io(err, action, path) => {
                    if action == Self::OPEN_ACTION {
                        None
                    } else {
                        Some(Err(Error::Io(err, action, path)))
                    }
                }
                err => Some(Err(err)),
            },
        }
    }

    fn locate_inner(&self, id: &object::Id) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut inflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream = fs::File::open(&path).map_err(|e| Error::Io(e, Self::OPEN_ACTION, path.to_owned()))?;
            let bytes_read = istream
                .read(&mut compressed[..])
                .map_err(|e| Error::Io(e, "read", path.to_owned()))?;
            let mut out = Cursor::new(&mut decompressed[..]);

            (
                inflate
                    .once(&compressed[..bytes_read], &mut out, true)
                    .map_err(|e| Error::DecompressFile(e, path.to_owned()))?,
                bytes_read,
                istream,
            )
        };

        let (kind, size, header_size) = parse_header(&decompressed[..consumed_out])?;
        let mut decompressed = SmallVec::from_buf(decompressed);
        decompressed.resize(consumed_out, 0);

        let (compressed, path) = if inflate.is_done {
            (SmallVec::default(), None)
        } else {
            match kind {
                object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
                    let mut compressed = SmallVec::from_buf(compressed);
                    // Read small objects right away and store them in memory while we
                    // have a data handle available and 'hot'. Note that we don't decompress yet!
                    let file_size = input_stream
                        .metadata()
                        .map_err(|e| Error::Io(e, "read metadata", path.to_owned()))?
                        .len();
                    assert!(file_size <= ::std::usize::MAX as u64);
                    let file_size = file_size as usize;
                    if bytes_read == file_size {
                        (compressed, None)
                    } else {
                        let cap = compressed.capacity();
                        if cap < file_size {
                            compressed.reserve_exact(file_size - cap);
                            debug_assert!(file_size == compressed.capacity());
                        }

                        compressed.resize(file_size, 0);
                        input_stream
                            .read_exact(&mut compressed[bytes_read..])
                            .map_err(|e| Error::Io(e, "read", path.to_owned()))?;
                        (compressed, None)
                    }
                }
                object::Kind::Blob => (SmallVec::default(), Some(path)), // we will open the data again when needed. Maybe we can load small sized objects anyway
            }
        };

        Ok(Object {
            kind,
            size,
            decompressed_data: decompressed,
            compressed_data: compressed,
            header_size,
            path,
            decompression_complete: inflate.is_done,
        })
    }
}
