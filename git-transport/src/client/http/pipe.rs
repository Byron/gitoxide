use std::io;

pub struct PipeWriter;

pub struct PipeReader;

impl io::Read for PipeReader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl io::Write for PipeWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

pub fn _unidirectional() -> (PipeWriter, PipeReader) {
    unimplemented!("unidirectional pipe")
}
