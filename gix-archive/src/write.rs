use crate::{Error, Options};
use gix_worktree_stream::Stream;

/// Write the worktree `stream` to `out` configured according to `opts`.
pub fn write_to(_stream: &mut Stream, mut _out: impl std::io::Write, _opts: Options) -> Result<(), Error> {
    Ok(())
}
