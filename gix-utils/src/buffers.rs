use crate::Buffers;

/// Lifecycle
impl Buffers {
    /// Use this if there is an input buffer `src` which isn't owned by you, but which should be used as source when
    /// asking for [`src_and_dest()`](WithForeignSource::src_and_dest()).
    pub fn use_foreign_src<'a, 'src>(&'a mut self, src: &'src [u8]) -> WithForeignSource<'src, 'a> {
        self.clear();
        WithForeignSource {
            ro_src: Some(src),
            src: &mut self.src,
            dest: &mut self.dest,
        }
    }
}

impl Buffers {
    /// Clear all buffers, which should be called once processing is done.
    pub fn clear(&mut self) {
        self.src.clear();
        self.dest.clear();
    }

    /// Must be called after every change (i.e. when it's known that `dest` was written.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.src, &mut self.dest);
    }
}

/// A utility to do buffer-swapping with, similar to [`Buffers`], but with support for a
/// read-only one-time buffer as source.
pub struct WithForeignSource<'src, 'bufs> {
    /// The original source buffer, or `None` if already altered.
    pub ro_src: Option<&'src [u8]>,
    /// The source buffer that will be used after the first call to `swap`.
    pub src: &'bufs mut Vec<u8>,
    dest: &'bufs mut Vec<u8>,
}

impl WithForeignSource<'_, '_> {
    /// Must be called after every change (i.e. when it's known that `dest` was written.
    pub fn swap(&mut self) {
        self.ro_src.take();
        std::mem::swap(&mut self.src, &mut self.dest);
        self.dest.clear();
    }
    /// Obtain `(source, destination)`, which reads from the read-only source exactly once.
    pub fn src_and_dest(&mut self) -> (&[u8], &mut Vec<u8>) {
        match self.ro_src {
            Some(src) => (src, &mut self.dest),
            None => (self.src, &mut self.dest),
        }
    }
}
