#![allow(unused)]
use git_features::threading::OwnShared;
use git_odb::general;
use git_testtools::fixture_path;

fn db() -> git_odb::Handle {
    git_odb::at(fixture_path("objects")).expect("valid object path")
}

#[test]
fn basics() {
    let handle = db();
}
