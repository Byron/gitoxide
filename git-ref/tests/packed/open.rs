use crate::file::store_with_packed_refs;

#[test]
fn sorted_buffer_works() {
    let store = store_with_packed_refs().unwrap();
    store.packed().unwrap();
}

#[test]
fn unsorted_buffers_cannot_currently_be_opened() {
    // these are legacy and could be transformed on the fly, see https://github.com/git/git/blob/master/refs/packed-backend.c#L320:L320
    let dir = tempfile::tempdir().unwrap();
    let packed_refs_path = dir.path().join("packed-refs");

    for (packed_refs_data, memmap_cutoff) in &[
        (&b"# pack-refs with: peeled fully-peeled \nsomething else"[..], 20u64),
        (&b"starts with a bogus record, not a header anyway"[..], 32 * 1024),
    ] {
        std::fs::write(&packed_refs_path, packed_refs_data).unwrap();

        match git_ref::packed::Buffer::open(&packed_refs_path, *memmap_cutoff) {
            Ok(_) => unreachable!("unsorted buffers can't be opened"),
            Err(err) => assert_eq!(
                err.to_string(),
                "The packed-refs file did not have a header or wasn't sorted."
            ),
        }
    }
}
