use crate::Stream;
use gix_object::bstr::BStr;

/// Use `find` to traverse `tree` and fetch the contained blobs to return as [`Stream`], which makes them queryable
/// on demand with support for streaming each entry.
/// `pipeline` is used to convert blobs to their worktree representation, and `attributes` is used to read
/// the `export-ignore` attribute. If set on a directory or blob, it won't be added to the archive.
///
/// ### Progress and interruptions
///
/// For per-file progress, integrate progress handling into `find` as it is called for trees and blobs.
/// `find` should also be used for interrupt handling, as it can return an error once per file.
/// For progress on bytes-written, integrate progress reporting when consuming the stream.
/// Further it's possible to drop the returned [`Stream`] to halt all operation.
///
/// ### Threaded Operation
///
/// This function spawns a thread that will access the tree data in the background, synchronized through
/// `Stream` so that it will not be faster than the consumer, with at most one file in flight at any time.
///
/// ### Limitations
///
/// * `export-subst` is not support, as it requires the entire formatting engine of `git log`.
pub fn write_to<W, Find, E1, E2>(
    _tree: gix_hash::ObjectId,
    mut _find: Find,
    _pipeline: &mut gix_filter::Pipeline,
    _attributes: impl FnMut(&BStr, &mut gix_attributes::search::Outcome) -> Result<(), E2> + Send,
) -> Stream
where
    W: std::io::Write,
    Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::Data<'a>, E1> + Send,
    E1: std::error::Error + Send + Sync + 'static,
    E2: std::error::Error + Send + Sync + 'static,
{
    todo!()
}
