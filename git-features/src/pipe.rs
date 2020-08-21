use std::io;

pub struct Writer;

pub struct Reader;

impl io::Read for Reader {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl io::Write for Writer {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

pub fn unidirectional() -> (Writer, Reader) {
    unimplemented!("unidirectional pipe")
}
