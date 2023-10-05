//! Hash functions and hash utilities
//!
//! With the `fast-sha1` feature, the `Sha1` hash type will use a more elaborate implementation utilizing hardware support
//! in case it is available. Otherwise the `rustsha1` feature should be set. `fast-sha1` will take precedence.
//! Otherwise, a minimal yet performant implementation is used instead for a decent trade-off between compile times and run-time performance.
#[cfg(all(feature = "rustsha1", not(feature = "fast-sha1")))]
mod _impl {
    use super::Sha1Digest;

    /// A implementation of the Sha1 hash, which can be used once.
    #[derive(Default, Clone)]
    pub struct Sha1(sha1_smol::Sha1);

    impl Sha1 {
        /// Digest the given `bytes`.
        pub fn update(&mut self, bytes: &[u8]) {
            self.0.update(bytes)
        }
        /// Finalize the hash and produce a digest.
        pub fn digest(self) -> Sha1Digest {
            self.0.digest().bytes()
        }
    }
}

/// A 20 bytes digest produced by a [`Sha1`] hash implementation.
#[cfg(any(feature = "fast-sha1", feature = "rustsha1"))]
pub type Sha1Digest = [u8; 20];

#[cfg(feature = "fast-sha1")]
mod _impl {
    use sha1::Digest;

    use super::Sha1Digest;

    /// A implementation of the Sha1 hash, which can be used once.
    #[derive(Default, Clone)]
    pub struct Sha1(sha1::Sha1);

    impl Sha1 {
        /// Digest the given `bytes`.
        pub fn update(&mut self, bytes: &[u8]) {
            self.0.update(bytes)
        }
        /// Finalize the hash and produce a digest.
        pub fn digest(self) -> Sha1Digest {
            self.0.finalize().into()
        }
    }
}

#[cfg(any(feature = "rustsha1", feature = "fast-sha1"))]
pub use _impl::Sha1;

/// Compute a CRC32 hash from the given `bytes`, returning the CRC32 hash.
///
/// When calling this function for the first time, `previous_value` should be `0`. Otherwise it
/// should be the previous return value of this function to provide a hash of multiple sequential
/// chunks of `bytes`.
#[cfg(feature = "crc32")]
pub fn crc32_update(previous_value: u32, bytes: &[u8]) -> u32 {
    let mut h = crc32fast::Hasher::new_with_initial(previous_value);
    h.update(bytes);
    h.finalize()
}

/// Compute a CRC32 value of the given input `bytes`.
///
/// In case multiple chunks of `bytes` are present, one should use [`crc32_update()`] instead.
#[cfg(feature = "crc32")]
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut h = crc32fast::Hasher::new();
    h.update(bytes);
    h.finalize()
}

/// Produce a hasher suitable for the given kind of hash.
#[cfg(any(feature = "rustsha1", feature = "fast-sha1"))]
pub fn hasher(kind: gix_hash::Kind) -> Sha1 {
    match kind {
        gix_hash::Kind::Sha1 => Sha1::default(),
    }
}

/// Compute the hash of `kind` for the bytes in the file at `path`, hashing only the first `num_bytes_from_start`
/// while initializing and calling `progress`.
///
/// `num_bytes_from_start` is useful to avoid reading trailing hashes, which are never part of the hash itself,
/// denoting the amount of bytes to hash starting from the beginning of the file.
///
/// # Note
///
/// * Only available with the `gix-object` feature enabled due to usage of the [`gix_hash::Kind`] enum and the
///   [`gix_hash::ObjectId`] return value.
/// * [Interrupts][crate::interrupt] are supported.
#[cfg(all(feature = "progress", any(feature = "rustsha1", feature = "fast-sha1")))]
pub fn bytes_of_file(
    path: &std::path::Path,
    num_bytes_from_start: u64,
    kind: gix_hash::Kind,
    progress: &mut dyn crate::progress::Progress,
    should_interrupt: &std::sync::atomic::AtomicBool,
) -> std::io::Result<gix_hash::ObjectId> {
    bytes(
        &mut std::fs::File::open(path)?,
        num_bytes_from_start,
        kind,
        progress,
        should_interrupt,
    )
}

/// Similar to [`bytes_of_file`], but operates on a stream of bytes.
#[cfg(all(feature = "progress", any(feature = "rustsha1", feature = "fast-sha1")))]
pub fn bytes(
    read: &mut dyn std::io::Read,
    num_bytes_from_start: u64,
    kind: gix_hash::Kind,
    progress: &mut dyn crate::progress::Progress,
    should_interrupt: &std::sync::atomic::AtomicBool,
) -> std::io::Result<gix_hash::ObjectId> {
    bytes_with_hasher(read, num_bytes_from_start, hasher(kind), progress, should_interrupt)
}

/// Similar to [`bytes()`], but takes a `hasher` instead of a hash kind.
#[cfg(all(feature = "progress", any(feature = "rustsha1", feature = "fast-sha1")))]
pub fn bytes_with_hasher(
    read: &mut dyn std::io::Read,
    num_bytes_from_start: u64,
    mut hasher: Sha1,
    progress: &mut dyn crate::progress::Progress,
    should_interrupt: &std::sync::atomic::AtomicBool,
) -> std::io::Result<gix_hash::ObjectId> {
    let start = std::time::Instant::now();
    // init progress before the possibility for failure, as convenience in case people want to recover
    progress.init(
        Some(num_bytes_from_start as prodash::progress::Step),
        crate::progress::bytes(),
    );

    const BUF_SIZE: usize = u16::MAX as usize;
    let mut buf = [0u8; BUF_SIZE];
    let mut bytes_left = num_bytes_from_start;

    while bytes_left > 0 {
        let out = &mut buf[..BUF_SIZE.min(bytes_left as usize)];
        read.read_exact(out)?;
        bytes_left -= out.len() as u64;
        progress.inc_by(out.len());
        hasher.update(out);
        if should_interrupt.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Interrupted"));
        }
    }

    let id = gix_hash::ObjectId::from(hasher.digest());
    progress.show_throughput(start);
    Ok(id)
}

#[cfg(any(feature = "rustsha1", feature = "fast-sha1"))]
mod write {
    use crate::hash::Sha1;

    /// A utility to automatically generate a hash while writing into an inner writer.
    pub struct Write<T> {
        /// The hash implementation.
        pub hash: Sha1,
        /// The inner writer.
        pub inner: T,
    }

    impl<T> std::io::Write for Write<T>
    where
        T: std::io::Write,
    {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let written = self.inner.write(buf)?;
            self.hash.update(&buf[..written]);
            Ok(written)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.inner.flush()
        }
    }

    impl<T> Write<T>
    where
        T: std::io::Write,
    {
        /// Create a new hash writer which hashes all bytes written to `inner` with a hash of `kind`.
        pub fn new(inner: T, object_hash: gix_hash::Kind) -> Self {
            match object_hash {
                gix_hash::Kind::Sha1 => Write {
                    inner,
                    hash: Sha1::default(),
                },
            }
        }
    }
}
#[cfg(any(feature = "rustsha1", feature = "fast-sha1"))]
pub use write::Write;
