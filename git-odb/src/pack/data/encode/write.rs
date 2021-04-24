use crate::pack::data::encode;

/// An implementation of [`Iterator`] to write [encoded entries][`encode::Entry`] to an inner implementation each time
/// `next()` is called.
pub struct Entries<I, W> {
    /// An iterator for input [`encode::Entry`] instances
    pub input: I,
    /// A way of writing encoded bytes.
    pub output: W,
}

impl<I, W, E> Entries<I, W>
where
    I: Iterator<Item = Result<Vec<encode::Entry>, encode::entries::Error<E>>>,
    W: std::io::Write,
    E: std::error::Error + 'static,
{
    /// Create a new instance from an `input` iterator and an `output` writer.
    pub fn new(input: I, output: W) -> Self {
        Entries { input, output }
    }
}

impl<I, W, E> Iterator for Entries<I, W>
where
    I: Iterator<Item = Result<Vec<encode::Entry>, encode::entries::Error<E>>>,
    W: std::io::Write,
    E: std::error::Error + 'static,
{
    /// The amount of bytes written to `out` if `Ok` or the error `E` received from the input.
    type Item = Result<u64, encode::entries::Error<E>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("next()")
    }
}
