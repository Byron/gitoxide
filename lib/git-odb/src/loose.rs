use object::{Id, Kind};

use walkdir::WalkDir;
use failure::{Error, ResultExt};
use hex::{FromHex, ToHex};
use smallvec::SmallVec;
use std::{fs::File, io::{Cursor, Read}, path::PathBuf};
use deflate;
use object::parsed;

const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 512;

pub struct Db {
    pub path: PathBuf,
}

pub struct Object {
    pub kind: Kind,
    pub size: usize,
    uncompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    consumed_compressed_bytes: usize,
    consumed_uncompressed_bytes: usize,
    path: PathBuf,
    deflate: deflate::State,
}

impl Object {
    pub fn parsed(&mut self) -> Result<parsed::Object, Error> {
        Ok(match self.kind {
            Kind::Tag | Kind::Commit => {
                let bytes = self.uncompressed_data.as_slice();
                if !self.deflate.is_done {
                    //                    File::open(&self.path)?.read_to_end()?;
                }
                match self.kind {
                    Kind::Tag => parsed::Object::Tag(parsed::Tag::from_bytes(bytes)?),
                    Kind::Commit => unimplemented!(),
                }
            }
        })
    }
}

impl Db {
    pub fn iter(&self) -> impl Iterator<Item = Result<Id, Error>> {
        use std::path::Component::Normal;
        WalkDir::new(&self.path)
            .min_depth(2)
            .max_depth(3)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| {
                let mut is_valid_path = false;
                let e = e.map_err(Error::from).map(|e| {
                    let p = e.path();
                    let (c1, c2) = p.components()
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

    pub fn find(&self, id: &Id) -> Result<Object, Error> {
        let path = {
            let mut path = self.path.clone();
            let mut buf = String::with_capacity(40);
            id.write_hex(&mut buf)
                .expect("no failure as everything is preset by now");
            path.push(&buf[..2]);
            path.push(&buf[2..]);
            path
        };

        let mut deflate = deflate::State::default();
        let mut uncompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((consumed_in, consumed_out), input_stream) = {
            let mut istream = File::open(&path)?;
            let bytes_read = istream.read(&mut compressed[..])?;
            let mut out = Cursor::new(&mut uncompressed[..]);

            (
                deflate
                    .once(&compressed[..bytes_read], &mut out)
                    .with_context(|_| {
                        format!(
                            "Could not decode zip stream for reading header in '{}'",
                            path.display()
                        )
                    })?,
                istream,
            )
        };

        let (kind, size) = parse::header(&uncompressed[..consumed_out]).with_context(|_| {
            format!(
                "Invalid header layout at '{}', expected '<type> <size>'",
                path.display()
            )
        })?;

        let uncompressed = SmallVec::from_buf(uncompressed);
        let compressed = SmallVec::from_buf(compressed);

        match kind {
            Kind::Tag | Kind::Commit | Kind::Tree => {
                unimplemented!()
            }
            Kind::Blob => {}
        }

        Ok(Object {
            kind,
            size,
            uncompressed_data: uncompressed,
            compressed_data: compressed,
            consumed_compressed_bytes: consumed_in,
            consumed_uncompressed_bytes: consumed_out,
            path,
            deflate,
        })
    }
}

pub mod parse {
    use failure::{err_msg, Error};

    use object;
    use std::str;

    pub fn header(input: &[u8]) -> Result<(object::Kind, usize), Error> {
        let header_end = input
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| err_msg("Invalid header, did not find 0 byte"))?;
        let header = &input[..header_end];
        let mut split = header.split(|&b| b == b' ');
        match (split.next(), split.next()) {
            (Some(kind), Some(size)) => Ok((
                object::Kind::from_bytes(kind)?,
                str::from_utf8(size)?.parse()?,
            )),
            _ => bail!("expected '<type> <size>'"),
        }
    }
}

pub fn at(path: impl Into<PathBuf>) -> Db {
    Db { path: path.into() }
}
