use crate::{oid, ObjectId, Prefix};
use quick_error::quick_error;
use std::cmp::Ordering;

quick_error! {
    /// The error returned by [Prefix::try_from_id()][super::Prefix::try_from_id()].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        TooShort { hex_len: usize } {
            display("The minimum hex length of a short object id is 4, got {}", hex_len)
        }
        TooLong { object_kind: crate::Kind, hex_len: usize } {
            display("An object of kind {} cannot be larger than {} in hex, but {} was requested", object_kind, object_kind.len_in_hex(), hex_len)
        }
    }
}

impl Prefix {
    /// Create a new instance by taking a full `id` as input and truncating it to `hex_len`.
    ///
    /// For instance, with `hex_len` of 7 the resulting prefix is 3.5 bytes, or 3 bytes and 4 bits
    /// wide, with all other bytes and bits set to zero.
    pub fn new(id: impl AsRef<oid>, hex_len: usize) -> Result<Self, Error> {
        let id = id.as_ref();
        if hex_len > id.kind().len_in_hex() {
            Err(Error::TooLong {
                object_kind: id.kind(),
                hex_len,
            })
        } else if hex_len < 4 {
            Err(Error::TooShort { hex_len })
        } else {
            let mut prefix = ObjectId::null(id.kind());
            let b = prefix.as_mut_slice();
            let copy_len = (hex_len + 1) / 2;
            b[..copy_len].copy_from_slice(&id.as_bytes()[..copy_len]);
            if hex_len % 2 == 1 {
                b[hex_len / 2] &= 0xf0;
            }

            Ok(Prefix { bytes: prefix, hex_len })
        }
    }

    /// Returns the prefix as object id.
    ///
    /// Note that it may be deceptive to use given that it looks like a full
    /// object id, even though its post-prefix bytes/bits are set to zero.
    pub fn as_oid(&self) -> &oid {
        &self.bytes
    }

    /// Return the amount of hexadecimal characters that are set in the prefix.
    ///
    /// This gives the prefix a granularity of 4 bits.
    pub fn hex_len(&self) -> usize {
        self.hex_len
    }

    /// Provided with candidate id which is a full hash, determine how this prefix compares to it,
    /// only looking at the prefix bytes, ignoring everything behind that.
    pub fn cmp_oid(&self, candidate: &oid) -> Ordering {
        let common_len = self.hex_len / 2;

        self.bytes.as_bytes()[..common_len]
            .cmp(&candidate.as_bytes()[..common_len])
            .then(if self.hex_len % 2 == 1 {
                let half_byte_idx = self.hex_len / 2;
                self.bytes.as_bytes()[half_byte_idx].cmp(&(candidate.as_bytes()[half_byte_idx] & 0xf0))
            } else {
                Ordering::Equal
            })
    }

    /// Create an instance from the given hexadecimal prefix, e.g. `35e77c16` would yield a `Prefix` with `hex_len()` = 8.
    pub fn from_hex(_hex: &str) -> Self {
        todo!("Prefix::from_hex()")
    }
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bytes.to_hex_with_len(self.hex_len).fmt(f)
    }
}
