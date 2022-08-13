use crate::{extension, extension::Signature};

/// The signature of the end-of-index-entry extension
pub const SIGNATURE: Signature = *b"EOIE";
/// The minimal size of the extension, depending on the shortest hash.
pub const MIN_SIZE: usize = 4 /* offset to extensions */ + git_hash::Kind::shortest().len_in_bytes();
/// The smallest size of the extension varying by hash kind, along with the standard extension header.
pub const MIN_SIZE_WITH_HEADER: usize = extension::MIN_SIZE + MIN_SIZE;

mod decode;
pub use decode::decode;

mod write {
    use crate::extension::end_of_index_entry::SIGNATURE;
    use crate::extension::Signature;

    /// Write this extension to out and generate a hash of `hash_kind` over all `prior_extensions` which are specified as `(signature, size)`
    /// pair. `one_past_entries` is the offset to the first byte past the entries, which is also the first byte of the signature of the
    /// first extension in `prior_extensions`. Note that `prior_extensions` must have been written prior to this one, as the name suggests,
    /// allowing this extension to be the last one in the index file.
    pub fn write_to(
        out: &mut impl std::io::Write,
        hash_kind: git_hash::Kind,
        one_past_entries: u32,
        prior_extensions: &[(Signature, u32)],
    ) -> Result<(), std::io::Error> {
        out.write_all(&SIGNATURE)?;
        out.write_all(&(4 + hash_kind.len_in_bytes()).to_be_bytes())?;
        out.write_all(&one_past_entries.to_be_bytes())?;

        let mut hasher = git_features::hash::hasher(hash_kind);
        for (signature, size) in prior_extensions {
            hasher.update(signature);
            hasher.update(&size.to_be_bytes());
        }
        out.write_all(&hasher.digest())?;

        Ok(())
    }
}
pub use write::write_to;
