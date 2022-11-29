use std::{
    borrow::Borrow,
    convert::TryInto,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::{borrowed::oid, Kind, SIZE_OF_SHA1_DIGEST};

/// An owned hash identifying objects, most commonly Sha1
#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectId {
    /// A SHA 1 hash digest
    Sha1([u8; SIZE_OF_SHA1_DIGEST]),
}

// False positive: https://github.com/rust-lang/rust-clippy/issues/2627
// ingoring some fields while hashing is perfectly valid and just leads to
// increased HashCollisions. One Sha1 being a prefix of another Sha256 is
// extremly unlikely to begin with so it doesn't matter.
// This implementation matches the `Hash` implementation for `oid`
// and allows the usage of custom Hashers that only copy a truncated ShaHash
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for ObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_slice())
    }
}

#[allow(missing_docs)]
pub mod decode {
    use std::str::FromStr;

    use crate::object_id::ObjectId;

    /// An error returned by [`ObjectId::from_hex()`][crate::ObjectId::from_hex()]
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("A hash sized {0} hexadecimal characters is invalid")]
        InvalidHexEncodingLength(usize),
        #[error("Invalid character {c} at position {index}")]
        Invalid { c: char, index: usize },
    }

    /// Hash decoding
    impl ObjectId {
        /// Create an instance from a `buffer` of 40 bytes encoded with hexadecimal notation.
        ///
        /// Such a buffer can be obtained using [`oid::write_hex_to(buffer)`][super::oid::write_hex_to()]
        pub fn from_hex(buffer: &[u8]) -> Result<ObjectId, Error> {
            use hex::FromHex;
            match buffer.len() {
                40 => Ok(ObjectId::Sha1(<[u8; 20]>::from_hex(buffer).map_err(
                    |err| match err {
                        hex::FromHexError::InvalidHexCharacter { c, index } => Error::Invalid { c, index },
                        hex::FromHexError::OddLength | hex::FromHexError::InvalidStringLength => {
                            unreachable!("BUG: This is already checked")
                        }
                    },
                )?)),
                len => Err(Error::InvalidHexEncodingLength(len)),
            }
        }
    }

    impl FromStr for ObjectId {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_hex(s.as_bytes())
        }
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
