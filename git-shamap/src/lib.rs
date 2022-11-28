#![deny(missing_docs, rust_2018_idioms)]
//! Customized HashMap and Hasher implementation optimized for using `ObjectId`s as keys

use git_hash::ObjectId;
use std::hash::{BuildHasher, Hasher};

pub use hashbrown::{hash_map, hash_set, raw, Equivalent};
use hashbrown::{HashMap, HashSet};

// mod raw;

#[derive(Default, Clone, Copy)]
/// A Hasher for usage with HashMap keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub struct ShaHasher(u64);

impl Hasher for ShaHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(bytes[..8].try_into().unwrap());
    }
}

#[derive(Default, Clone, Copy)]
/// A Hasher for usage with HashMap keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub struct ShaHashBuilder;
impl BuildHasher for ShaHashBuilder {
    type Hasher = ShaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ShaHasher::default()
    }
}

/// A HashMap for usage with keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub type ShaHashMap<K, V> = HashMap<K, V, ShaHashBuilder>;
/// A HashSet for usage with keys that are already robust hashes (like an `ObjectId`).
/// The first `8` bytes of the hash are used as the `HashMap` hash
pub type ShaHashSet<T = ObjectId> = HashSet<T, ShaHashBuilder>;
