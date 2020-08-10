use crate::pack;
use std::{cell::RefCell, io, path::PathBuf, rc::Rc};
use tempfile::NamedTempFile;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index: pack::index::write::Outcome,
    pub pack_kind: pack::data::Kind,

    pub index_path: Option<PathBuf>,
    pub data_path: Option<PathBuf>,
}

impl Outcome {
    pub fn to_bundle(&self) -> Option<Result<pack::Bundle, pack::bundle::Error>> {
        self.index_path.as_ref().map(pack::Bundle::at)
    }
}

pub(crate) struct PassThrough<R> {
    pub reader: R,
    pub writer: Option<Rc<RefCell<NamedTempFile>>>,
}

impl<R> io::Read for PassThrough<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        if let Some(writer) = self.writer.as_mut() {
            use io::Write;
            writer.borrow_mut().write_all(&buf[..bytes_read])?;
        }
        Ok(bytes_read)
    }
}
