use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use git_tempfile::{AutoRemove, ContainingDirectory};

fn main() -> std::io::Result<()> {
    let filepath = PathBuf::new().join("tempfile.ext");
    let _tempfile = git_tempfile::mark_at(&filepath, ContainingDirectory::Exists, AutoRemove::Tempfile)?;
    assert!(filepath.is_file(), "a tempfile was created");

    writeln!(stdout(), "{}", filepath.display())?;
    stdout().flush()?;

    signal_hook::low_level::raise(signal_hook::consts::SIGTERM)?;
    unreachable!("the above line aborts the process, and prevents destructors from running. The tempfile will go away nonetheless");
}
