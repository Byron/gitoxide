#![allow(unused)]
use git_features::threading::OwnShared;
use git_odb::general;
use git_testtools::fixture_path;

fn db() -> git_odb::Store {
    todo!()
    // general::Store::at(fixture_path("objects")).expect("valid object path")
}
