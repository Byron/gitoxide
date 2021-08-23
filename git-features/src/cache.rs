#[cfg(feature = "cache-efficiency-debug")]
mod impl_ {
    /// A helper to collect useful information about cache efficiency.
    pub struct Debug {
        owner: String,
        hits: usize,
        puts: usize,
        misses: usize,
    }

    impl Debug {
        /// Create a new instance
        #[inline]
        pub fn new(owner: impl Into<String>) -> Self {
            Debug {
                owner: owner.into(),
                hits: 0,
                puts: 0,
                misses: 0,
            }
        }
        /// Count cache insertions
        #[inline]
        pub fn put(&mut self) {
            self.puts += 1;
        }
        /// Count hits
        #[inline]
        pub fn hit(&mut self) {
            self.hits += 1;
        }
        /// Count misses
        #[inline]
        pub fn miss(&mut self) {
            self.misses += 1;
        }
    }

    impl Drop for Debug {
        fn drop(&mut self) {
            let hits = self.hits;
            let misses = self.misses;
            let ratio = hits as f32 / misses as f32;
            eprintln!(
                "{}[{:0x}]: {} / {} (hits/misses) = {:.02}%, puts = {}",
                self.owner, self as *const _ as usize, hits, misses, ratio, self.puts
            );
        }
    }
}
#[cfg(not(feature = "cache-efficiency-debug"))]
mod impl_ {
    /// The disabled, zero size do-nothing equivalent
    pub struct Debug;

    impl Debug {
        /// Create a new instance
        #[inline]
        pub fn new(_owner: impl Into<String>) -> Self {
            Debug
        }
        /// noop
        pub fn put(&mut self) {}
        /// noop
        pub fn hit(&mut self) {}
        /// noop
        pub fn miss(&mut self) {}
    }
}

pub use impl_::Debug;
