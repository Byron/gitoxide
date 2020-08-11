use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(feature = "interrupt-handler")]
mod _impl {
    use once_cell::sync::OnceCell;

    pub fn init_interrupt_handler() {
        static INIT_INTERRUPT_ONCE: OnceCell<()> = OnceCell::new();
        INIT_INTERRUPT_ONCE.get_or_init(|| {
            ctrlc::set_handler(|| {
                super::IS_INTERRUPTED.store(true, std::sync::atomic::Ordering::Relaxed);
            })
            .expect("it is up to the application to ensure only one interrupt handler is installed");
        });
    }
}
#[cfg(not(feature = "interrupt-handler"))]
mod _impl {
    pub fn init_interrupt_handler() {}
}
pub use _impl::init_interrupt_handler;

pub struct Read<R> {
    pub inner: R,
}

impl<R> io::Read for Read<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if is_interrupted() {
            return Err(io::Error::new(io::ErrorKind::Other, "interrupted by user"));
        }
        self.inner.read(buf)
    }
}

static IS_INTERRUPTED: AtomicBool = AtomicBool::new(false);

pub fn is_interrupted() -> bool {
    IS_INTERRUPTED.load(Ordering::Relaxed)
}
pub fn interrupt() {
    IS_INTERRUPTED.store(true, Ordering::Relaxed);
}
pub fn uninterrupt() {
    IS_INTERRUPTED.store(false, Ordering::Relaxed);
}
