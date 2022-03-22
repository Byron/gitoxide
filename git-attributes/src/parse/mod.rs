pub mod ignore;

pub mod attribute;

pub fn ignore(buf: &[u8]) -> ignore::Lines<'_> {
    ignore::Lines::new(buf)
}
