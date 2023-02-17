use crate::hex_to_id;

fn oid() -> gix_hash::ObjectId {
    hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
}

fn oid2() -> gix_hash::ObjectId {
    hex_to_id("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
}

#[test]
fn include() {
    assert_eq!(
        gix_revision::Spec::Include(oid()).to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
}

#[test]
fn exclude() {
    assert_eq!(
        gix_revision::Spec::Exclude(oid()).to_string(),
        "^aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
}

#[test]
fn range() {
    assert_eq!(
        gix_revision::Spec::Range {
            from: oid(),
            to: oid2()
        }
        .to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa..bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    );
}

#[test]
fn merge() {
    assert_eq!(
        gix_revision::Spec::Merge {
            theirs: oid(),
            ours: oid2()
        }
        .to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa...bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    );
}

#[test]
fn include_parents() {
    assert_eq!(
        gix_revision::Spec::IncludeOnlyParents(oid()).to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa^@"
    );
}

#[test]
fn exclude_parents() {
    assert_eq!(
        gix_revision::Spec::ExcludeParents(oid()).to_string(),
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa^!"
    );
}
