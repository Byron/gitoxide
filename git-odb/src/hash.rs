#[cfg(feature = "minimal-sha1")]
mod _impl {
    #[derive(Default)]
    pub struct Sha1(sha1::Sha1);

    impl Sha1 {
        pub fn update(&mut self, d: &[u8]) {
            self.0.update(d)
        }
        pub fn digest(self) -> git_object::Id {
            git_object::Id(self.0.digest().bytes())
        }
    }
}

#[cfg(all(feature = "fast-sha1", not(feature = "minimal-sha1")))]
mod _impl {
    use fastsha1::Digest;
    #[derive(Default)]
    pub struct Sha1(fastsha1::Sha1);

    impl Sha1 {
        pub fn update(&mut self, d: &[u8]) {
            self.0.update(d)
        }
        pub fn digest(self) -> git_object::Id {
            git_object::Id(self.0.finalize().into())
        }
    }
}

pub use _impl::Sha1;

pub use crc::crc32::checksum_ieee as crc32;
