use std::path::PathBuf;

use git_pack::multi_index::File;

fn multi_index() -> (File, PathBuf) {
    let path = git_testtools::scripted_fixture_read_only("make_pack_gen_repo_multi_index.sh")
        .expect("test fixture exists")
        .join(".git/objects/pack/multi-pack-index");
    let file = git_pack::multi_index::File::at(&path).unwrap();
    (file, path)
}

mod access;

mod verify;

mod write;
