use bstr::ByteSlice;
use std::path::PathBuf;

///
pub mod git_dir {
    use bstr::BString;

    /// The error returned by [`parse::git_dir()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Format should be 'gitdir: <path>', but got: {:?}", .input)]
        InvalidFormat { input: BString },
        #[error("Couldn't decode {:?} as UTF8", .input)]
        IllformedUtf8 { input: BString },
    }
}

/// Parse typical `git_dir` files as seen in worktrees.
pub fn git_dir(input: &[u8]) -> Result<PathBuf, git_dir::Error> {
    let path = input
        .strip_prefix(b"gitdir: ")
        .ok_or_else(|| git_dir::Error::InvalidFormat { input: input.into() })?
        .as_bstr();
    let path = path.trim_end().as_bstr();
    if path.is_empty() {
        return Err(git_dir::Error::InvalidFormat { input: input.into() });
    }
    Ok(git_path::try_from_bstr(path)
        .map_err(|_| git_dir::Error::IllformedUtf8 { input: input.into() })?
        .into_owned())
}
