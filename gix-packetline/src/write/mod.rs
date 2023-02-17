use crate::Writer;

#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
pub(crate) mod async_io;

#[cfg(feature = "blocking-io")]
pub(crate) mod blocking_io;

/// Common methods
impl<T> Writer<T> {
    /// As [`enable_text_mode()`][Writer::enable_text_mode()], but suitable for chaining.
    pub fn text_mode(mut self) -> Self {
        self.enable_text_mode();
        self
    }
    /// As [`enable_binary_mode()`][Writer::enable_binary_mode()], but suitable for chaining.
    pub fn binary_mode(mut self) -> Self {
        self.enable_binary_mode();
        self
    }
}
