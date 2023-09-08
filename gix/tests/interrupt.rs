#[cfg(feature = "interrupt")]
mod needs_feature {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use signal_hook::consts::SIGTERM;

    #[test]
    fn multi_registration() -> gix_testtools::Result {
        static V1: AtomicUsize = AtomicUsize::new(0);
        static V2: AtomicBool = AtomicBool::new(false);

        let reg1 = gix::interrupt::init_handler(3, || {
            V1.fetch_add(1, Ordering::SeqCst);
        })
        .expect("succeeds");
        assert!(!gix::interrupt::is_triggered());
        assert_eq!(V1.load(Ordering::Relaxed), 0);
        let reg2 =
            gix::interrupt::init_handler(2, || V2.store(true, Ordering::SeqCst)).expect("multi-initialization is OK");
        assert!(!V2.load(Ordering::Relaxed));

        signal_hook::low_level::raise(SIGTERM).expect("signal can be raised");
        assert!(gix::interrupt::is_triggered(), "this happens automatically");
        assert_eq!(V1.load(Ordering::Relaxed), 1, "the first trigger is invoked");
        assert!(!V2.load(Ordering::Relaxed), "the second trigger was ignored");

        reg1.deregister()?;
        signal_hook::low_level::raise(SIGTERM).expect("signal can be raised");
        assert_eq!(V1.load(Ordering::Relaxed), 2, "the first trigger is still invoked");

        assert!(gix::interrupt::is_triggered(), "this happens automatically");
        // now the registration is actually removed.
        reg2.with_reset(true).deregister()?;
        assert!(
            !gix::interrupt::is_triggered(),
            "the deregistration succeeded and this is an optional side-effect"
        );

        let reg1 = gix::interrupt::init_handler(3, || {
            V1.fetch_add(1, Ordering::SeqCst);
        })
        .expect("succeeds");
        assert_eq!(V1.load(Ordering::Relaxed), 2, "nothing changed yet");
        let reg2 =
            gix::interrupt::init_handler(2, || V2.store(true, Ordering::SeqCst)).expect("multi-initialization is OK");
        assert!(!V2.load(Ordering::Relaxed));

        signal_hook::low_level::raise(SIGTERM).expect("signal can be raised");
        assert_eq!(V1.load(Ordering::Relaxed), 3, "the first trigger is invoked");
        assert!(!V2.load(Ordering::Relaxed), "the second trigger was ignored");

        reg2.auto_deregister();
        reg1.with_reset(true).auto_deregister();

        assert!(
            !gix::interrupt::is_triggered(),
            "the deregistration succeeded and this is an optional side-effect"
        );

        Ok(())
    }
}
