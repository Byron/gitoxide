use git_repository::prelude::ObjectIdExt;
use git_repository::revision::Spec;
use git_testtools::hex_to_id;

mod util;
pub use util::*;

mod ambiguous;
mod regex;
mod index {
    use crate::revision::spec::from_bytes::{parse_spec, repo};
    use git_repository::prelude::ObjectIdExt;
    use git_repository::revision::Spec;
    use git_testtools::hex_to_id;

    #[test]
    #[ignore]
    fn at_default_stage() {
        let repo = repo("complex_graph").unwrap();
        assert_eq!(
            parse_spec(":file", &repo).unwrap(),
            Spec::from_id(hex_to_id("fe27474251f7f8368742f01fbd3bd5666b630a82").attach(&repo))
        );

        assert_eq!(
            parse_spec(":1:file", &repo).unwrap_err().to_string(),
            "give hint as to where to find the file in the index and if it exists on disk",
        );

        assert_eq!(
            parse_spec(":foo", &repo).unwrap_err().to_string(),
            "does not exist (but use same error message as above, as it's parametric)",
        );
    }
}

#[test]
fn names_are_made_available_via_references() {
    let repo = repo("complex_graph").unwrap();
    let spec = parse_spec_no_baseline("main..g", &repo).unwrap();
    let (a, b) = spec.clone().into_references();
    assert_eq!(
        a.as_ref().map(|r| r.name().as_bstr().to_string()),
        Some("refs/heads/main".into())
    );
    assert_eq!(
        b.as_ref().map(|r| r.name().as_bstr().to_string()),
        Some("refs/heads/g".into())
    );
    assert_eq!(spec.first_reference(), a.as_ref().map(|r| &r.inner));
    assert_eq!(spec.second_reference(), b.as_ref().map(|r| &r.inner));

    let spec = parse_spec_no_baseline("@", &repo).unwrap();
    assert_eq!(spec.second_reference(), None);
    assert_eq!(
        spec.first_reference().map(|r| r.name.as_bstr().to_string()),
        Some("HEAD".into())
    );
}

#[test]
fn bad_objects_are_valid_until_they_are_actually_read_from_the_odb() {
    {
        let repo = repo("blob.bad").unwrap();
        assert_eq!(
            parse_spec("e328", &repo).unwrap(),
            Spec::from_id(hex_to_id("e32851d29feb48953c6f40b2e06d630a3c49608a").attach(&repo)),
            "we are able to return objects even though they are 'bad' when trying to decode them, like git",
        );
        assert_eq!(
            format!("{:?}", parse_spec("e328^{object}", &repo).unwrap_err()),
            r#"FindObject(Find(Loose(Decode(ObjectHeader(InvalidObjectKind("bad"))))))"#,
            "Now we enforce the object to exist and be valid, as ultimately it wants to match with a certain type"
        );
    }

    {
        let repo = repo("blob.corrupt").unwrap();
        assert_eq!(
            parse_spec("cafea", &repo).unwrap(),
            Spec::from_id(hex_to_id("cafea31147e840161a1860c50af999917ae1536b").attach(&repo))
        );
        assert_eq!(
            &format!("{:?}", parse_spec("cafea^{object}", &repo).unwrap_err())[..80],
            r#"FindObject(Find(Loose(DecompressFile { source: Inflate(DecompressError(General {"#
        );
    }
}

#[test]
fn access_blob_through_tree() {
    let repo = repo("ambiguous_blob_tree_commit").unwrap();
    assert_eq!(
        parse_spec("0000000000cdc:a0blgqsjc", &repo).unwrap(),
        Spec::from_id(hex_to_id("0000000000b36b6aa7ea4b75318ed078f55505c3").attach(&repo))
    );

    assert_eq!(
        parse_spec("0000000000cdc:missing", &repo).unwrap_err().to_string(),
        "Could not find path \"missing\" in tree 0000000000c of parent object 0000000000c"
    );
}
