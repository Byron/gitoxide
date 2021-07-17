use crate::file::store_at;
use git_ref::packed;
use std::time::Instant;

#[test]
fn empty() -> crate::Result {
    assert_eq!(
        packed::Iter::new(&[])?.count(),
        0,
        "empty buffers are fine and lead to no line returned"
    );
    Ok(())
}

#[test]
fn packed_refs_with_header() -> crate::Result {
    let dir = git_testtools::scripted_fixture_repo_read_only("make_packed_ref_repository.sh")?;
    let buf = std::fs::read(dir.join(".git").join("packed-refs"))?;
    let iter = packed::Iter::new(&buf)?;
    assert_eq!(iter.count(), 8, "it finds the right amount of items");
    Ok(())
}

#[test]
fn packed_refs_without_header() -> crate::Result {
    let packed_refs = b"916840c0e2f67d370291042cb5274a597f4fa9bc refs/tags/TEST-0.0.1
c4cebba92af964f2d126be90b8a6298c4cf84d45 refs/tags/git-actor-v0.1.0
^13da90b54699a6b500ec5cd7d175f2cd5a1bed06
0b92c8a256ae06c189e3b9c30b646d62ac8f7d10 refs/tags/git-actor-v0.1.1\n";
    assert_eq!(
        packed::Iter::new(packed_refs)?.collect::<Result<Vec<_>, _>>()?,
        vec![
            packed::Reference {
                full_name: "refs/tags/TEST-0.0.1".into(),
                target: "916840c0e2f67d370291042cb5274a597f4fa9bc".into(),
                object: None
            },
            packed::Reference {
                full_name: "refs/tags/git-actor-v0.1.0".into(),
                target: "c4cebba92af964f2d126be90b8a6298c4cf84d45".into(),
                object: Some("13da90b54699a6b500ec5cd7d175f2cd5a1bed06".into())
            },
            packed::Reference {
                full_name: "refs/tags/git-actor-v0.1.1".into(),
                target: "0b92c8a256ae06c189e3b9c30b646d62ac8f7d10".into(),
                object: None
            }
        ]
    );
    Ok(())
}

#[test]
fn broken_ref_doesnt_end_the_iteration() -> crate::Result {
    let packed_refs = b"916840c0e2f67d370291042cb5274a597f4fa9bc refs/tags/TEST-0.0.1
buggy-hash refs/wrong
^buggy-hash-too
0b92c8a256ae06c189e3b9c30b646d62ac8f7d10 refs/tags/git-actor-v0.1.1\n";
    let mut iter = packed::Iter::new(packed_refs)?;

    assert!(iter.next().expect("first ref").is_ok(), "first line is valid");
    assert_eq!(
        iter.next()
            .expect("second ref")
            .expect_err("an error is produced")
            .to_string(),
        "Invalid reference in line 2: 'buggy-hash refs/wrong'",
        "second line is invalid",
    );
    assert_eq!(
        iter.next()
            .expect("third ref")
            .expect_err("an error is produced")
            .to_string(),
        "Invalid reference in line 3: '^buggy-hash-too'",
        "third line is invalid",
    );
    assert!(iter.next().expect("last ref").is_ok(), "last line is valid");
    assert!(iter.next().is_none(), "exhausted");
    Ok(())
}

#[test]
fn iteration_speed() {
    let store = store_at("make_repository_with_lots_of_packed_refs.sh").unwrap();
    let start = Instant::now();
    let actual = store
        .packed()
        .unwrap()
        .expect("packed-refs present")
        .iter()
        .unwrap()
        .count();
    assert_eq!(actual, 150003);
    let elapsed = start.elapsed().as_secs_f32();
    eprintln!(
        "Enumerated {} refs in {}s ({} refs/s)",
        actual,
        elapsed,
        actual as f32 / elapsed
    );
}
