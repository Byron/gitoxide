use failure::{Error, ResultExt};

use byteorder::{BigEndian, ByteOrder};
use std::{mem::size_of, path::Path};
use filebuffer::FileBuffer;

use object::SHA1_SIZE;

const N32_SIZE: usize = size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Kind {
    V2,
    V3,
}

pub struct File {
    data: FileBuffer,
    kind: Kind,
    size: u32,
}

impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn at(path: &Path) -> Result<Self, Error> {
        let data = FileBuffer::open(path)
            .with_context(|_| format!("Could not map pack file at '{}'", path.display()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            bail!(
                "Pack file of size {} is too small for even an empty pack",
                pack_len
            );
        }
        let mut ofs = 0;
        if &data[ofs..ofs + N32_SIZE] != b"PACK" {
            bail!("Pack file type not recognized")
        }
        ofs += N32_SIZE;
        let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
            2 => Kind::V2,
            3 => Kind::V3,
            v @ _ => bail!("Unknown pack version: {}", v),
        };
        ofs += N32_SIZE;
        let size = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

        Ok(File { data, kind, size })
    }
}
