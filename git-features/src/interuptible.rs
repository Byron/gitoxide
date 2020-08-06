#[cfg(feature = "interuptible")]
mod _impl {
    use ctrlc;
    use once_cell::sync::Lazy;
    use std::{
        io,
        sync::atomic::{AtomicBool, Ordering},
    };

    static IS_INTERRUPTED: Lazy<AtomicBool> = Lazy::new(|| {
        ctrlc::set_handler(|| {
            IS_INTERRUPTED.store(true, Ordering::Relaxed);
        })
        .ok(); // if the application has already registered a handler, it's up to them to handle interruption entirely
        AtomicBool::new(false)
    });

    pub struct Read<R> {
        pub inner: R,
    }

    impl<R> io::Read for Read<R>
    where
        R: io::Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if is_interupted() {
                return Err(io::Error::new(io::ErrorKind::Other, "interrupted by user"));
            }
            self.inner.read(buf)
        }
    }

    pub fn is_interupted() -> bool {
        IS_INTERRUPTED.load(Ordering::Relaxed)
    }

    pub fn interupt() {
        IS_INTERRUPTED.store(true, Ordering::Relaxed);
    }
}

#[cfg(not(feature = "interuptible"))]
mod _impl {
    use std::io;

    pub struct Read<R> {
        pub inner: R,
    }

    impl<R> io::Read for Read<R>
    where
        R: io::Read,
    {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.inner.read(buf)
        }
    }

    pub fn is_interrupted() -> bool {
        false
    }
}

pub use _impl::*;
