use crate::{
    store::file::{log, loose, Reference},
    FullName,
};
use bstr::ByteSlice;
use std::io::Read;

impl<'a> Reference<'a> {
    /// Returns true if a reflog exists.
    ///
    /// Please note that this method shouldn't be used to check if a log exists before trying to read it, but instead
    /// is meant to be the fastest possible way to determine if a log exists or not.
    /// If the caller needs to know if it's readable, try to read the log instead with a reverse or forward iterator.
    pub fn log_exists(&self) -> Result<bool, loose::reflog::Error> {
        // NOTE: Have to repeat the implementation of store::reflog_iter here as borrow_check believes impl Iterator binds self
        use os_str_bytes::OsStrBytes;
        let name = self.relative_path.as_path().to_raw_bytes();
        Ok(self.parent.reflog_path(FullName(name.as_bstr())).is_file())
    }
    /// Return a reflog reverse iterator for this ref, reading chunks from the back into the fixed buffer `buf`.
    ///
    /// The iterator will traverse log entries from most recent to oldest, reading the underlying file in chunks from the back.
    /// Return `Ok(None)` if no reflog exists.
    pub fn log_iter_rev<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<Option<log::iter::Reverse<'b, std::fs::File>>, loose::reflog::Error> {
        // NOTE: Have to repeat the implementation of store::reflog_iter here as borrow_check believes impl Iterator binds self
        use os_str_bytes::OsStrBytes;
        let name = self.relative_path.as_path().to_raw_bytes();
        let file = match std::fs::File::open(self.parent.reflog_path(FullName(name.as_bstr()))) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        Ok(Some(log::iter::reverse(file, buf, self.parent.hash)?))
    }

    /// Return a reflog forward iterator for this ref and write its file contents into `buf`.
    ///
    /// The iterator will traverse log entries from oldest to newest.
    /// Return `Ok(None)` if no reflog exists.
    pub fn log_iter<'b>(
        &self,
        buf: &'b mut Vec<u8>,
    ) -> Result<Option<impl Iterator<Item = Result<log::Line<'b>, log::iter::decode::Error>>>, loose::reflog::Error>
    {
        // NOTE: Have to repeat the implementation of store::reflog_iter here as borrow_check believes impl Iterator binds self
        use os_str_bytes::OsStrBytes;
        let name = self.relative_path.as_path().to_raw_bytes();
        match std::fs::File::open(self.parent.reflog_path(FullName(name.as_bstr()))) {
            Ok(mut file) => {
                buf.clear();
                file.read_to_end(buf)?;
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err.into()),
        };
        Ok(Some(log::iter::forward(buf, self.parent.hash)))
    }
}
