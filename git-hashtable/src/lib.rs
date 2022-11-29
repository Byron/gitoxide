//! Customized HashMap and Hasher implementation optimized for using `ObjectId`s as keys.
//!
//! The crate mirrors `std::collections` in layout for familiarity.
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

use git_hash::ObjectId;
pub use hashbrown::{hash_map, hash_set, raw, Equivalent};

///
pub mod hash {
    /// A Hasher for usage with HashMap keys that are already robust hashes (like an `ObjectId`).
    /// The first `8` bytes of the hash are used as the `HashMap` hash
    #[derive(Default, Clone, Copy)]
    pub struct Hasher(u64);

    impl std::hash::Hasher for Hasher {
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline(always)]
        fn write(&mut self, bytes: &[u8]) {
            self.0 = u64::from_ne_bytes(bytes[..8].try_into().unwrap());
        }
    }

    /// A Hasher for usage with HashMap keys that are already robust hashes (like an `ObjectId`).
    /// The first `8` bytes of the hash are used as the `HashMap` hash
    #[derive(Default, Clone, Copy)]
    pub struct Builder;
    impl std::hash::BuildHasher for Builder {
        type Hasher = Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            Hasher::default()
        }
    }
}

/// A HashMap for usage with keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub type HashMap<K, V> = hashbrown::HashMap<K, V, hash::Builder>;
/// A HashSet for usage with keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub type HashSet<T = ObjectId> = hashbrown::HashSet<T, hash::Builder>;
