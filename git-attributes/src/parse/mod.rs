pub mod ignore;

pub mod attributes {}

pub fn ignore(buf: &[u8]) -> ignore::Iter<'_> {
    ignore::Iter::new(buf)
}
