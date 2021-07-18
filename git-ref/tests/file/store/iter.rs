use crate::file::{store, store_at, store_with_packed_refs};
use bstr::ByteSlice;
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
fn loose_iter_with_broken_refs() -> crate::Result {
    let store = store()?;

    let mut actual: Vec<_> = store.loose_iter()?.collect();
    assert_eq!(actual.len(), 15);
    actual.sort_by_key(|r| r.is_err());
    let first_error = actual
        .iter()
        .enumerate()
        .find_map(|(idx, r)| if r.is_err() { Some(idx) } else { None })
        .expect("there is an error");

    assert_eq!(
        first_error, 14,
        "there is exactly one invalid item, and it didn't abort the iterator most importantly"
    );
    #[cfg(not(windows))]
    let msg = "The reference at 'refs/broken' could not be instantiated";
    #[cfg(windows)]
    let msg = "The reference at 'refs\\broken' could not be instantiated";
    assert_eq!(
        actual[first_error].as_ref().expect_err("unparseable ref").to_string(),
        msg
    );
    let ref_paths: Vec<_> = actual
        .drain(..first_error)
        .filter_map(|e| e.ok().map(|e| e.into_relative_path()))
        .collect();

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
    Ok(())
}

#[test]
fn loose_iter_with_prefix_wont_allow_absolute_paths() -> crate::Result {
    let store = store()?;
    #[cfg(not(windows))]
    let abs_path = "/hello";
    #[cfg(windows)]
    let abs_path = "c:\\hello";

    match store.loose_iter_prefixed(abs_path) {
        Ok(_) => unreachable!("absolute paths aren't allowed"),
        Err(err) => assert_eq!(err.to_string(), "prefix must be a relative path, like 'refs/heads'"),
    }
    Ok(())
}

#[test]
fn loose_iter_with_prefix() -> crate::Result {
    let store = store()?;

    let actual = store
        .loose_iter_prefixed("refs/heads/")?
        .collect::<Result<Vec<_>, _>>()
        .expect("no broken ref in this subset")
        .into_iter()
        .map(|e| e.into_relative_path())
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![
            "refs/heads/d1",
            "refs/heads/dt1",
            "refs/heads/main",
            "refs/heads/multi-link-target1",
        ]
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>(),
        "all paths are as expected"
    );
    Ok(())
}

#[test]
fn overlay_iter() {
    let store = store_at("make_packed_ref_repository_for_overlay.sh").unwrap();
    let ref_names = store
        .iter(&store.packed().unwrap().expect("packed-refs"))
        .unwrap()
        .map(|r| r.map(|r| r.name().expect("valid names only").into_inner()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        ref_names,
        vec![
            b"refs/heads/main".as_bstr(),
            "refs/heads/newer-as-loose".into(),
            "refs/remotes/origin/HEAD".into(),
            "refs/remotes/origin/main".into(),
            "refs/tags/tag-object".into()
        ]
    );
}
