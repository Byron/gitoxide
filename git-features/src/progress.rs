//! Various `prodash` types along with various utilities for comfort.
use std::io;

pub use prodash::{
    messages::MessageLevel,
    progress::{Discard, DoOrDiscard, Either, Step, StepShared, ThroughputOnDrop, UNKNOWN},
    unit, Progress, Unit,
};

/// A unit for displaying bytes with throughput and progress percentage.
pub fn bytes() -> Option<Unit> {
    Some(unit::dynamic_and_mode(
        unit::Bytes,
        unit::display::Mode::with_throughput().and_percentage(),
    ))
}

/// A unit for displaying human readable numbers with throughput and progress percentage.
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

/// A predefined unit for displaying a multi-step progress
pub fn steps() -> Option<Unit> {
    Some(unit::dynamic(unit::Range::new("steps")))
}

/// A structure passing every [`read`][std::io::Read::read()] call through to the contained Progress instance using [`inc_by(bytes_read)`][Progress::inc_by()].
pub struct Read<T, P> {
    /// The implementor of [`std::io::Read`] to which progress is added
    pub inner: T,
    /// The progress instance receiving progress information on each invocation of `reader`
    pub progress: P,
}

impl<T, P> io::Read for Read<T, P>
where
    T: io::Read,
    P: Progress,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.progress.inc_by(bytes_read);
        Ok(bytes_read)
    }
}

impl<T, P> io::BufRead for Read<T, P>
where
    T: io::BufRead,
    P: Progress,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

/// A structure passing every [`write`][std::io::Write::write()] call through to the contained Progress instance using [`inc_by(bytes_written)`][Progress::inc_by()].
///
/// This is particularly useful if the final size of the bytes to write is known or can be estimated precisely enough.
pub struct Write<T, P> {
    /// The implementor of [`std::io::Write`] to which progress is added
    pub inner: T,
    /// The progress instance receiving progress information on each invocation of `reader`
    pub progress: P,
}

impl<T, P> io::Write for Write<T, P>
where
    T: io::Write,
    P: Progress,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.progress.inc_by(written);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
