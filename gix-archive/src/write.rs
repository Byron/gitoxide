use crate::{Error, Options};
use gix_object::bstr::BStr;

/// Use `find` to traverse `tree` and fetch the contained blobs to write to `out` configured according to `opts`.
/// `pipeline` is used to convert blobs to their worktree representation, and `attributes` is used to read
/// the `export-ignore` attribute. If set on a directory or blob, it won't be added to the archive.
///
/// ### Progress and interruptions
///
/// For per-file progress, integrate progress handling into `find` as it is called for trees and blobs.
/// `find` should also be used for interrupt handling, as it can return an error once per file.
/// For progress on bytes-written, integrate progress reporting into `out`.
///
/// ### Limitations
///
/// * `export-subst` is not support, as it requires the entire formatting engine of `git log`.
pub fn write_to<W, Find, E>(
    _tree: &gix_hash::oid,
    mut _find: Find,
    _pipeline: &mut gix_filter::Pipeline,
    _attributes: impl FnOnce(&BStr, &mut gix_attributes::search::Outcome),
    mut _out: W,
    _opts: Options,
) -> Result<(), Error<E>>
where
    W: std::io::Write,
    Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::Data<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(())
}
