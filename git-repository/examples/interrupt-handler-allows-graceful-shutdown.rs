use std::path::Path;

use git_tempfile::{AutoRemove, ContainingDirectory};

fn main() -> anyhow::Result<()> {
    git_repository::interrupt::init_handler(|| {})?;
    eprintln!("About to emit the first term signal");
    let tempfile_path = Path::new("example-file.tmp");
    let _keep_tempfile = git_tempfile::mark_at(tempfile_path, ContainingDirectory::Exists, AutoRemove::Tempfile)?;

    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    eprintln!(
        "Still here to showdown gracefully, our handler was triggered to kick that off. Tempfiles are still present."
    );
    assert!(tempfile_path.is_file());
    eprintln!("The next signal will abort this process but leave no tempfile nonetheless");
    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    unreachable!("the above aborts");
}
