pub mod iter;
mod find_one {
    use crate::file::store_with_packed_refs;

    #[test]
    #[ignore]
    fn all_iterable_refs_can_be_found() {
        let store = store_with_packed_refs().unwrap();
        let packed_refs = store.packed().unwrap().expect("packed-refs exist");

        for reference in packed_refs.iter().unwrap() {
            let reference = reference.unwrap();
            let found = packed_refs
                .find(reference.full_name)
                .unwrap()
                .expect("reference exists");
            assert_eq!(reference, found, "both refs are exactly the same");
            let found = packed_refs.find_existing(reference.full_name).unwrap();
            assert_eq!(reference, found);
        }
    }

    #[test]
    #[ignore]
    fn all_iterable_refs_are_found_even_on_initially_unsorted_buffers() {}
}
