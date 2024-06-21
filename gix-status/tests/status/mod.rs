mod index_as_worktree;
mod index_as_worktree_with_renames;

pub fn fixture_path(name: &str) -> std::path::PathBuf {
    let dir = gix_testtools::scripted_fixture_read_only_standalone(std::path::Path::new(name).with_extension("sh"))
        .expect("script works");
    dir
}
