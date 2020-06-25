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

    use std::io::{Result, Write};
    impl Write for Sha1 {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.update(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
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

    use std::io::{Result, Write};
    impl Write for Sha1 {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.update(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
}

pub use _impl::Sha1;
