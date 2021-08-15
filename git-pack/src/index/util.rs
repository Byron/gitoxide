use std::{io, time::Instant};

use git_features::progress::{self, Progress};

pub(crate) fn index_entries_sorted_by_offset_ascending(
    idx: &crate::index::File,
    mut progress: impl Progress,
) -> Vec<crate::index::Entry> {
    progress.init(Some(idx.num_objects as usize), progress::count("entries"));
    let start = Instant::now();

    let mut v = Vec::with_capacity(idx.num_objects as usize);
    for entry in idx.iter() {
        v.push(entry);
        progress.inc();
    }
    v.sort_by_key(|e| e.pack_offset);

    progress.show_throughput(start);
    v
}

pub(crate) struct Count<W> {
    pub bytes: u64,
    pub inner: W,
}

impl<W> Count<W> {
    pub fn new(inner: W) -> Self {
        Count { bytes: 0, inner }
    }
}

impl<W> io::Write for Count<W>
where
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
