use crate::file::store_with_packed_refs;
use crate::packed::write_packed_refs_with;

#[test]
fn sorted_buffer_works() {
    let store = store_with_packed_refs().unwrap();
    store.packed().unwrap();
}

#[test]
fn empty_buffers_cannot_be_opened() -> crate::Result {
    let (_keep, path) = write_packed_refs_with(&[])?;
    assert!(
        git_ref::packed::Buffer::open(path, 512).is_err(),
        "they probably count as unsorted which isn't allowed"
    );
    Ok(())
}

#[test]
fn unsorted_buffers_cannot_currently_be_opened() -> crate::Result {
    // these are legacy and could be transformed on the fly, see https://github.com/git/git/blob/master/refs/packed-backend.c#L320:L320
    for (packed_refs_data, memmap_cutoff) in &[
        (&b"# pack-refs with: peeled fully-peeled \nsomething else"[..], 20u64),
        (&b"starts with a bogus record, not a header anyway"[..], 32 * 1024),
    ] {
        let (_keep, path) = write_packed_refs_with(packed_refs_data)?;

        match git_ref::packed::Buffer::open(path, *memmap_cutoff) {
            Ok(_) => unreachable!("unsorted buffers can't be opened"),
            Err(err) => assert_eq!(
                err.to_string(),
                "The packed-refs file did not have a header or wasn't sorted."
            ),
        }
    }
    Ok(())
}
