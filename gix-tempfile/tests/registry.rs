//! These must be run in their own module to avoid interfering with other tests.
use std::{io::Write, path::Path};

use gix_tempfile::{AutoRemove, ContainingDirectory};

fn filecount_in(path: impl AsRef<Path>) -> usize {
    std::fs::read_dir(path).expect("valid dir").count()
}

#[test]
fn cleanup_tempfiles() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let mut tempfile = gix_tempfile::new(dir.path(), ContainingDirectory::Exists, AutoRemove::Tempfile)?;
    assert_eq!(
        filecount_in(dir.path()),
        1,
        "only one tempfile exists no matter the iteration"
    );
    gix_tempfile::registry::cleanup_tempfiles();
    assert_eq!(
        filecount_in(dir.path()),
        0,
        "the signal triggers removal but won't terminate the process (anymore)"
    );
    assert!(
        tempfile.write_all(b"bogus").is_err(),
        "cannot write into a tempfile that doesn't exist in registry"
    );
    Ok(())
}
