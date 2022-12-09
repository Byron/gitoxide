fn main() -> anyhow::Result<()> {
    {
        let _deregister_on_drop = git_repository::interrupt::init_handler(|| {})?.auto_deregister();
    }
    eprintln!("About to emit the first term signal, which acts just like a normal one");
    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    unreachable!("the above aborts");
}
