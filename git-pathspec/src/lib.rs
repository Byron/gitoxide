#![forbid(unsafe_code, rust_2018_idioms)]

use bitflags::bitflags;
pub use parse::Pattern;

mod parse;

bitflags! {
    pub struct MagicSignature: u32 {
        const TOP = 1 << 0;
        const LITERAL = 1 << 1;
        const ICASE = 1 << 2;
        const GLOB = 1 << 3;
        const ATTR = 1 << 4;
        const EXCLUDE = 1 << 5;
    }
}

pub fn parse(input: &[u8]) -> Pattern {
    Pattern::from_bytes(input)
}
