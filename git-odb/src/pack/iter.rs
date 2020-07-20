use std::io;

/// A pack iterator yielding entries from offsets and a seekable stream
///
/// Useful for pack files on disk
pub struct Indexed<I, RS> {
    _inner: RS,
    _offsets: I,
}

type PackOffset = u64;

impl<I, RS> Indexed<I, RS>
where
    I: Iterator<Item = PackOffset>,
    RS: io::Read + io::Seek,
{
}
