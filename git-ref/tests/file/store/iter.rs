use crate::file::{store, store_with_packed_refs};
use std::path::PathBuf;

#[test]
fn no_packed_available_thus_no_iteration_possible() -> crate::Result {
    let store_without_packed = store()?;
    assert!(
        store_without_packed.packed()?.is_none(),
        "there is no packed refs in this store"
    );
    Ok(())
}

#[test]
fn packed_file_iter() -> crate::Result {
    let store = store_with_packed_refs()?;
    assert_eq!(store.packed()?.expect("pack available").iter()?.count(), 8);
    Ok(())
}

#[test]
fn iter_loose_with_broken_refs() {
    let store = store().unwrap();

    let mut actual: Vec<_> = store.loose_iter().unwrap().collect();
    actual.sort_by_key(|r| r.is_err());
    let first_error = actual
        .iter()
        .enumerate()
        .find_map(|(idx, r)| if r.is_err() { Some(idx) } else { None })
        .expect("there is an error");

    assert_eq!(actual.len(), 15);
    assert_eq!(
        first_error, 14,
        "there is exactly one invalid item, and it didn't abort the iterator most importantly"
    );
    assert_eq!(
        actual[first_error].as_ref().expect_err("unparseable ref").to_string(),
        "The reference at 'refs/broken' could not be instantiated"
    );
    let mut ref_paths: Vec<_> = actual
        .drain(..first_error)
        .filter_map(|e| e.ok().map(|e| e.into_relative_path()))
        .collect();
    ref_paths.sort();

    assert_eq!(
        ref_paths,
        vec![
            "d1",
            "heads/d1",
            "heads/dt1",
            "heads/main",
            "heads/multi-link-target1",
            "loop-a",
            "loop-b",
            "multi-link",
            "remotes/origin/HEAD",
            "remotes/origin/main",
            "remotes/origin/multi-link-target3",
            "tags/dt1",
            "tags/multi-link-target2",
            "tags/t1"
        ]
        .into_iter()
        .map(|p| PathBuf::from(format!("refs/{}", p)))
        .collect::<Vec<_>>(),
        "all paths are as expected"
    );
}
