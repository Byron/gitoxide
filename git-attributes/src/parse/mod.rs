///
pub mod ignore;

mod attribute;
pub use attribute::{Error, Iter, Kind, Lines};

/// Parse git ignore patterns, line by line, from `bytes`.
pub fn ignore(bytes: &[u8]) -> ignore::Lines<'_> {
    ignore::Lines::new(bytes)
}
