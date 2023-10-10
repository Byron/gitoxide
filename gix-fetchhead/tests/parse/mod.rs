use std::{fs::File, io::Read, env, path::PathBuf};

#[test]
fn git_repo_test() -> crate::Result {
    println!("TESTING");
    //let path = gix_testtools::fixture_path("git");
    //let path = gix_testtools::fixture_path_standalone("git");
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures/git");
    println!("{:?}", path);
    let mut source = File::open(&path)?;
    let mut s = String::new();
    match source.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {:?}: {}", path, why),
        Ok(_) => print!("{:?} contains:\n{}", path, s),
    }
    Ok(())
}
