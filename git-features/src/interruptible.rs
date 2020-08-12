use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(feature = "interrupt-handler")]
mod _impl {
    use std::{
        io,
        sync::atomic::{AtomicUsize, Ordering},
    };

    pub fn init_interrupt_handler(mut message_channel: impl io::Write + Send + 'static) {
        ctrlc::set_handler(move || {
            const MESSAGES: &[&str] = &[
                "interrupt requested", 
                "please wait…", 
                "the program will respond soon…", 
                "if the program doesn't respond quickly enough, please let us know here: https://github.com/Byron/gitoxide/issues"
            ];
            static CURRENT_MESSAGE: AtomicUsize = AtomicUsize::new(0);
            if !super::is_interrupted() {
                CURRENT_MESSAGE.store(0, Ordering::Relaxed);
            }
            let msg_idx =CURRENT_MESSAGE.fetch_add(1, Ordering::Relaxed);
            super::IS_INTERRUPTED.store(true, Ordering::Relaxed);
            writeln!(message_channel, "{}", MESSAGES[msg_idx % MESSAGES.len()]).ok();
        })
        .expect("it is up to the application to ensure only one interrupt handler is installed, and this function is called only once.")
    }
}
#[cfg(not(feature = "interrupt-handler"))]
mod _impl {
    use std::io;

    pub fn init_interrupt_handler(_message_channel: impl io::Write + Send + 'static) {}
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

/// Useful if some parts of the program set the interrupt programmatically to cause others to stop, while
/// assuring the interrupt state is reset at the end of the function to avoid other side-effects.
///
/// Note that this is inherently racy and that this will only work deterministically if there is only one
/// top-level function running in a process.
pub struct ResetInterruptOnDrop {
    was_interrupted: bool,
}

impl ResetInterruptOnDrop {
    pub fn new() -> Self {
        ResetInterruptOnDrop {
            was_interrupted: is_interrupted(),
        }
    }
}

impl Drop for ResetInterruptOnDrop {
    fn drop(&mut self) {
        if self.was_interrupted {
            interrupt()
        } else {
            uninterrupt()
        }
    }
}
