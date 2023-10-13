use std::{fs::File, io::Read, env};
use gix_fetchhead::{FetchHead, FetchHeadEntry, parse::parse};
use gix_hash::ObjectId;

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
    let test_data = FetchHead {
        entries: vec![
        FetchHeadEntry{
            head: ObjectId::from_hex(b"493f4622739e9b64f24b465b21aa85870dd9dc09").unwrap(),
            merge_status: true,
            branch: "master",
            remote: b"github.com:git/git".into(),
        },
        FetchHeadEntry{
            head: ObjectId::from_hex(b"c6bb019724237deb91ba4a9185fd04507aadeb6a").unwrap(),
            merge_status: false,
            branch: "jch",
            remote: b"github.com:git/git".into(),
        },
        FetchHeadEntry{
            head: ObjectId::from_hex(b"43c8a30d150ecede9709c1f2527c8fba92c65f40").unwrap(),
            merge_status: false,
            branch: "maint",
            remote: "github.com:git/git".into(),
        },
        FetchHeadEntry{
            head: ObjectId::from_hex(b"17973ab24622019495b7da9b5595e3852d11d27c").unwrap(),
            merge_status: false,
            branch: "next",
            remote: b"github.com:git/git".into(),
        },
        FetchHeadEntry{
            head: ObjectId::from_hex(b"382943e21bfb8c4f3dd86ce0c889643c75e2213b").unwrap(),
            merge_status: false,
            branch: "seen",
            remote: b"github.com:git/git".into(),
        },
        FetchHeadEntry{
            head: ObjectId::from_hex(b"c6657dee71fda3b9ad48317f304977c70b2af303").unwrap(),
            merge_status: false,
            branch: "todo",
            remote: "github.com:git/git".into(),
        },],
    };
    //assert_eq!(parse(path)?, test_data);
    Ok(())
}
