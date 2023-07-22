use crate::util::named_repo;

mod diff;

#[test]
fn find_entry() -> crate::Result {
    let repo = named_repo("make_basic_repo.sh")?;
    let tree = repo.head_commit()?.tree()?;
    assert_eq!(tree.find_entry("this").expect("present").filename(), "this");

    assert!(tree.find_entry("not there").is_none());
    Ok(())
}
