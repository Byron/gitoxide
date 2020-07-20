use crate::pack::data;
use byteorder::{BigEndian, ByteOrder};
use quick_error::quick_error;

pub(crate) const N32_SIZE: usize = std::mem::size_of::<u32>();

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error, path: std::path::PathBuf) {
            display("Could not open pack file at '{}'", path.display())
            source(err)
        }
        Corrupt(msg: String) {
            display("{}", msg)
        }
        UnsupportedVersion(version: u32) {
            display("Unsupported pack version: {}", version)
        }
    }
}

// Return (data::Kind, num_objects_in_pack)
pub fn header(data: &[u8; 12]) -> Result<(data::Kind, u32), Error> {
    let mut ofs = 0;
    if &data[ofs..ofs + b"PACK".len()] != b"PACK" {
        return Err(Error::Corrupt("Pack data type not recognized".into()));
    }
    ofs += N32_SIZE;
    let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
        2 => data::Kind::V2,
        3 => data::Kind::V3,
        v => return Err(Error::UnsupportedVersion(v)),
    };
    ofs += N32_SIZE;
    let num_objects = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

    Ok((kind, num_objects))
}
