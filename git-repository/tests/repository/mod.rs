use git_repository::Repository;

mod config;
mod object;
mod reference;
mod remote;
mod state;
mod worktree;

#[test]
fn size_in_memory() {
    let expected = [688, 696];
    assert!(
        expected.contains(&std::mem::size_of::<Repository>()),
        "size of Repository shouldn't change without us noticing, it's meant to be cloned: should have been within {:?}",
        expected
    );
}
