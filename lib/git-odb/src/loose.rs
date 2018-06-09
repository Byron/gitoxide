use object::{Id, Kind};

use walkdir::WalkDir;
use failure::{Error, ResultExt};
use hex::{FromHex, ToHex};
use smallvec::SmallVec;
use std::{fs::File, io::{Cursor, Read}, path::PathBuf};
use std::os::unix::fs::MetadataExt;
use deflate;
use object::parsed;
use miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;

const HEADER_READ_COMPRESSED_BYTES: usize = 256;
const HEADER_READ_UNCOMPRESSED_BYTES: usize = 256;

pub struct Db {
    pub path: PathBuf,
}

pub struct Object {
    pub kind: Kind,
    pub size: usize,
    decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    end_of_consumed_compressed_bytes: usize,
    end_of_decompressed_bytes: usize,
    header_size: usize,
    path: Option<PathBuf>,
    deflate: deflate::State,
}

impl Object {
    pub fn parsed(&mut self) -> Result<parsed::Object, Error> {
        Ok(match self.kind {
            Kind::Tag | Kind::Commit | Kind::Tree => {
                if !self.deflate.is_done {
                    let total_size = self.header_size + self.size;
                    let cap = self.decompressed_data.capacity();
                    if cap < total_size {
                        self.decompressed_data
                            .reserve_exact(total_size - cap);
                    }
                    unsafe {
                        debug_assert!(self.decompressed_data.capacity() >= total_size);
                        self.decompressed_data.set_len(total_size);
                    }
                    println!("out capacity: {}", self.decompressed_data.len());
                    let mut cursor = Cursor::new(&mut self.decompressed_data[..]);
                    self.deflate = Default::default();
                    let (consumed_in, consumed_out) = self.deflate.to_end(
                        &self.compressed_data[..],
                        &mut cursor,
                        TINFL_FLAG_PARSE_ZLIB_HEADER,
                    )?;
                    debug_assert!(self.deflate.is_done);
                    self.end_of_decompressed_bytes = consumed_out;
                    self.end_of_consumed_compressed_bytes = consumed_in;
                    debug_assert!(self.end_of_decompressed_bytes == total_size);
                }
                let bytes = &self.decompressed_data[..self.end_of_decompressed_bytes];
                match self.kind {
                    Kind::Tag => parsed::Object::Tag(parsed::Tag::from_bytes(bytes)?),
                    _ => unimplemented!(),
                }
            }
            Kind::Blob => unimplemented!(),
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
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream = File::open(&path)?;
            let bytes_read = istream.read(&mut compressed[..])?;
            let mut out = Cursor::new(&mut decompressed[..]);

            (
                deflate
                    .once(
                        &compressed[..bytes_read],
                        &mut out,
                        TINFL_FLAG_PARSE_ZLIB_HEADER,
                    )
                    .with_context(|_| {
                        format!("ZIP inflating failed while reading '{}'", path.display())
                    })?,
                bytes_read,
                istream,
            )
        };
        println!("{:?} = stauts", _status);

        let (kind, size, header_size) =
            parse::header(&decompressed[..consumed_out]).with_context(|_| {
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
            Kind::Blob => Some(path),
        };

        Ok(Object {
            kind,
            size,
            decompressed_data: decompressed,
            compressed_data: compressed,
            end_of_consumed_compressed_bytes: consumed_in,
            end_of_decompressed_bytes: consumed_out,
            header_size,
            path,
            deflate,
        })
    }
}

pub mod parse {
    use failure::{err_msg, Error};

    use object;
    use std::str;

    pub fn header(input: &[u8]) -> Result<(object::Kind, usize, usize), Error> {
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
                header_end + 1 // account for 0 byte
            )),
            _ => bail!("expected '<type> <size>'"),
        }
    }
}

pub fn at(path: impl Into<PathBuf>) -> Db {
    Db { path: path.into() }
}
