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

pub fn crc32_update(previous_value: u32, bytes: &[u8]) -> u32 {
    crc::crc32::update(previous_value, &crc::crc32::IEEE_TABLE, bytes)
}
pub use crc::crc32::checksum_ieee as crc32;

#[cfg(feature = "git-object")]
pub fn bytes_of_file(
    path: impl AsRef<std::path::Path>,
    num_bytes_from_start: usize,
    progress: &mut impl crate::progress::Progress,
) -> std::io::Result<git_object::owned::Id> {
    let mut hasher = crate::hash::Sha1::default();
    let start = std::time::Instant::now();
    // init progress before the possibility for failure, as convenience in case people want to recover
    progress.init(Some(num_bytes_from_start), crate::progress::bytes());

    let mut file = std::fs::File::open(path)?;
    use std::io::Read;
    const BUF_SIZE: usize = u16::MAX as usize;
    let mut buf = [0u8; BUF_SIZE];
    let mut bytes_left = num_bytes_from_start;

    while bytes_left > 0 {
        let out = &mut buf[..BUF_SIZE.min(bytes_left)];
        file.read_exact(out)?;
        bytes_left -= out.len();
        progress.inc_by(out.len());
        hasher.update(out);
        if crate::interrupt::is_triggered() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Interrupted"));
        }
    }

    let id = git_object::owned::Id::new_sha1(hasher.digest());
    progress.show_throughput(start);
    Ok(id)
}
