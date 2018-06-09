use object::{Id, Kind};

use std::path::PathBuf;

use walkdir::WalkDir;
use failure::{Error, ResultExt};
use hex::{FromHex, ToHex};
use miniz_oxide::inflate::core::{decompress, DecompressorOxide,
                                 inflate_flags::{TINFL_FLAG_PARSE_ZLIB_HEADER,
                                                 TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF}};
use std::{fs::File, io::{Cursor, Read}};

const HEADER_READ_COMPRESSED_BYTES: usize = 1024;

pub struct Db {
    pub path: PathBuf,
}

pub struct Object {
    pub kind: Kind,
    pub size: usize,
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

        let mut out = [0; HEADER_READ_COMPRESSED_BYTES];
        let (out, read_out) = {
            let mut rbuf = [0; HEADER_READ_COMPRESSED_BYTES];
            let bytes_read = File::open(&path)?.read(&mut rbuf[..])?;
            let mut state = DecompressorOxide::default();
            let mut out = Cursor::new(&mut out[..]);

            let (status, _read_in, read_out) = decompress(
                &mut state,
                &rbuf[..bytes_read],
                &mut out,
                TINFL_FLAG_PARSE_ZLIB_HEADER | TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF,
            );
            use miniz_oxide::inflate::TINFLStatus::*;
            match status {
                Failed | FailedCannotMakeProgress | BadParam | Adler32Mismatch | NeedsMoreInput => {
                    bail!(
                    "Could not decode zip stream for reading header in '{}', zip status was '{:?}'",
                    path.display(),
                    status
                )
                }
                Done | HasMoreOutput => {}
            };
            (out.into_inner(), read_out)
        };

        parse::header(&out[..read_out])
            .map(|(kind, size)| Object { kind, size })
            .with_context(|_| {
                format!(
                    "Invalid header layout at '{}', expected '<type> <size>'",
                    path.display()
                )
            })
            .map_err(Into::into)
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
