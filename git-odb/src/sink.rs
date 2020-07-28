use crate::{loose, zlib::stream::DeflateWriter};
use git_object::{owned::Id, HashKind};
use std::{
    cell::RefCell,
    convert::TryInto,
    io::{self, Write},
};

pub struct Sink {
    compressor: Option<RefCell<DeflateWriter<io::Sink>>>,
}

impl Sink {
    pub fn compress(&mut self, enable: bool) -> &mut Self {
        if enable {
            self.compressor = Some(RefCell::new(DeflateWriter::new(io::sink())));
        } else {
            self.compressor = None;
        }
        self
    }
}

pub fn sink() -> Sink {
    Sink { compressor: None }
}

impl crate::Write for Sink {
    type Error = io::Error;

    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        mut from: impl io::Read,
        hash: HashKind,
    ) -> Result<Id, Self::Error> {
        use git_features::hash::Sha1;
        let mut buf = [0u8; 8096];

        let possibly_compress = |buf: &[u8]| -> io::Result<()> {
            if let Some(compressor) = self.compressor.as_ref() {
                compressor.try_borrow_mut().expect("no recursion").write_all(buf)?;
            }
            Ok(())
        };
        match hash {
            HashKind::Sha1 => {
                let mut hasher = Sha1::default();
                let header_len = loose::object::header::encode(kind, size, &mut buf[..])?;
                hasher.update(&buf[..header_len]);
                possibly_compress(&buf[..header_len])?;

                let mut size: usize = size.try_into().unwrap();
                while size != 0 {
                    let bytes = size.min(buf.len());
                    from.read_exact(&mut buf[..bytes])?;
                    hasher.update(&buf[..bytes]);
                    possibly_compress(&buf[..bytes])?;
                    size -= bytes;
                }
                if let Some(compressor) = self.compressor.as_ref() {
                    let mut c = compressor.borrow_mut();
                    c.flush()?;
                    c.reset();
                }

                Ok(hasher.digest().into())
            }
        }
    }
}
