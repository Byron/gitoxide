use crate::file::store;
use std::path::Path;

mod existing {
    use crate::file::store;
    use std::path::Path;

    #[test]
    fn success_and_failure() -> crate::Result {
        let store = store()?;
        for (partial_name, expected_path) in &[("main", Some("refs/heads/main")), ("does-not-exist", None)] {
            let reference = store.find_one_existing(*partial_name);
            match expected_path {
                Some(expected_path) => assert_eq!(reference?.relative_path(), Path::new(expected_path)),
                None => match reference {
                    Ok(_) => panic!("Expected error"),
                    Err(git_ref::file::find::existing::Error::NotFound(name)) => {
                        assert_eq!(name, Path::new(*partial_name));
                    }
                    Err(err) => panic!("Unexpected err: {:?}", err),
                },
            }
        }
        Ok(())
    }
}

#[test]
fn success() -> crate::Result {
    let store = store()?;
    for (partial_name, expected_path, expected_ref_kind) in &[
        ("dt1", "refs/tags/dt1", git_ref::Kind::Peeled), // tags before heads
        ("heads/dt1", "refs/heads/dt1", git_ref::Kind::Peeled),
        ("d1", "refs/d1", git_ref::Kind::Peeled), // direct refs before heads
        ("heads/d1", "refs/heads/d1", git_ref::Kind::Peeled),
        ("HEAD", "HEAD", git_ref::Kind::Symbolic), // it finds shortest paths first
        ("origin", "refs/remotes/origin/HEAD", git_ref::Kind::Symbolic),
        ("origin/HEAD", "refs/remotes/origin/HEAD", git_ref::Kind::Symbolic),
        ("origin/main", "refs/remotes/origin/main", git_ref::Kind::Peeled),
        ("t1", "refs/tags/t1", git_ref::Kind::Peeled),
        ("main", "refs/heads/main", git_ref::Kind::Peeled),
        ("heads/main", "refs/heads/main", git_ref::Kind::Peeled),
        ("refs/heads/main", "refs/heads/main", git_ref::Kind::Peeled),
    ] {
        let reference = store.find_one(*partial_name)?.expect("exists");
        assert_eq!(reference.relative_path(), Path::new(expected_path));
        assert_eq!(reference.target().kind(), *expected_ref_kind);
    }
    Ok(())
}

#[test]
fn failure() -> crate::Result {
    let store = store()?;
    for (partial_name, reason, is_err) in &[
        ("foobar", "does not exist", false),
        ("broken", "does not parse", true),
        ("../escaping", "an invalid ref name", true),
    ] {
        let reference = store.find_one(*partial_name);
        if *is_err {
            assert!(reference.is_err(), "{}", reason);
        } else {
            let reference = reference?;
            assert!(reference.is_none(), "{}", reason);
        }
    }
    Ok(())
}
