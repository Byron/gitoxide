use crate::{SignalHandlerMode, REGISTER, SIGNAL_HANDLER_MODE};

/// # Safety
/// Note that Mutexes of any kind are not allowed, and so aren't allocation or deallocation of memory.
/// We are usign lock-free datastructures and sprinkle in `std::mem::forget` to avoid deallocating.
pub fn cleanup_tempfiles() {
    let current_pid = std::process::id();
    for mut tempfile in REGISTER.iter_mut() {
        if tempfile
            .as_ref()
            .map_or(false, |tf| tf.owning_process_id == current_pid)
        {
            if let Some(tempfile) = tempfile.take() {
                tempfile.drop_without_deallocation();
            }
        }
    }
}

/// On linux we can handle the actual signal as we know it.
#[cfg(not(windows))]
pub fn cleanup_tempfiles_nix(sig: &libc::siginfo_t) {
    cleanup_tempfiles();
    let restore_original_behaviour = SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour as usize;
    if SIGNAL_HANDLER_MODE.load(std::sync::atomic::Ordering::SeqCst) == restore_original_behaviour {
        signal_hook::low_level::emulate_default_handler(sig.si_signo).ok();
    }
}

/// On windows, assume sig-term and emulate sig-term unconditionally.
#[cfg(windows)]
pub fn cleanup_tempfiles_windows() {
    cleanup_tempfiles();
    let restore_original_behaviour = SignalHandlerMode::DeleteTempfilesOnTerminationAndRestoreDefaultBehaviour as usize;
    if SIGNAL_HANDLER_MODE.load(std::sync::atomic::Ordering::SeqCst) == restore_original_behaviour {
        signal_hook::low_level::emulate_default_handler(signal_hook::consts::SIGTERM).ok();
    }
}

#[cfg(test)]
mod tests {
    use crate::{AutoRemove, ContainingDirectory};
    use std::path::Path;

    fn filecount_in(path: impl AsRef<Path>) -> usize {
        std::fs::read_dir(path).expect("valid dir").count()
    }

    #[test]
    fn various_termination_signals_remove_tempfiles_unconditionally() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        for sig in signal_hook::consts::TERM_SIGNALS {
            let _tempfile = crate::new(dir.path(), ContainingDirectory::Exists, AutoRemove::Tempfile)?;
            assert_eq!(
                filecount_in(dir.path()),
                1,
                "only one tempfile exists no matter the iteration"
            );
            signal_hook::low_level::raise(*sig)?;
            assert_eq!(
                filecount_in(dir.path()),
                0,
                "the signal triggers removal but won't terminate the process (anymore)"
            );
        }
        Ok(())
    }
}
