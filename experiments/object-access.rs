#!/usr/bin/env -S cargo eval --
//! Currently this experiment fails as `locate(…)` can't actually find objects. My guess is that
//! it can't find objects in packs for some reason because it could find some objects there were
//! probably loose, but failed right away after a `git gc`.
//! Let's see if a unit test can reproduce this too, right now this functionality is entirely untested
//! I think.
//! ```cargo
//! [dependencies]
//! atty = "0.2"
//! anyhow = "1"
//! git-odb = { version = "0.9", path = "../git-odb" }
//! git-hash = { version = "0.1", path = "../git-hash" }
//! git2 = "0.13"
//! ```
use std::io::BufRead;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    if atty::is(atty::Stream::Stdin) {
        anyhow::bail!("Need object hashes on stdin, one per line");
    }

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let hashes = stdin.lines().collect::<Result<Vec<String>, _>>()?;

    println!("{} objects", hashes.len());

    let start = Instant::now();
    let bytes = do_git2(hashes.as_slice())?;
    let elapsed = start.elapsed();
    println!("libgit2:  confirmed {} bytes in {:?}", bytes, elapsed);

    let start = Instant::now();
    let bytes = do_gitoxide(hashes.as_slice())?;
    let elapsed = start.elapsed();
    println!("gitoxide: confirmed {} bytes in {:?}", bytes, elapsed);

    Ok(())
}

fn do_git2(hashes: &[String]) -> anyhow::Result<u64> {
    git2::opts::strict_hash_verification(false);
    let repo = git2::Repository::open("../.git")?;
    let odb = repo.odb()?;
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git2::Oid::from_str(&hash)?;
        let obj = odb.read(hash)?;
        bytes += obj.len() as u64;
    }
    Ok(bytes)
}

fn do_gitoxide(hashes: &[String]) -> anyhow::Result<u64> {
    let odb = git_odb::compound::Db::at("../.git/objects")?;
    let mut buf = Vec::new();
    let mut bytes = 0u64;
    for hash in hashes {
        let hash = git_hash::owned::Digest::from_40_bytes_in_hex(hash.as_bytes())?;
        let obj = odb.locate(hash.to_borrowed(), &mut buf).expect("object must exist")?;
        bytes += obj.size() as u64;
    }
    Ok(bytes)
}
