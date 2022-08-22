use git_repository::Repository;

mod config;
mod object;
mod open;
mod reference;
mod remote;
mod state;
mod worktree;

#[test]
fn size_in_memory() {
    let expected = [744, 760];
    let actual_size = std::mem::size_of::<Repository>();
    assert!(
        expected.contains(&actual_size),
        "size of Repository shouldn't change without us noticing, it's meant to be cloned: should have been within {:?}, was {}",
        expected, actual_size
    );
}
