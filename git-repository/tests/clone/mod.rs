use crate::remote;
use git_repository as git;

#[test]
#[cfg(feature = "blocking-network-client")]
fn fetch_only_with_configuration() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let called_configure_remote = std::sync::Arc::new(std::sync::atomic::AtomicBool::default());
    let remote_name = "special";
    let mut prepare = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?
        .with_remote_name(remote_name)?
        .configure_remote({
            let called_configure_remote = called_configure_remote.clone();
            move |r| {
                called_configure_remote.store(true, std::sync::atomic::Ordering::Relaxed);
                Ok(
                    r.with_refspec("+refs/tags/*:refs/tags/*", git::remote::Direction::Fetch)
                        .expect("valid static spec"),
                )
            }
        });
    let repo = prepare.fetch_only(git::progress::Discard, &std::sync::atomic::AtomicBool::default())?;
    drop(prepare);

    assert!(
        called_configure_remote.load(std::sync::atomic::Ordering::Relaxed),
        "custom remote configuration is called"
    );
    assert_eq!(repo.remote_names().len(), 1, "only ever one remote");
    let remote = repo.find_remote(remote_name)?;
    assert_eq!(
        remote.refspecs(git::remote::Direction::Fetch).len(),
        2,
        "our added spec was stored as well"
    );

    Ok(())
}

#[test]
fn clone_and_early_persist_without_receive() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let repo = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?.persist();
    assert!(repo.is_bare(), "repo is now ours and remains");
    Ok(())
}

#[test]
fn clone_bare_into_empty_directory_and_early_drop() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    // this breaks isolation, but shouldn't be affecting the test. If so, use isolation options for opening the repo.
    let prep = git::prepare_clone_bare(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join("HEAD");
    assert!(head.is_file(), "now a bare basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}

#[test]
fn clone_into_empty_directory_and_early_drop() -> crate::Result {
    let tmp = git_testtools::tempfile::TempDir::new()?;
    let prep = git::prepare_clone(remote::repo("base").path(), tmp.path())?;
    let head = tmp.path().join(".git").join("HEAD");
    assert!(head.is_file(), "now a basic repo is present");
    drop(prep);

    assert!(!head.is_file(), "we cleanup if the clone isn't followed through");
    Ok(())
}
