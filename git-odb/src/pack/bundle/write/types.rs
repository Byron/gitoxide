use crate::pack;
use std::io;
use tempfile::NamedTempFile;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index: pack::index::write::Outcome,
    pub pack_kind: pack::data::Kind,
}

pub(crate) struct PassThrough<R> {
    pub reader: R,
    pub writer: Option<NamedTempFile>,
}

impl<R> io::Read for PassThrough<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        if let Some(writer) = self.writer.as_mut() {
            use io::Write;
            writer.write_all(&buf[..bytes_read])?;
        }
        Ok(bytes_read)
    }
}
