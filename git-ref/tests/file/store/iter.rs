use crate::file::{store, store_with_pack};

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
    let store = store_with_pack()?;
    assert_eq!(store.packed()?.expect("pack available").iter()?.count(), 8);
    Ok(())
}

#[test]
#[ignore]
fn iter_loose() {
    let store = store().unwrap();
    assert_eq!(store.loose_iter().unwrap().count(), 15);
    todo!("more thorough checking of return values")
}

#[test]
#[ignore]
fn iter_looose_with_broken_refs() {}
