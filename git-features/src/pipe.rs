use bytes::{Buf, BufMut, BytesMut};
use std::io;

pub struct Writer {
    channel: std::sync::mpsc::SyncSender<BytesMut>,
    buf: BytesMut,
}

pub struct Reader {
    channel: std::sync::mpsc::Receiver<BytesMut>,
    buf: BytesMut,
}

impl io::Read for Reader {
    fn read(&mut self, mut out: &mut [u8]) -> io::Result<usize> {
        let mut written = 0;
        while !out.is_empty() {
            if self.buf.is_empty() {
                if let Ok(buf) = self.channel.recv() {
                    self.buf = buf;
                }
                if self.buf.is_empty() {
                    break;
                }
            }
            let bytes_to_write = self.buf.len().min(out.len());
            let (to_write, rest) = out.split_at_mut(bytes_to_write);
            self.buf.split_to(bytes_to_write).copy_to_slice(to_write);
            out = rest;
            written += bytes_to_write;
        }
        Ok(written)
    }
}

impl io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.put_slice(buf);
        self.channel
            .send(self.buf.split())
            .map_err(|err| io::Error::new(io::ErrorKind::BrokenPipe, err))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn unidirectional(in_flight_writes: impl Into<Option<usize>>) -> (Writer, Reader) {
    let (tx, rx) = std::sync::mpsc::sync_channel(in_flight_writes.into().unwrap_or(0));
    (
        Writer {
            channel: tx,
            buf: BytesMut::with_capacity(4096),
        },
        Reader {
            channel: rx,
            buf: BytesMut::new(),
        },
    )
}
