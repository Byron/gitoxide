#[cfg(not(feature = "interrupt"))]
fn main() -> anyhow::Result<()> {
    anyhow::bail!("Needs 'interrupt' feature toggle to be enabled");
}

#[cfg(feature = "interrupt")]
fn main() -> anyhow::Result<()> {
    use gix_tempfile::{AutoRemove, ContainingDirectory};
    gix::interrupt::init_handler(1, || {})?;
    eprintln!("About to emit the first term signal");
    let tempfile_path = std::path::Path::new("example-file.tmp");
    let _keep_tempfile = gix_tempfile::mark_at(tempfile_path, ContainingDirectory::Exists, AutoRemove::Tempfile)?;

    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    eprintln!(
        "Still here to showdown gracefully, our handler was triggered to kick that off. Tempfiles are still present."
    );
    assert!(tempfile_path.is_file());
    eprintln!("The next signal will abort this process but leave no tempfile nonetheless");
    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    unreachable!("the above aborts");
}
