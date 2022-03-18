pub mod ignore;

pub fn ignore(buf: &[u8]) -> ignore::Iter<'_> {
    ignore::Iter::new(buf)
}
