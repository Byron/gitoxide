use crate::{
    file::{store_at, store_with_packed_refs},
    packed::write_packed_refs_with,
};
use git_ref::{packed, PartialName};
use std::convert::TryFrom;

#[test]
fn a_lock_file_would_not_be_a_valid_partial_name() {
    // doesn't really belong here but want to make sure refname validation works as expected.
    let err = PartialName::try_from("heads/hello.lock").expect_err("this should fail");
    assert_eq!(err.to_string(), "The reference name 'heads/hello.lock' is invalid");
}

#[test]
fn all_iterable_refs_can_be_found() -> crate::Result {
    let store = store_with_packed_refs()?;
    let packed_refs = store.packed()?.expect("packed-refs exist");

    for reference in packed_refs.iter()? {
        let reference = reference?;
        let found = packed_refs.find(reference.full_name)?.expect("reference exists");
        assert_eq!(reference, found, "both refs are exactly the same");
        let found = packed_refs.find_existing(reference.full_name)?;
        assert_eq!(reference, found);
    }
    Ok(())
}

#[test]
fn find_packed_refs_with_peeled_items_and_full_or_partial_names() -> crate::Result {
    let packed_refs = b"# pack-refs with: peeled fully-peeled sorted
916840c0e2f67d370291042cb5274a597f4fa9bc refs/tags/TEST-0.0.1
c4cebba92af964f2d126be90b8a6298c4cf84d45 refs/tags/git-actor-v0.1.0
^13da90b54699a6b500ec5cd7d175f2cd5a1bed06
0b92c8a256ae06c189e3b9c30b646d62ac8f7d10 refs/tags/git-actor-v0.1.1\n";
    let (_keep, path) = write_packed_refs_with(packed_refs)?;

    let buf = packed::Buffer::open(path, 1024)?;
    let name = "refs/tags/TEST-0.0.1";
    assert_eq!(
        buf.find(name)?.expect("reference exists"),
        packed::Reference {
            full_name: name.into(),
            target: "916840c0e2f67d370291042cb5274a597f4fa9bc".into(),
            object: None
        }
    );
    let name = "refs/tags/git-actor-v0.1.0";
    assert_eq!(
        buf.find(name)?.expect("reference exists"),
        packed::Reference {
            full_name: name.into(),
            target: "c4cebba92af964f2d126be90b8a6298c4cf84d45".into(),
            object: Some("13da90b54699a6b500ec5cd7d175f2cd5a1bed06".into())
        }
    );
    let name = "refs/tags/git-actor-v0.1.1";
    assert_eq!(
        buf.find(name)?.expect("reference exists"),
        packed::Reference {
            full_name: name.into(),
            target: "0b92c8a256ae06c189e3b9c30b646d62ac8f7d10".into(),
            object: None
        }
    );
    Ok(())
}

#[test]
#[ignore]
fn partial_name_to_full_name_conversion_rules_are_applied() {
    let _store = store_at("make_packed_refs_for_lookup_rules.sh").unwrap();
    todo!("see if these name gereration rules can be unified, it definitely needs some thought to be correct")
}

#[test]
fn invalid_refs_within_a_file_do_not_lead_to_incorrect_results() -> crate::Result {
    let broken_packed_refs = b"# pack-refs with: peeled fully-peeled sorted
916840c0e2f67d370291042cb5274a597f4fa9bc refs/tags/TEST-0.0.1
bogus refs/tags/git-actor-v0.1.0
^13da90b54699a6b500ec5cd7d175f2cd5a1bed06
0b92c8a256ae06c189e3b9c30b646d62ac8f7d10 refs/tags/git-actor-v0.1.1\n";
    let (_keep, path) = write_packed_refs_with(broken_packed_refs)?;

    let buf = packed::Buffer::open(path, 1024)?;

    let name = "refs/tags/git-actor-v0.1.1";
    assert_eq!(
        buf.find(name)?.expect("reference exists"),
        packed::Reference {
            full_name: name.into(),
            target: "0b92c8a256ae06c189e3b9c30b646d62ac8f7d10".into(),
            object: None
        }
    );

    for failing_name in &["refs/tags/TEST-0.0.1", "refs/tags/git-actor-v0.1.0"] {
        assert_eq!(
            buf.find(*failing_name)
                .expect_err("it should detect an err")
                .to_string(),
            "The reference could not be parsed"
        );
    }
    Ok(())
}
