use std::{borrow::Borrow, convert::TryInto, fmt, ops::Deref};

use crate::{borrowed::oid, Kind, SIZE_OF_SHA1_DIGEST};

/// An partial owned hash possibly identifying an object uniquely,
/// whose non-prefix bytes are zeroed.
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefix {
    bytes: ObjectId,
    hex_len: usize,
}

///
pub mod prefix {
    use quick_error::quick_error;

    quick_error! {
        /// TODO:
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
}

impl Prefix {
    /// Create a new instance by taking a full `id` as input and truncating it to `hex_len`.
    ///
    /// For instance, with `hex_len` of 7 the resulting prefix is 3.5 bytes, or 3 bytes and 4 bits
    /// wide, with all other bytes and bits set to zero.
    pub fn try_from_id(id: impl AsRef<oid>, hex_len: usize) -> Result<Self, prefix::Error> {
        let id = id.as_ref();
        if hex_len > id.kind().len_in_hex() {
            return Err(prefix::Error::TooLong {
                object_kind: id.kind(),
                hex_len,
            });
        } else if hex_len < 4 {
            return Err(prefix::Error::TooShort { hex_len });
        }

        let mut prefix = ObjectId::null(id.kind());
        let b = prefix.as_mut_slice();
        let copy_len = (hex_len + 1) / 2;
        b[..copy_len].copy_from_slice(&id.as_bytes()[..copy_len]);
        if hex_len % 2 == 1 {
            b[hex_len / 2] &= 0xf0;
        }

        Ok(Prefix { bytes: prefix, hex_len })
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
}

/// An owned hash identifying objects, most commonly Sha1
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectId {
    /// A SHA 1 hash digest
    Sha1([u8; SIZE_OF_SHA1_DIGEST]),
}

impl std::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectId::Sha1(_hash) => f.write_str("Sha1(")?,
        }
        for b in self.as_bytes() {
            write!(f, "{:02x}", b)?;
        }
        f.write_str(")")
    }
}

/// Access and conversion
impl ObjectId {
    /// Returns the kind of hash used in this `Id`
    #[inline]
    pub fn kind(&self) -> crate::Kind {
        match self {
            ObjectId::Sha1(_) => crate::Kind::Sha1,
        }
    }
    /// Return the raw byte slice representing this hash
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Sha1(b) => b.as_ref(),
        }
    }
    /// Return the raw mutable byte slice representing this hash
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            Self::Sha1(b) => b.as_mut(),
        }
    }

    /// The hash of an empty tree
    #[inline]
    pub const fn empty_tree(hash: Kind) -> ObjectId {
        match hash {
            Kind::Sha1 => {
                ObjectId::Sha1(*b"\x4b\x82\x5d\xc6\x42\xcb\x6e\xb9\xa0\x60\xe5\x4b\xf8\xd6\x92\x88\xfb\xee\x49\x04")
            }
        }
    }

    /// Returns true if this hash consists of all null bytes
    #[inline]
    pub fn is_null(&self) -> bool {
        match self {
            ObjectId::Sha1(digest) => &digest[..] == oid::null_sha1().as_bytes(),
        }
    }

    /// Returns an Digest representing a hash with whose memory is zeroed.
    #[inline]
    pub const fn null(kind: crate::Kind) -> ObjectId {
        match kind {
            crate::Kind::Sha1 => Self::null_sha1(),
        }
    }
}

/// Sha1 hash specific methods
impl ObjectId {
    /// Instantiate an Digest from 20 bytes of a Sha1 digest.
    #[inline]
    fn new_sha1(id: [u8; SIZE_OF_SHA1_DIGEST]) -> Self {
        ObjectId::Sha1(id)
    }

    /// Instantiate an Digest from a slice 20 borrowed bytes of a Sha1 digest.
    ///
    /// Panics of the slice doesn't have a length of 20.
    #[inline]
    pub(crate) fn from_20_bytes(b: &[u8]) -> ObjectId {
        let mut id = [0; SIZE_OF_SHA1_DIGEST];
        id.copy_from_slice(b);
        ObjectId::Sha1(id)
    }

    /// Returns an Digest representing a Sha1 with whose memory is zeroed.
    #[inline]
    pub(crate) const fn null_sha1() -> ObjectId {
        ObjectId::Sha1([0u8; 20])
    }
}

impl From<[u8; SIZE_OF_SHA1_DIGEST]> for ObjectId {
    fn from(v: [u8; 20]) -> Self {
        Self::new_sha1(v)
    }
}

impl From<&[u8]> for ObjectId {
    fn from(v: &[u8]) -> Self {
        match v.len() {
            20 => Self::Sha1(v.try_into().expect("prior length validation")),
            other => panic!("BUG: unsupported hash len: {}", other),
        }
    }
}

impl From<&crate::oid> for ObjectId {
    fn from(v: &oid) -> Self {
        match v.kind() {
            crate::Kind::Sha1 => ObjectId::from_20_bytes(v.as_bytes()),
        }
    }
}

impl Deref for ObjectId {
    type Target = oid;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<crate::oid> for ObjectId {
    fn as_ref(&self) -> &oid {
        oid::from_bytes_unchecked(self.as_slice())
    }
}

impl Borrow<crate::oid> for ObjectId {
    fn borrow(&self) -> &oid {
        self.as_ref()
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl PartialEq<&crate::oid> for ObjectId {
    fn eq(&self, other: &&oid) -> bool {
        self.as_ref() == *other
    }
}
