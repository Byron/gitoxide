use std::borrow::Cow;

use gix_object::bstr::ByteSlice;
use gix_revision::describe;

use crate::hex_to_id;

#[test]
fn exact_match_with_dirty_and_long() {
    let mut format = describe::Outcome {
        name: Some(Cow::Borrowed(b"main".as_bstr())),
        id: hex_to_id("b920bbb055e1efb9080592a409d3975738b6efb3"),
        depth: 0,
        name_by_oid: Default::default(),
        commits_seen: 0,
    }
    .into_format(7);
    assert!(format.is_exact_match());
    assert_eq!(format.to_string(), "main");
    assert_eq!(format.long(true).to_string(), "main-0-gb920bbb");

    format.dirty_suffix = Some("dirty".into());
    assert_eq!(format.long(false).to_string(), "main-dirty");
    assert_eq!(format.long(true).to_string(), "main-0-gb920bbb-dirty");

    format.dirty_suffix = None;
    format.depth = 42;
    assert!(!format.is_exact_match());
    assert_eq!(format.long(false).to_string(), "main-42-gb920bbb");

    format.dirty_suffix = Some("dirty".into());
    assert_eq!(format.to_string(), "main-42-gb920bbb-dirty");
    assert_eq!(format.long(true).to_string(), "main-42-gb920bbb-dirty");
}

#[test]
fn show_abbrev_hash_if_no_name_is_known() {
    let mut format = describe::Outcome {
        name: None,
        id: hex_to_id("b920bbb055e1efb9080592a409d3975738b6efb3"),
        depth: 0,
        name_by_oid: Default::default(),
        commits_seen: 0,
    }
    .into_format(7);
    assert!(
        format.is_exact_match(),
        "it reports true as it is only dependent on the depth which plays no role here"
    );
    assert_eq!(format.long(false).to_string(), "b920bbb");
    assert_eq!(format.long(true).to_string(), "b920bbb");

    format.dirty_suffix = Some("dirty".into());
    assert_eq!(format.long(false).to_string(), "b920bbb-dirty");
    assert_eq!(format.long(true).to_string(), "b920bbb-dirty");
}
