use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Clone)]
pub enum OwnedOrStaticAtomicBool {
    Owned {
        flag: Arc<AtomicBool>,
        #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
        private: bool,
    },
    Shared(&'static AtomicBool),
}

impl Default for OwnedOrStaticAtomicBool {
    fn default() -> Self {
        OwnedOrStaticAtomicBool::Owned {
            flag: Arc::new(AtomicBool::default()),
            private: true,
        }
    }
}

impl Deref for OwnedOrStaticAtomicBool {
    type Target = std::sync::atomic::AtomicBool;

    fn deref(&self) -> &Self::Target {
        match self {
            OwnedOrStaticAtomicBool::Owned { flag, .. } => flag,
            OwnedOrStaticAtomicBool::Shared(flag) => flag,
        }
    }
}

impl From<&'static AtomicBool> for OwnedOrStaticAtomicBool {
    fn from(value: &'static AtomicBool) -> Self {
        OwnedOrStaticAtomicBool::Shared(value)
    }
}

impl<'a> From<&'a Arc<AtomicBool>> for OwnedOrStaticAtomicBool {
    fn from(value: &'a Arc<AtomicBool>) -> Self {
        OwnedOrStaticAtomicBool::Owned {
            flag: value.clone(),
            private: false,
        }
    }
}

impl From<Arc<AtomicBool>> for OwnedOrStaticAtomicBool {
    fn from(flag: Arc<AtomicBool>) -> Self {
        OwnedOrStaticAtomicBool::Owned { flag, private: false }
    }
}
#[cfg(feature = "parallel")]
pub fn parallel_iter_drop<T, U>(
    mut rx_and_join: Option<(std::sync::mpsc::Receiver<T>, std::thread::JoinHandle<U>)>,
    should_interrupt: &OwnedOrStaticAtomicBool,
) {
    let Some((rx, handle)) = rx_and_join.take() else {
        return;
    };
    let prev = should_interrupt.swap(true, std::sync::atomic::Ordering::Relaxed);
    let undo = match &should_interrupt {
        OwnedOrStaticAtomicBool::Shared(flag) => *flag,
        OwnedOrStaticAtomicBool::Owned { flag, private: false } => flag.as_ref(),
        OwnedOrStaticAtomicBool::Owned { private: true, .. } => {
            // Leak the handle to let it shut down in the background, so drop returns more quickly.
            drop((rx, handle));
            return;
        }
    };
    // Wait until there is time to respond before we undo the change.
    handle.join().ok();
    undo.fetch_update(
        std::sync::atomic::Ordering::SeqCst,
        std::sync::atomic::Ordering::SeqCst,
        |current| current.then_some(prev),
    )
    .ok();
}
