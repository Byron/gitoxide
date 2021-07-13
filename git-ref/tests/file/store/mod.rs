mod find_one;
mod reflog;
mod iter {
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
    fn packed_file() -> crate::Result {
        let store = store_with_pack()?;
        assert_eq!(store.packed()?.expect("pack available").iter()?.count(), 8);
        Ok(())
    }
}
