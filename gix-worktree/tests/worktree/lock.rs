use std::{fs, io};

use bstr::ByteSlice;
use gix_testtools::tempfile::tempdir;

#[test]
fn locks_updates_and_unlocks_with_byte_oriented_reasons() -> gix_error::TestResult {
    let tmp = tempdir()?;
    let locked = tmp.path().join("locked");
    for (reason, expected) in [
        (b"keep while offline".as_slice(), b"keep while offline\n".as_slice()),
        (b"", b"\n"),
        (b"already terminated\n", b"already terminated\n"),
        (b"preserve\nmultiple lines\n\n", b"preserve\nmultiple lines\n\n"),
        (b"non-UTF8 \xff", b"non-UTF8 \xff\n"),
    ] {
        gix_worktree::set_lock(tmp.path(), Some(reason.as_bstr()))?;
        assert_eq!(
            fs::read(&locked)?,
            expected,
            "the reason is preserved, with a trailing newline appended only when needed"
        );
    }

    gix_worktree::set_lock(tmp.path(), None)?;
    assert!(!locked.exists(), "unlocking removes the lock marker");
    gix_worktree::set_lock(tmp.path(), None)?;
    assert!(!locked.exists(), "unlocking an unlocked worktree is idempotent");
    Ok(())
}

#[test]
fn missing_administrative_directories_are_not_created() -> gix_error::TestResult {
    let tmp = tempdir()?;
    let git_dir = tmp.path().join("missing");
    let err = gix_worktree::set_lock(&git_dir, Some(b"reason".as_bstr()))
        .expect_err("locking requires an existing administrative directory");
    assert_eq!(err.kind(), io::ErrorKind::NotFound, "the native error is preserved");
    gix_worktree::set_lock(&git_dir, None)?;
    assert!(
        !git_dir.exists(),
        "neither operation creates administrative directories"
    );
    Ok(())
}

#[test]
fn lock_and_unlock_preserve_errors_other_than_missing_files() -> gix_error::TestResult {
    let tmp = tempdir()?;
    let locked = tmp.path().join("locked");
    fs::create_dir(&locked)?;
    for reason in [Some(b"reason".as_bstr()), None] {
        let err = gix_worktree::set_lock(tmp.path(), reason)
            .expect_err("a directory at the lock path prevents changing the lock");
        assert_ne!(
            err.kind(),
            io::ErrorKind::NotFound,
            "only absent lock files are ignored"
        );
        assert!(locked.is_dir(), "a directory at the lock path is never removed");
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn locking_a_fifo_fails_without_waiting_for_a_reader() -> gix_error::TestResult {
    use std::{os::unix::fs::FileTypeExt, sync::mpsc, time::Duration};

    // The isolated process can exit after a timeout even if its worker is stuck opening the FIFO.
    if gix_testtools::run_in_isolated_process()? {
        return Ok(());
    }
    let tmp = tempdir()?;
    gix_testtools::invoke_bash(tmp.path(), "mkfifo locked");
    let git_dir = tmp.path().to_owned();
    let (send, receive) = mpsc::channel();
    let worker = std::thread::spawn(move || {
        let _ = send.send(gix_worktree::set_lock(&git_dir, Some(b"reason".as_bstr())));
    });
    let result = receive
        .recv_timeout(Duration::from_secs(5))
        .expect("locking a FIFO must return without waiting for a reader");
    worker.join().expect("the lock worker completed without panicking");
    assert_eq!(
        result.expect_err("a FIFO cannot contain a lock reason").kind(),
        io::ErrorKind::InvalidInput,
        "non-file lock paths are rejected before opening them"
    );
    assert!(
        tmp.path().join("locked").symlink_metadata()?.file_type().is_fifo(),
        "rejecting the FIFO leaves it in place"
    );
    Ok(())
}

#[cfg(any(unix, windows))]
#[test]
fn lock_does_not_write_through_symlinks_and_unlock_removes_only_the_link() -> gix_error::TestResult {
    let tmp = gix_testtools::scripted_fixture_writable_with_args(
        "make_worktree_lock_symlinks.sh",
        None::<String>,
        gix_testtools::Creation::Execute,
    )?;
    if !gix_testtools::fixture_has_symlinks(tmp.path())? {
        return Ok(());
    }
    for (case, existing_target) in [("live", true), ("dangling", false)] {
        let dir = tmp.path().join(case);
        let git_dir = dir.join("git-dir");
        let locked = git_dir.join("locked");
        let target = dir.join("target");
        assert!(
            locked.symlink_metadata()?.file_type().is_symlink(),
            "the fixture lock must be a symlink, not a copied regular file"
        );

        gix_worktree::set_lock(&git_dir, Some(b"reason".as_bstr()))
            .expect_err("locking must not overwrite a live or dangling symlink target");
        assert!(
            locked.symlink_metadata()?.file_type().is_symlink(),
            "rejecting a symlink leaves the existing lock in place"
        );
        gix_worktree::set_lock(&git_dir, None)?;
        assert!(
            locked.symlink_metadata().is_err(),
            "unlocking removes the symlink itself"
        );
        if existing_target {
            assert_eq!(fs::read(&target)?, b"keep", "the target contents are preserved");
        } else {
            assert!(!target.exists(), "a dangling target is never created");
        }
    }
    Ok(())
}
