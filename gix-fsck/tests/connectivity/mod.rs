use gix_fsck::Connectivity;
use gix_hash::ObjectId;
use gix_hashtable::HashMap;
use gix_object::Kind;
use gix_testtools::once_cell::sync::Lazy;

use crate::hex_to_id;

fn check_missing<'a>(repo_name: &str, commits: impl IntoIterator<Item = &'a ObjectId>) -> HashMap<ObjectId, Kind> {
    let db = {
        let fixture_path = gix_testtools::scripted_fixture_read_only("make_test_repos.sh")
            .expect("fixture path")
            .join(repo_name)
            .join(".git")
            .join("objects");
        let mut db = gix_odb::at(fixture_path).expect("valid odb");
        db.refresh_never();
        db
    };

    let mut missing: HashMap<ObjectId, Kind> = HashMap::default();
    let record_missing_and_assert_no_duplicate = |oid: &ObjectId, kind: Kind| {
        missing.try_insert(*oid, kind).expect("no duplicate oid");
    };

    let mut check = Connectivity::new(db, record_missing_and_assert_no_duplicate);
    for commit in commits.into_iter() {
        check.check_commit(commit).expect("commit is present")
    }
    missing
}

fn hex_to_ids<'a>(hex_ids: impl IntoIterator<Item = &'a str>) -> Vec<ObjectId> {
    hex_ids.into_iter().map(hex_to_id).collect()
}

fn hex_to_objects<'a>(hex_ids: impl IntoIterator<Item = &'a str>, kind: Kind) -> HashMap<ObjectId, Kind> {
    hex_to_ids(hex_ids).into_iter().map(|id| (id, kind)).collect()
}

// Get a `&Vec<ObjectID` for each commit in the test fixture repository
fn all_commits() -> &'static [ObjectId] {
    static ALL_COMMITS: Lazy<Vec<ObjectId>> = Lazy::new(|| {
        hex_to_ids([
            "5d18db2e2aabadf7b914435ef34f2faf8b4546dd",
            "3a3dfaa55a515f3fb3a25751107bbb523af6a1b0",
            "734c926856a328d1168ffd7088532e0d1ad19bbe",
        ])
    });
    &ALL_COMMITS
}

#[test]
fn no_missing() {
    // The "base" repo is the original, and has every object present
    assert_eq!(check_missing("base", all_commits()), HashMap::default());
}

#[test]
fn missing_blobs() {
    // The "blobless" repo is cloned with `--filter=blob:none`, and is missing one blob
    let expected = hex_to_objects(["c18147dc648481eeb65dc5e66628429a64843327"], Kind::Blob);
    assert_eq!(check_missing("blobless", all_commits()), expected);
}

#[test]
fn missing_trees() {
    // The "treeless" repo is cloned with `--filter=tree:0`, and is missing two trees
    // NOTE: This repo is also missing a blob, but we have no way of knowing that, as the tree referencing it is missing
    let expected = hex_to_objects(
        [
            "9561cfbae43c5e2accdfcd423378588dd10d827f",
            "fc264b3b6875a46e9031483aeb9994a1b897ffd3",
        ],
        Kind::Tree,
    );
    assert_eq!(check_missing("treeless", all_commits()), expected);
}
