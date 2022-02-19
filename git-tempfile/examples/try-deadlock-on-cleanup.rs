use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use git_tempfile::handle::Writable;
use git_tempfile::{AutoRemove, ContainingDirectory, Handle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secs_to_run: usize = std::env::args()
        .nth(1)
        .ok_or_else(|| "the first argument is the amount of seconds to run")?
        .parse()?;
    let suspected_dashmap_block_size = 64;
    let tmp = tempfile::TempDir::new()?;
    let tempfiles_created = Arc::new(AtomicUsize::default());
    let tempfiles_registry_locked = Arc::new(AtomicUsize::default());
    let signal_raised = Arc::new(AtomicUsize::default());
    git_tempfile::setup(git_tempfile::SignalHandlerMode::DeleteTempfilesOnTermination);

    for tid in 0..suspected_dashmap_block_size {
        std::thread::spawn({
            let tmp = tmp.path().to_owned();
            let tempfiles_created = Arc::clone(&tempfiles_created);
            let tempfiles_registry_locked = Arc::clone(&tempfiles_registry_locked);
            move || {
                let mut tfile = tempfile_for_thread_or_panic(tid, &tmp, &tempfiles_created);
                // Cause it to be repeatedly fetched from the registry for writing, causing high contention on the write lock
                // of the dashmap block it should be in.
                loop {
                    if tfile
                        .with_mut(|_| {
                            tempfiles_registry_locked.fetch_add(1, Ordering::SeqCst);
                        })
                        .is_err()
                    {
                        // The cleanup handler runs continuously, so we create a new file once our current one is removed
                        // This test is clearly limited by IOPS
                        tfile = tempfile_for_thread_or_panic(tid, &tmp, &tempfiles_created);
                    }
                }
            }
        });
    }

    std::thread::spawn({
        let tempfiles_created = Arc::clone(&tempfiles_created);
        let tempfiles_registry_locked = Arc::clone(&tempfiles_registry_locked);
        let signal_raised = Arc::clone(&signal_raised);
        move || {
            eprintln!(
                "If a deadlock occours tempfiles will be left in '{}'",
                tmp.path().display()
            );
            for ttl in (1..=secs_to_run).rev() {
                std::thread::sleep(Duration::from_secs(1));
                eprintln!(
                    "TTL = {}s unless we are deadlocked (tempfiles created = {}, locks acquired =  {}, num times cleaned up = {})",
                    ttl,
                    tempfiles_created.load(Ordering::SeqCst),
                    tempfiles_registry_locked.load(Ordering::SeqCst),
                    signal_raised.load(Ordering::SeqCst)
                );
            }
            if let Err(err) = tmp.close() {
                eprintln!("Error when removing tempdir: {}", err);
            }
            eprintln!(
                "OK: survived {}s without deadlock with {} tempfiles created, lock obtained {} times, cleanup handler ran {} times",
                secs_to_run,
                tempfiles_created.load(Ordering::SeqCst),
                tempfiles_registry_locked.load(Ordering::SeqCst),
                signal_raised.load(Ordering::SeqCst)
            );
            std::process::abort();
        }
    });

    // Repeatedly cause the tempfile cleanup to run to cause a deadlock
    loop {
        signal_hook::low_level::raise(signal_hook::consts::SIGINT)?;
        signal_raised.fetch_add(1, Ordering::SeqCst);
    }
}

fn tempfile_for_thread_or_panic(tid: i32, tmp: &Path, count: &AtomicUsize) -> Handle<Writable> {
    let res = git_tempfile::writable_at(
        tmp.join(format!("thread-{}", tid)),
        ContainingDirectory::Exists,
        AutoRemove::Tempfile,
    )
    .unwrap();
    count.fetch_add(1, Ordering::SeqCst);
    res
}
