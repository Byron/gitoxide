use bstr::BString;

use crate::{
    extension::{FsMonitor, Signature},
    util::{read_u32, read_u64, split_at_byte_exclusive},
};

#[derive(Clone)]
pub enum Token {
    V1 { nanos_since_1970: u64 },
    V2 { token: BString },
}

pub const SIGNATURE: Signature = *b"FSMN";

pub fn decode(data: &[u8]) -> Option<FsMonitor> {
    let (version, data) = read_u32(data)?;
    let (token, data) = match version {
        1 => {
            let (nanos_since_1970, data) = read_u64(data)?;
            (Token::V1 { nanos_since_1970 }, data)
        }
        2 => {
            let (token, data) = split_at_byte_exclusive(data, 0)?;
            (Token::V2 { token: token.into() }, data)
        }
        _ => return None,
    };

    let (ewah_size, data) = read_u32(data)?;
    let (entry_dirty, data) = gix_bitmap::ewah::decode(&data[..ewah_size as usize]).ok()?;

    if !data.is_empty() {
        return None;
    }

    FsMonitor { token, entry_dirty }.into()
}
