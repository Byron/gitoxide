use crate::{loose, zlib::stream::deflate};
use std::{
    cell::RefCell,
    convert::TryInto,
    io::{self, Write},
};

/// An object database equivalent to `/dev/null`, dropping all objects stored into it.
///
/// It can optionally compress the content, similarly to what would happen when using a [`loose::Db`].
///
pub struct Sink {
    compressor: Option<RefCell<deflate::Writer<io::Sink>>>,
}

impl Sink {
    /// Enable or disable compression. Compression is disabled by default
    pub fn compress(mut self, enable: bool) -> Self {
        if enable {
            self.compressor = Some(RefCell::new(deflate::Writer::new(io::sink())));
        } else {
            self.compressor = None;
        }
        self
    }
}

/// Create a new [`Sink`] with compression disabled.
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
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        use git_features::hash::Sha1;
        let mut buf = [0u8; 8096];

        let possibly_compress = |buf: &[u8]| -> io::Result<()> {
            if let Some(compressor) = self.compressor.as_ref() {
                compressor.try_borrow_mut().expect("no recursion").write_all(buf)?;
            }
            Ok(())
        };
        match hash {
            git_hash::Kind::Sha1 => {
                let mut hasher = Sha1::default();
                let header_len = loose::object::header::encode(kind, size, &mut buf[..])?;
                hasher.update(&buf[..header_len]);
                possibly_compress(&buf[..header_len])?;

                let mut size: usize = size.try_into().expect("object size to fit into usize");
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
