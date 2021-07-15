use crate::file::store_with_packed_refs;
use git_ref::PartialName;
use std::convert::TryFrom;

#[test]
fn a_lock_file_would_not_be_a_valid_partial_name() {
    // doesn't really belong here but want to make sure refname validation works as expected.
    let err = PartialName::try_from("heads/hello.lock").expect_err("this should fail");
    assert_eq!(err.to_string(), "The reference name 'heads/hello.lock' is invalid");
}

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
