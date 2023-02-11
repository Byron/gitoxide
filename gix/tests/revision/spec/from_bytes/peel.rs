use gix::{prelude::ObjectIdExt, revision::Spec};

use crate::{
    revision::spec::from_bytes::{parse_spec, repo},
    util::hex_to_id,
};

#[test]
fn peel_to_object() {
    let repo = &repo("complex_graph").unwrap();

    let expected = Spec::from_id(hex_to_id("55e825ebe8fd2ff78cad3826afb696b96b576a7e").attach(repo));
    assert_eq!(parse_spec("@^{}", repo).unwrap(), expected);
    assert_eq!(parse_spec("main^{}", repo).unwrap(), expected);
    assert_eq!(
        parse_spec("b-tag^{}", repo).unwrap(),
        Spec::from_id(hex_to_id("5b3f9e24965d0b28780b7ce5daf2b5b7f7e0459f").attach(repo))
    );
}

#[test]
fn trailing_colon_is_equivalent_to_peel_to_tree() {
    let repo = &repo("complex_graph").unwrap();
    assert_eq!(parse_spec("@^{tree}", repo).unwrap(), parse_spec("@:", repo).unwrap());
}
