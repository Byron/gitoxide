use std::mem::MaybeUninit;

use git_hash::ObjectId;

pub trait ExtractHash {
    fn trucated_hash(&self) -> &[u8; 8];

    #[inline(always)]
    fn aligned_hash(&self) -> u64 {
        let mut hash = MaybeUninit::uninit();
        let mut dst = hash.as_mut_ptr() as *mut [u8; 8];
        unsafe {
            dst.copy_from_nonoverlapping(self.trucated_hash() as *const _, 1);
        }
        hash.assume_init()
    }
}

impl ExtractHash for ObjectId {
    fn trucated_hash(&self) -> &[u8; 8] {
        match self {
            ObjectId::Sha1(slice) => {
                // bounds check is always elided as the size is a compile time constant
                assert!(slice.len() >= 8);
                let ptr = slice.as_ptr() as *const [u8; 8];
                // safety: This is save because the pointer is always in bounds (see assert above)
                // and was created from a reference of the same lifetime
                unsafe { &*ptr }
            }
        }
    }
}
impl<T> ExtractHash for (ObjectId, T) {
    fn trucated_hash(&self) -> &[u8; 8] {
        self.0.trucated_hash()
    }
}

#[derive(Clone)]
pub struct RawObjectMap<T: ExtractHash> {
    table: hashbrown::raw::RawTable<T>,
}

impl<T: ExtractHash> RawObjectMap<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert(&mut self, val: T) {
        let hash = val.aligned_hash();
        if 
        self.table.insert(hash, val, |it| it.aligned_hash());
    }
    #[inline]
    pub fn remove(&mut self, val: T) {
        let hash = val.aligned_hash();
        self.table.insert(hash, val, |it| it.aligned_hash());
    }
}
