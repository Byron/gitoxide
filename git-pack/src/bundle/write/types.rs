use std::io::SeekFrom;
use std::{io, path::PathBuf, sync::Arc};
use tempfile::NamedTempFile;

/// Configuration for [write_to_directory][crate::Bundle::write_to_directory()] or
/// [write_to_directory_eagerly][crate::Bundle::write_to_directory_eagerly()]
#[derive(Debug, Clone)]
pub struct Options {
    /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
    pub thread_limit: Option<usize>,
    /// Determine how much processing to spend on protecting against corruption or recovering from errors.
    pub iteration_mode: crate::data::input::Mode,
    /// The version of pack index to write, should be [`crate::index::Version::default()`]
    pub index_kind: crate::index::Version,
}

impl Default for Options {
    /// Options which favor speed and correctness and write the most commonly supported index file.
    fn default() -> Self {
        Options {
            thread_limit: None,
            iteration_mode: crate::data::input::Mode::Verify,
            index_kind: Default::default(),
        }
    }
}

/// Returned by [write_to_directory][crate::Bundle::write_to_directory()] or
/// [write_to_directory_eagerly][crate::Bundle::write_to_directory_eagerly()]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    /// The successful result of the index write operation
    pub index: crate::index::write::Outcome,
    /// The version of the pack
    pub pack_kind: crate::data::Version,

    /// The path to the pack index file
    pub index_path: Option<PathBuf>,
    /// The path to the pack data file
    pub data_path: Option<PathBuf>,
}

impl Outcome {
    /// Instantiate a bundle from the newly written index and data file that are represented by this `Outcome`
    pub fn to_bundle(&self) -> Option<Result<crate::Bundle, crate::bundle::Error>> {
        self.index_path.as_ref().map(crate::Bundle::at)
    }
}

pub(crate) struct PassThrough<R> {
    pub reader: R,
    pub writer: Option<Arc<parking_lot::Mutex<NamedTempFile>>>,
}

impl<R> io::Read for PassThrough<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        if let Some(writer) = self.writer.as_mut() {
            use io::Write;
            writer.lock().write_all(&buf[..bytes_read])?;
        }
        Ok(bytes_read)
    }
}
impl<R> io::BufRead for PassThrough<R>
where
    R: io::BufRead,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

pub(crate) struct LockWriter {
    pub writer: Arc<parking_lot::Mutex<NamedTempFile>>,
}

impl io::Write for LockWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.lock().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.lock().flush()
    }
}

impl io::Read for LockWriter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.writer.lock().read(buf)
    }
}

impl io::Seek for LockWriter {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.writer.lock().seek(pos)
    }
}
