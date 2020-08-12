use git_features::hash;
use git_object::{owned, HashKind};
use std::{io, path::Path};

pub(crate) struct Write<T> {
    pub hash: hash::Sha1,
    pub inner: T,
}

impl<T> io::Write for Write<T>
where
    T: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hash.update(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T> Write<T>
where
    T: io::Write,
{
    pub fn new(inner: T, kind: HashKind) -> Self {
        match kind {
            HashKind::Sha1 => Write {
                inner,
                hash: hash::Sha1::default(),
            },
        }
    }
}

pub(crate) fn bytes_of_file(
    path: impl AsRef<Path>,
    num_bytes_from_start: usize,
    progress: &mut impl git_features::progress::Progress,
) -> io::Result<owned::Id> {
    let mut hasher = hash::Sha1::default();
    let start = std::time::Instant::now();
    // init progress before the possibility for failure, as convenience in case people want to recover
    progress.init(Some(num_bytes_from_start), git_features::progress::bytes());

    let mut file = std::fs::File::open(path)?;
    use std::io::Read;
    const BUF_SIZE: usize = u16::MAX as usize;
    let mut buf = [0u8; BUF_SIZE];
    let mut bytes_left = num_bytes_from_start;

    while bytes_left > 0 {
        let out = &mut buf[..BUF_SIZE.min(bytes_left)];
        file.read_exact(out)?;
        bytes_left -= out.len();
        progress.inc_by(out.len());
        hasher.update(out);
    }

    let id = owned::Id::new_sha1(hasher.digest());
    progress.show_throughput(start);
    Ok(id)
}
