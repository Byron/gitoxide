pub mod ignore;

mod attribute;
pub use attribute::{Error, Iter, Kind, Lines};

pub fn ignore(buf: &[u8]) -> ignore::Lines<'_> {
    ignore::Lines::new(buf)
}
