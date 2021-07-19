use crate::{
    file::{store_at, store_with_packed_refs},
    packed::write_packed_refs_with,
};
use git_ref::{packed, PartialName};
use std::{convert::TryFrom, path::Path};

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
fn partial_name_to_full_name_conversion_rules_are_applied() -> crate::Result {
    let store = store_at("make_packed_refs_for_lookup_rules.sh")?;
    let packed = store.packed()?.expect("packed-refs exists");

    assert_eq!(
        store.loose_find_existing("origin")?.relative_path(),
        Path::new("refs/remotes/origin/HEAD"),
        "a special that only applies to loose refs"
    );
    assert!(
        packed.find("origin")?.is_none(),
        "packed refs don't have this special case as they don't store HEADs or symrefs"
    );
    assert_eq!(
        store.loose_find_existing("HEAD")?.relative_path(),
        Path::new("HEAD"),
        "HEAD can be found in loose stores"
    );
    assert!(
        packed.find("HEAD")?.is_none(),
        "packed refs definitely don't contain HEAD"
    );
    assert_eq!(
        packed.find("head-or-tag")?.expect("present").full_name,
        "refs/tags/head-or-tag",
        "it finds tags first"
    );
    assert_eq!(
        packed.find("heads/head-or-tag")?.expect("present").full_name,
        "refs/heads/head-or-tag",
        "it finds heads when disambiguated"
    );
    assert_eq!(
        packed.find("main")?.expect("present").full_name,
        "refs/heads/main",
        "it finds local heads before remote ones"
    );
    assert_eq!(
        packed.find("origin/main")?.expect("present").full_name,
        "refs/remotes/origin/main",
        "it finds remote heads when disambiguated"
    );
    assert_eq!(
        packed.find("remotes/origin/main")?.expect("present").full_name,
        "refs/remotes/origin/main",
        "more specification is possible, too"
    );
    assert_eq!(
        packed.find("tag-object")?.expect("present"),
        packed::Reference {
            full_name: "refs/tags/tag-object".into(),
            target: "b3109a7e51fc593f85b145a76c70ddd1d133fafd".into(),
            object: Some("134385f6d781b7e97062102c6a483440bfda2a03".into())
        },
        "tag objects aren't special, but lets test a little more"
    );
    Ok(())
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

#[test]
fn find_speed() -> crate::Result {
    let store = store_at("make_repository_with_lots_of_packed_refs.sh")?;
    let packed = store.packed()?.expect("packed-refs present");
    let start = std::time::Instant::now();
    let mut num_refs = 0;
    for r in packed.iter()?.take(10_000) {
        num_refs += 1;
        let r = r?;
        assert_eq!(
            packed.find(r.full_name)?.expect("ref was found"),
            r,
            "the refs are the same"
        );
    }
    let elapsed = start.elapsed().as_secs_f32();
    eprintln!(
        "Found {} refs in {}s ({} refs/s)",
        num_refs,
        elapsed,
        num_refs as f32 / elapsed
    );
    Ok(())
}
