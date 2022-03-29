use std::borrow::Cow;

use git_object::bstr::ByteSlice;
use git_revision::describe;
use git_testtools::hex_to_id;

#[test]
fn exact_match_with_dirty_and_long() {
    let mut format = describe::Outcome {
        name: Cow::Borrowed(b"main".as_bstr()),
        id: hex_to_id("b920bbb055e1efb9080592a409d3975738b6efb3"),
        depth: 0,
    }
    .into_format(7);
    assert!(format.is_exact_match());
    assert_eq!(format.to_string(), "main");
    assert_eq!(format.long().to_string(), "main-0-gb920bbb");

    format.dirty_suffix = Some("dirty".into());
    assert_eq!(format.short().to_string(), "main-dirty");
    assert_eq!(format.long().to_string(), "main-0-gb920bbb-dirty");

    format.dirty_suffix = None;
    format.depth = 42;
    assert!(!format.is_exact_match());
    assert_eq!(format.short().to_string(), "main-42-gb920bbb");

    format.dirty_suffix = Some("dirty".into());
    assert_eq!(format.to_string(), "main-42-gb920bbb-dirty");
    assert_eq!(format.long().to_string(), "main-42-gb920bbb-dirty");
}
