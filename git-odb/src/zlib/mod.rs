use flate2::Decompress;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not write all bytes when decompressing content")]
    WriteInflated(#[from] std::io::Error),
    #[error("Could not decode zip stream, status was '{0:?}'")]
    Inflate(#[from] flate2::DecompressError),
}

/// Decompress a few bytes of a zlib stream without allocation
pub struct Inflate {
    pub(crate) state: Decompress,
}

impl Default for Inflate {
    fn default() -> Self {
        Inflate {
            state: Decompress::new(true),
        }
    }
}

impl Inflate {
    /// Run the decompressor exactly once. Cannot be run multiple times
    pub fn once(&mut self, input: &[u8], out: &mut [u8]) -> Result<(flate2::Status, usize, usize), Error> {
        let before_in = self.state.total_in();
        let before_out = self.state.total_out();
        let status = self.state.decompress(input, out, flate2::FlushDecompress::None)?;
        Ok((
            status,
            (self.state.total_in() - before_in) as usize,
            (self.state.total_out() - before_out) as usize,
        ))
    }
}

pub mod stream;
