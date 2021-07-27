#[test]
#[ignore]
fn create_a_new_packed_ref_file_with_a_single_edit() {
    todo!("use file::Store::packed_transaction()")
}

#[cfg(test)]
mod delete {

    #[test]
    #[ignore]
    fn unavailable_packed_ref_file_will_not_be_created_on_deletion() {}

    #[test]
    #[ignore]
    fn no_failure_if_target_ref_does_not_exist() {
        todo!("also check that no edits are actually performed")
    }

    #[test]
    #[ignore]
    fn failure_if_target_must_exist_but_does_not() {}

    #[test]
    #[ignore]
    fn failure_if_target_does_not_have_desired_value() {}

    #[test]
    #[ignore]
    fn all_contained_references_deletes_the_packed_ref_file_too() {}
}
