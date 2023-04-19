use crate::{Error, Options};

/// Use `find` to traverse `tree` and fetch the contained blobs to write to `out` configured according to `opts`.
///
/// ### Limitations
///
/// * `.gitattributes` aren't considered, and filters are not applied, affecting correctness.
pub fn write_to<W, Find, E>(_tree: &gix_hash::oid, mut _find: Find, mut _out: W, _opts: Options) -> Result<(), Error<E>>
where
    W: std::io::Write,
    Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::Data<'a>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(())
}
