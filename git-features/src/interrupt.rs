//! Process-global interrupt handling
//!
//! This module contains facilities to globally request an interrupt, which will cause supporting computations to
//! abort once it is observed.
//! Such checks for interrupts are provided in custom implementations of various traits to transparently add interrupt
//! support to methods who wouldn't otherwise by injecting it. see [`Read`].

#[cfg(all(feature = "interrupt-handler", not(feature = "disable-interrupts")))]
mod _impl {
    use std::{
        io,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    };

    /// Initialize a signal handler to listen to SIGINT and SIGTERM and trigger our [`trigger()`][super::trigger()] that way.
    ///
    /// # Note
    ///
    /// * This implementation is available only with the **interrupt-handler** feature toggle with the **disable-interrupts** feature disabled.
    /// * It will abort the process on second press and won't inform the user about this behaviour either as we are unable to do so without
    ///   deadlocking even when trying to write to stderr directly.
    pub fn init_handler() -> io::Result<()> {
        static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
        if IS_INITIALIZED.load(Ordering::SeqCst) {
            return Err(io::Error::new(io::ErrorKind::Other, "Already initialized"));
        }
        for sig in signal_hook::consts::TERM_SIGNALS {
            // # SAFETY
            // * we only set atomics or call functions that do
            // * there is no use of the heap
            #[allow(unsafe_code)]
            unsafe {
                signal_hook::low_level::register(*sig, move || {
                    static INTERRUPT_COUNT: AtomicUsize = AtomicUsize::new(0);
                    if !super::is_triggered() {
                        INTERRUPT_COUNT.store(0, Ordering::SeqCst);
                    }
                    let msg_idx = INTERRUPT_COUNT.fetch_add(1, Ordering::SeqCst);
                    if msg_idx == 1 {
                        signal_hook::low_level::emulate_default_handler(signal_hook::consts::SIGTERM).ok();
                    }
                    super::trigger();
                })?;
            }
        }
        Ok(())
    }
}
use std::io;
#[cfg(not(feature = "disable-interrupts"))]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(any(feature = "disable-interrupts", not(feature = "interrupt-handler")))]
mod _impl {
    /// Does nothing, as the **disable-interrupts** feature is enabled while the **interrupt-handler** feature is not present.
    pub fn init_handler() {}
}
pub use _impl::init_handler;

/// A wrapper for implementors of [`std::io::Read`] or [`std::io::BufRead`] with interrupt support.
///
/// It fails a [read][`std::io::Read::read`] while an interrupt was requested.
pub struct Read<R> {
    /// The actual implementor of [`std::io::Read`] to which interrupt support will be added.
    pub inner: R,
}

impl<R> io::Read for Read<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if is_triggered() {
            return Err(io::Error::new(io::ErrorKind::Other, "interrupted by user"));
        }
        self.inner.read(buf)
    }
}

impl<R> io::BufRead for Read<R>
where
    R: io::BufRead,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

#[cfg(not(feature = "disable-interrupts"))]
static IS_INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Returns true if an interrupt is requested.
///
/// Only implemented if the **disable-interrupts** feature toggle is not present.
#[cfg(not(feature = "disable-interrupts"))]
pub fn is_triggered() -> bool {
    IS_INTERRUPTED.load(Ordering::Relaxed)
}

/// Returns always false if the **disable-interrupts** feature is present.
#[cfg(feature = "disable-interrupts")]
pub fn is_triggered() -> bool {
    false
}

/// Trigger an interrupt, signalling to those checking for [`is_triggered()`] to stop what they are doing.
///
/// # Note
/// Only effective if the **disable-interrupts** feature is **not** present.
pub fn trigger() {
    #[cfg(not(feature = "disable-interrupts"))]
    IS_INTERRUPTED.store(true, Ordering::SeqCst);
}
/// Sets the interrupt request to false, thus allowing those checking for [`is_triggered()`] to proceed.
///
/// Call this in code that is able to trigger an interrupt.
/// This may also be performed by the [`ResetOnDrop`] helper to assure the trigger state is returned
/// to its original state.
///
/// # Note
/// Only effective if the **disable-interrupts** feature is **not** present.
pub fn reset() {
    #[cfg(not(feature = "disable-interrupts"))]
    IS_INTERRUPTED.store(false, Ordering::SeqCst);
}

/// Useful if some parts of the program set the interrupt programmatically to cause others to stop, while
/// assuring the interrupt state is reset at the end of the function to avoid other side-effects.
///
/// Note that this is inherently racy and that this will only work deterministically if there is only one
/// top-level function running in a process.
pub struct ResetOnDrop {
    was_interrupted: bool,
}

impl Default for ResetOnDrop {
    fn default() -> Self {
        ResetOnDrop {
            was_interrupted: is_triggered(),
        }
    }
}

impl Drop for ResetOnDrop {
    fn drop(&mut self) {
        if self.was_interrupted {
            trigger()
        } else {
            reset()
        }
    }
}
