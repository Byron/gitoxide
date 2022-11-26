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
    let actual_size = std::mem::size_of::<Repository>();
    let limit = 1000;
    assert!(
        actual_size <= limit,
        "size of Repository shouldn't change without us noticing, it's meant to be cloned: should have been below {:?}, was {} (bigger on windows)",
        limit,
        actual_size
    );
}
