use std::{
    cell::RefCell,
    convert::TryInto,
    io::{self, Write},
};

use gix_features::zlib::stream::deflate;

use crate::Sink;

impl Sink {
    /// Enable or disable compression. Compression is disabled by default
    pub fn compress(mut self, enable: bool) -> Self {
        if enable {
            self.compressor = Some(RefCell::new(deflate::Write::new(io::sink())));
        } else {
            self.compressor = None;
        }
        self
    }
}

impl crate::traits::Write for Sink {
    type Error = io::Error;

    fn write_stream(
        &self,
        kind: gix_object::Kind,
        size: u64,
        mut from: impl io::Read,
    ) -> Result<gix_hash::ObjectId, Self::Error> {
        let mut size = size.try_into().expect("object size to fit into usize");
        use gix_features::hash::Sha1;
        let mut buf = [0u8; 8096];
        let header = gix_object::encode::loose_header(kind, size);

        let possibly_compress = |buf: &[u8]| -> io::Result<()> {
            if let Some(compressor) = self.compressor.as_ref() {
                compressor.try_borrow_mut().expect("no recursion").write_all(buf)?;
            }
            Ok(())
        };
        match self.object_hash {
            gix_hash::Kind::Sha1 => {
                let mut hasher = Sha1::default();
                hasher.update(&header);
                possibly_compress(&header)?;

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
