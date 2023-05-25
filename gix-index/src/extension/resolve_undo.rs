use bstr::BString;
use gix_hash::ObjectId;

use crate::{
    extension::Signature,
    util::{split_at_byte_exclusive, split_at_pos},
};

pub type Paths = Vec<ResolvePath>;

#[allow(dead_code)]
#[derive(Clone)]
pub struct ResolvePath {
    /// relative to the root of the repository, or what would be stored in the index
    name: BString,

    /// 0 = ancestor/common, 1 = ours, 2 = theirs
    stages: [Option<Stage>; 3],
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Stage {
    mode: u32,
    id: ObjectId,
}

pub const SIGNATURE: Signature = *b"REUC";

pub fn decode(mut data: &[u8], object_hash: gix_hash::Kind) -> Option<Paths> {
    let hash_len = object_hash.len_in_bytes();
    let mut out = Vec::new();

    while !data.is_empty() {
        let (path, rest) = split_at_byte_exclusive(data, 0)?;
        data = rest;

        let mut modes = [0u32; 3];
        for mode in &mut modes {
            let (mode_ascii, rest) = split_at_byte_exclusive(data, 0)?;
            data = rest;
            *mode = u32::from_str_radix(std::str::from_utf8(mode_ascii).ok()?, 8).ok()?;
        }

        let mut stages = [None, None, None];
        for (mode, stage) in modes.iter().zip(stages.iter_mut()) {
            if *mode == 0 {
                continue;
            }
            let (hash, rest) = split_at_pos(data, hash_len)?;
            data = rest;
            *stage = Some(Stage {
                mode: *mode,
                id: ObjectId::from(hash),
            });
        }

        out.push(ResolvePath {
            name: path.into(),
            stages,
        });
    }
    out.into()
}
