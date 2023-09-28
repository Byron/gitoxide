use std::{
    cell::RefCell,
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
    fn write_stream(
        &self,
        kind: gix_object::Kind,
        mut size: u64,
        from: &mut dyn io::Read,
    ) -> Result<gix_hash::ObjectId, crate::write::Error> {
        let mut buf = [0u8; u16::MAX as usize];
        let header = gix_object::encode::loose_header(kind, size);

        let possibly_compress = |buf: &[u8]| -> io::Result<()> {
            if let Some(compressor) = self.compressor.as_ref() {
                compressor.try_borrow_mut().expect("no recursion").write_all(buf)?;
            }
            Ok(())
        };

        let mut hasher = gix_features::hash::hasher(self.object_hash);
        hasher.update(&header);
        possibly_compress(&header).map_err(Box::new)?;

        while size != 0 {
            let bytes = (size as usize).min(buf.len());
            from.read_exact(&mut buf[..bytes]).map_err(Box::new)?;
            hasher.update(&buf[..bytes]);
            possibly_compress(&buf[..bytes]).map_err(Box::new)?;
            size -= bytes as u64;
        }
        if let Some(compressor) = self.compressor.as_ref() {
            let mut c = compressor.borrow_mut();
            c.flush().map_err(Box::new)?;
            c.reset();
        }

        Ok(hasher.digest().into())
    }
}
