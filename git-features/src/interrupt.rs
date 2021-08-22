//! Utilities to cause interruptions in common traits, like Read/Write and Iterator.
use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
};

/// A wrapper for an inner iterator which will check for interruptions on each iteration.
pub struct Iter<'a, I, EFN> {
    /// The actual iterator to yield elements from.
    pub inner: I,
    make_err: Option<EFN>,
    should_interrupt: &'a AtomicBool,
}

impl<'a, I, EFN, E> Iter<'a, I, EFN>
where
    I: Iterator,
    EFN: FnOnce() -> E,
{
    /// Create a new iterator over `inner` which checks for interruptions on each iteration and cals `make_err()` to
    /// signal an interruption happened, causing no further items to be iterated from that point on.
    pub fn new(inner: I, make_err: EFN, should_interrupt: &'a AtomicBool) -> Self {
        Iter {
            inner,
            make_err: Some(make_err),
            should_interrupt,
        }
    }
}

impl<'a, I, EFN, E> Iterator for Iter<'a, I, EFN>
where
    I: Iterator,
    EFN: FnOnce() -> E,
{
    type Item = Result<I::Item, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.make_err.as_ref()?;
        if self.should_interrupt.load(Ordering::Relaxed) {
            return Some(Err(self.make_err.take().expect("no bug")()));
        }
        match self.inner.next() {
            Some(next) => Some(Ok(next)),
            None => {
                self.make_err = None;
                None
            }
        }
    }
}

/// A wrapper for implementors of [`std::io::Read`] or [`std::io::BufRead`] with interrupt support.
///
/// It fails a [read][`std::io::Read::read`] while an interrupt was requested.
pub struct Read<'a, R> {
    /// The actual implementor of [`std::io::Read`] to which interrupt support will be added.
    pub inner: R,
    /// The flag to trigger interruption
    pub should_interrupt: &'a AtomicBool,
}

impl<'a, R> io::Read for Read<'a, R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.should_interrupt.load(Ordering::Relaxed) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        self.inner.read(buf)
    }
}

impl<'a, R> io::BufRead for Read<'a, R>
where
    R: io::BufRead,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}
