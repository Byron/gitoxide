#[cfg(not(feature = "interrupt"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("Needs 'interrupt' feature toggle to be enabled");
}

#[cfg(feature = "interrupt")]
fn main() -> anyhow::Result<()> {
    {
        // SAFETY: The closure doesn't use mutexes or memory allocation, so it should be safe to call from a signal handler.
        let _deregister_on_drop = unsafe { gix::interrupt::init_handler(1, || {})?.auto_deregister() };
    }
    eprintln!("About to emit the first term signal, which acts just like a normal one");
    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    unreachable!("the above aborts");
}
