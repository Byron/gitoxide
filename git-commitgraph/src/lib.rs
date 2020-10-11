#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod file;
pub mod graph;

pub use graph::Graph;

pub const GENERATION_NUMBER_INFINITY: u32 = 0xffff_ffff;
pub const GENERATION_NUMBER_MAX: u32 = 0x3fff_ffff;

/// The maximum number of commits that can be stored in a commit graph.
pub const MAX_COMMITS: u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1;

// TODO: pub type ImpossibleVariantError = !;
#[derive(Debug)]
pub struct ImpossibleVariantError;

impl std::error::Error for ImpossibleVariantError {}

impl std::fmt::Display for ImpossibleVariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("this enum was constructed with an invalid variant")
    }
}
