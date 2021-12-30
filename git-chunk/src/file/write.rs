#![allow(missing_docs, unused)]
use crate::file::index::Entry;
use crate::file::Index;

enum State {
    Collecting,
    WriteChunks,
}

mod write_chunk {
    use crate::file::index;
    use std::collections::VecDeque;

    pub struct Chunk<W> {
        chunks_to_write: VecDeque<index::Entry>,
        inner: W,
        next_chunk: Option<index::Entry>,
        written_bytes: usize,
    }

    impl<W> Chunk<W>
    where
        W: std::io::Write,
    {
        pub(crate) fn new(out: W, chunks: VecDeque<index::Entry>) -> Chunk<W>
        where
            W: std::io::Write,
        {
            Chunk {
                chunks_to_write: chunks,
                inner: out,
                next_chunk: None,
                written_bytes: 0,
            }
        }
    }

    impl<W> std::io::Write for Chunk<W>
    where
        W: std::io::Write,
    {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let written = self.inner.write(buf)?;
            self.written_bytes += written;
            Ok(written)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.inner.flush()
        }
    }

    impl<W> Chunk<W> {
        /// Return the inner writer - should only be called once there is no more chunk to write.
        pub fn into_inner(self) -> W {
            self.inner
        }
        /// Return the next chunk-id to write, if there is one.
        pub fn next_chunk(&mut self) -> Option<crate::Id> {
            if let Some(entry) = self.next_chunk.take() {
                assert_eq!(
                    entry.offset.end,
                    self.written_bytes as u64,
                    "BUG: expected to write {} bytes, but only wrote {} for chunk {:?}",
                    entry.offset.end,
                    self.written_bytes,
                    std::str::from_utf8(&entry.kind)
                )
            }
            self.written_bytes = 0;
            self.next_chunk = self.chunks_to_write.pop_front();
            self.next_chunk.as_ref().map(|e| e.kind)
        }
    }
}
pub use write_chunk::Chunk;

/// Writing
impl Index {
    /// Create a new index whose sole purpose is to be receiving chunks using [`plan_chunk()`][Index::plan_chunk()] and to be written to
    /// an output using [`write_to()`][Index::into_write()]
    pub fn for_writing() -> Self {
        Index {
            will_write: true,
            chunks: Vec::new(),
        }
    }
    /// Plan to write a new chunk as part of the index when [`write_to()`][Index::write_to()] is called.
    pub fn plan_chunk(&mut self, chunk: crate::Id, exact_size_on_disk: u64) {
        self.will_write = true;
        self.chunks.push(Entry {
            kind: chunk,
            offset: 0..exact_size_on_disk,
        })
    }

    /// After [planning all chunks][Index::plan_chunk()] call this method with the destination to write the chunks to.
    /// Use the [Chunk] writer to write each chunk in order.
    pub fn into_write<W>(self, out: W) -> std::io::Result<Chunk<W>>
    where
        W: std::io::Write,
    {
        // TODO: write index
        Ok(Chunk::new(out, self.chunks.into()))
    }
}
