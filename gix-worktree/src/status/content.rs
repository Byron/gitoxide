use gix_hash::ObjectId;
use gix_index as index;
use index::Entry;

/// Compares the content of two blobs in some way.
pub trait CompareBlobs {
    /// Output data produced by [`compare_blobs()`][CompareBlobs::compare_blobs()].
    type Output;

    /// Providing the underlying index `entry`, allow comparing a file in the worktree of size `worktree_blob_size`
    /// and allow reading its bytes using `worktree_blob`.
    /// If this function returns `None` the `entry` and the `worktree_blob` are assumed to be identical.
    /// Use `entry_blob` to obtain the data for the blob referred to by `entry`, allowing comparisons of the data itself.
    fn compare_blobs<'a, E>(
        &mut self,
        entry: &'a gix_index::Entry,
        worktree_blob_size: usize,
        worktree_blob: impl ReadDataOnce<'a, E>,
        entry_blob: impl ReadDataOnce<'a, E>,
    ) -> Result<Option<Self::Output>, E>;
}

/// Lazy borrowed access to blob data.
pub trait ReadDataOnce<'a, E> {
    /// Returns the contents of this blob.
    ///
    /// This potentially performs IO and other expensive operations
    /// and should only be called when necessary.
    fn read_data(self) -> Result<&'a [u8], E>;
}

/// Compares to blobs by comparing their size and oid, and only looks at the file if
/// the size matches, therefore it's very fast.
#[derive(Clone)]
pub struct FastEq;

impl CompareBlobs for FastEq {
    type Output = ();

    fn compare_blobs<'a, E>(
        &mut self,
        entry: &'a Entry,
        worktree_blob_size: usize,
        worktree_blob: impl ReadDataOnce<'a, E>,
        _entry_blob: impl ReadDataOnce<'a, E>,
    ) -> Result<Option<Self::Output>, E> {
        // make sure to account for racily smudged entries here so that they don't always keep
        // showing up as modified even after their contents have changed again, to a potentially
        // unmodified state. That means that we want to ignore stat.size == 0 for non_empty_blobs.
        if entry.stat.size as usize != worktree_blob_size && (entry.id.is_empty_blob() || entry.stat.size != 0) {
            return Ok(Some(()));
        }
        let blob = worktree_blob.read_data()?;
        let file_hash = gix_object::compute_hash(entry.id.kind(), gix_object::Kind::Blob, blob);
        Ok((entry.id != file_hash).then_some(()))
    }
}

/// Compares files to blobs by *always* comparing their hashes.
///
/// Same as [`FastEq`] but does not contain a fast path for files with mismatched files and
/// therefore always returns an OID that can be reused later.
#[derive(Clone)]
pub struct HashEq;

impl CompareBlobs for HashEq {
    type Output = ObjectId;

    fn compare_blobs<'a, E>(
        &mut self,
        entry: &'a Entry,
        _worktree_blob_size: usize,
        worktree_blob: impl ReadDataOnce<'a, E>,
        _entry_blob: impl ReadDataOnce<'a, E>,
    ) -> Result<Option<Self::Output>, E> {
        let blob = worktree_blob.read_data()?;
        let file_hash = gix_object::compute_hash(entry.id.kind(), gix_object::Kind::Blob, blob);
        Ok((entry.id != file_hash).then_some(file_hash))
    }
}
