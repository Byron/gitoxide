use crate::file::store;

#[test]
fn set_packed_buffer_mmap_threshold() -> crate::Result {
    let mut store = store()?;
    let prev = store.set_packed_buffer_mmap_threshold(0);
    if cfg!(windows) {
        assert_eq!(
            prev,
            u64::MAX,
            "on windows mmap are deactivated as otherwise we can't change packed-refs while it's mapped"
        );
    } else {
        assert_eq!(prev, 32 * 1024, "the default is the value that Git uses as well");
    }
    assert_eq!(
        store.set_packed_buffer_mmap_threshold(0),
        0,
        "it actually sets the value"
    );
    Ok(())
}
