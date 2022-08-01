use git_repository::Repository;

mod config;
mod object;
mod reference;
mod remote;
mod state;
mod worktree;

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Repository>(),
        696,
        "size of Repository shouldn't change without us noticing, it's meant to be cloned"
    );
}
