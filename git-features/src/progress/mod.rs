use std::io;

pub use prodash::progress::{Discard, DoOrDiscard, Either, ThroughputOnDrop};
pub use prodash::{unit, Progress, Unit};

pub fn bytes() -> Option<Unit> {
    Some(unit::dynamic_and_mode(
        unit::Bytes,
        unit::display::Mode::with_throughput().and_percentage(),
    ))
}

pub fn count(name: &'static str) -> Option<Unit> {
    Some(unit::dynamic_and_mode(
        unit::Human::new(
            {
                let mut f = unit::human::Formatter::new();
                f.with_decimals(1);
                f
            },
            name,
        ),
        unit::display::Mode::with_throughput().and_percentage(),
    ))
}

pub fn steps() -> Option<Unit> {
    Some(unit::dynamic(unit::Range::new("steps")))
}

/// A structure passing every 'read' call through to the contained Progress instance using `inc_by(bytes_read)`.
pub struct Read<R, P> {
    pub reader: R,
    pub progress: P,
}

impl<R, P> io::Read for Read<R, P>
where
    R: io::Read,
    P: Progress,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.progress.inc_by(bytes_read as usize);
        Ok(bytes_read)
    }
}

impl<R, P> io::BufRead for Read<R, P>
where
    R: io::BufRead,
    P: Progress,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}
