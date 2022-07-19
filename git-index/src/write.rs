use bstr::{BString, ByteVec};

use crate::Version;

pub fn header(version: Version, num_entries: i32) -> BString {
    let mut header = BString::from("");

    const SIGNATURE: &[u8; 4] = b"DIRC";

    let version = match version {
        Version::V2 => 2_i32.to_be_bytes(),
        Version::V3 => 3_i32.to_be_bytes(),
        Version::V4 => 4_i32.to_be_bytes(),
    };

    header.push_str(SIGNATURE);
    header.push_str(version);
    header.push_str(num_entries.to_be_bytes());

    header
}
