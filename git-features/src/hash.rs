#[cfg(not(feature = "fast-sha1"))]
mod _impl {
    use super::Sha1Digest;

    #[derive(Default, Clone)]
    pub struct Sha1(sha1::Sha1);

    impl Sha1 {
        pub fn update(&mut self, d: &[u8]) {
            self.0.update(d)
        }
        pub fn digest(self) -> Sha1Digest {
            self.0.digest().bytes()
        }
    }
}

pub type Sha1Digest = [u8; 20];

#[cfg(feature = "fast-sha1")]
mod _impl {
    use super::Sha1Digest;
    use fastsha1::Digest;

    #[derive(Default, Clone)]
    pub struct Sha1(fastsha1::Sha1);

    impl Sha1 {
        pub fn update(&mut self, d: &[u8]) {
            self.0.update(d)
        }
        pub fn digest(self) -> Sha1Digest {
            self.0.finalize().into()
        }
    }
}

pub use _impl::Sha1;

pub use crc::crc32::checksum_ieee as crc32;
