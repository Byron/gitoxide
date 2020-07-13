use crate::pack::index::{Error, File, Kind, FAN_LEN};
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use std::{convert::TryFrom, mem::size_of, path::Path};

const N32_SIZE: usize = size_of::<u32>();
const V2_SIGNATURE: &[u8] = b"\xfftOc";
const FOOTER_SIZE: usize = SHA1_SIZE * 2;

/// Instantiation
impl File {
    pub fn at(path: impl AsRef<Path>) -> Result<File, Error> {
        Self::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io(e, path.to_owned()))?;
        let idx_len = data.len();
        if idx_len < FAN_LEN * N32_SIZE + FOOTER_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack index of size {} is too small for even an empty index",
                idx_len
            )));
        }
        let (kind, version, fan, num_objects) = {
            let (kind, d) = {
                let (sig, d) = data.split_at(V2_SIGNATURE.len());
                if sig == V2_SIGNATURE {
                    (Kind::V2, d)
                } else {
                    (Kind::V1, &data[..])
                }
            };
            let (version, d) = {
                let (mut v, mut d) = (1, d);
                if let Kind::V2 = kind {
                    let (vd, dr) = d.split_at(N32_SIZE);
                    d = dr;
                    v = BigEndian::read_u32(vd);
                    if v != 2 {
                        return Err(Error::UnsupportedVersion(v));
                    }
                }
                (v, d)
            };
            let (fan, bytes_read) = read_fan(d);
            let (_, _d) = d.split_at(bytes_read);
            let num_objects = fan[FAN_LEN - 1];

            (kind, version, fan, num_objects)
        };
        Ok(File {
            data,
            kind,
            num_objects,
            version,
            fan,
        })
    }
}

fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
    let mut fan = [0; FAN_LEN];
    for (c, f) in d.chunks(N32_SIZE).zip(fan.iter_mut()) {
        *f = BigEndian::read_u32(c);
    }
    (fan, FAN_LEN * N32_SIZE)
}
