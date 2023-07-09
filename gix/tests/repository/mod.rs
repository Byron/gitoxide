use gix::Repository;

mod config;
mod filter;
mod object;
mod open;
mod reference;
mod remote;
mod shallow;
mod state;
mod worktree;

#[test]
fn size_in_memory() {
    let actual_size = std::mem::size_of::<Repository>();
    let limit = 1200;
    assert!(
        actual_size <= limit,
        "size of Repository shouldn't change without us noticing, it's meant to be cloned: should have been below {limit:?}, was {actual_size} (bigger on windows)"
    );
}
