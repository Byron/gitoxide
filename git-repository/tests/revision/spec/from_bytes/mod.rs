use git_repository::prelude::ObjectIdExt;
pub use git_repository::RevSpec;
use git_testtools::hex_to_id;

mod util;
pub use util::*;

mod ambiguous;
mod regex;

#[test]
fn bad_objects_are_valid_until_they_are_actually_read_from_the_odb() {
    {
        let repo = repo("blob.bad").unwrap();
        assert_eq!(
            parse_spec("e328", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("e32851d29feb48953c6f40b2e06d630a3c49608a").attach(&repo)),
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
            RevSpec::from_id(hex_to_id("cafea31147e840161a1860c50af999917ae1536b").attach(&repo))
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
        RevSpec::from_id(hex_to_id("0000000000b36b6aa7ea4b75318ed078f55505c3").attach(&repo))
    );

    assert_eq!(
        parse_spec("0000000000cdc:missing", &repo).unwrap_err().to_string(),
        "Could not find path \"missing\" in tree 0000000000c of parent object 0000000000c"
    );
}
