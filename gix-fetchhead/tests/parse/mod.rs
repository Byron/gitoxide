use std::{fs::File, io::Read, env};
use crate::fixture_path;


#[test]
fn git_repo_test() -> crate::Result {
    let path = fixture_path("git");
    println!("{path:?}");
    println!("{:?}", env::current_dir());
    let mut source = File::open(&path)?;
    let mut s = String::new();
    match source.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {path:?}: {why}"),
        Ok(_) => print!("{path:?} contains:\n{s}"),
    }
    Ok(())
}
