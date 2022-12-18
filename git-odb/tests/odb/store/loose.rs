use std::sync::atomic::AtomicBool;

use git_actor::{Sign, Time};
use git_features::progress;
use git_object::bstr::ByteSlice;
use git_odb::loose::Store;
use pretty_assertions::assert_eq;

use crate::{fixture_path, hex_to_id};

fn ldb() -> Store {
    Store::at(fixture_path("objects"), git_hash::Kind::Sha1)
}

pub fn object_ids() -> Vec<git_hash::ObjectId> {
    vec![
        hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980"), // blob
        hex_to_id("595dfd62fc1ad283d61bb47a24e7a1f66398f84d"), // blob
        hex_to_id("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f"), // tree
        hex_to_id("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9"), // tag
        hex_to_id("96ae868b3539f551c88fd5f02394d022581b11b0"), // tree
        hex_to_id("a706d7cd20fc8ce71489f34b50cf01011c104193"), // blob (big)
        hex_to_id("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec"), // commit
    ]
}

#[test]
fn iter() {
    let mut oids = ldb().iter().map(Result::unwrap).collect::<Vec<_>>();
    oids.sort();
    assert_eq!(oids, object_ids());
}
pub fn locate_oid(id: git_hash::ObjectId, buf: &mut Vec<u8>) -> git_object::Data<'_> {
    ldb().try_find(id, buf).expect("read success").expect("id present")
}

#[test]
fn verify_integrity() {
    let db = ldb();
    let outcome = db.verify_integrity(progress::Discard, &AtomicBool::new(false)).unwrap();
    assert_eq!(outcome.num_objects, 7);
}

mod write {
    use git_odb::{loose, Write};

    use crate::store::loose::{locate_oid, object_ids};

    #[test]
    fn read_and_write() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let db = loose::Store::at(dir.path(), git_hash::Kind::Sha1);
        let mut buf = Vec::new();
        let mut buf2 = Vec::new();

        for oid in object_ids() {
            let obj = locate_oid(oid, &mut buf);
            let actual = db.write(&obj.decode()?)?;
            assert_eq!(actual, oid);
            assert_eq!(
                db.try_find(oid, &mut buf2)?.expect("id present").decode()?,
                obj.decode()?
            );
            let actual = db.write_buf(obj.kind, obj.data)?;
            assert_eq!(actual, oid);
            assert_eq!(
                db.try_find(oid, &mut buf2)?.expect("id present").decode()?,
                obj.decode()?
            );
        }
        Ok(())
    }
}

mod contains {
    use crate::store::loose::ldb;

    #[test]
    fn iterable_objects_are_contained() {
        let store = ldb();
        for oid in store.iter().map(Result::unwrap) {
            assert!(store.contains(oid));
        }
    }
}

mod lookup_prefix {
    use std::collections::HashSet;

    use git_testtools::{fixture_path, hex_to_id};
    use maplit::hashset;

    use crate::store::loose::ldb;

    #[test]
    fn returns_none_for_prefixes_without_any_match() {
        let store = ldb();
        let prefix = git_hash::Prefix::new(git_hash::ObjectId::null(git_hash::Kind::Sha1), 7).unwrap();
        assert!(store.lookup_prefix(prefix, None).unwrap().is_none());

        let mut candidates = HashSet::default();
        assert!(
            store.lookup_prefix(prefix, Some(&mut candidates)).unwrap().is_none(),
            "error codes are the same"
        );
        assert!(candidates.is_empty());
    }

    #[test]
    fn returns_some_err_for_prefixes_with_more_than_one_match() {
        let objects_dir = git_testtools::tempfile::tempdir().unwrap();
        git_testtools::copy_recursively_into_existing_dir(fixture_path("objects"), &objects_dir).unwrap();
        std::fs::write(
            objects_dir
                .path()
                .join("37")
                .join("d4ffffffffffffffffffffffffffffffffffff"),
            b"fake",
        )
        .unwrap();
        let store = git_odb::loose::Store::at(objects_dir.path(), git_hash::Kind::Sha1);
        let input_id = hex_to_id("37d4e6c5c48ba0d245164c4e10d5f41140cab980");
        let prefix = git_hash::Prefix::new(input_id, 4).unwrap();
        assert_eq!(
            store.lookup_prefix(prefix, None).unwrap(),
            Some(Err(())),
            "there are two objects with that prefix"
        );

        let mut candidates = HashSet::default();
        assert_eq!(
            store.lookup_prefix(prefix, Some(&mut candidates)).unwrap(),
            Some(Err(())),
            "the error code is the same"
        );
        assert_eq!(
            candidates,
            hashset! {hex_to_id("37d4ffffffffffffffffffffffffffffffffffff"), input_id},
            "we get both matching objects"
        );
    }

    #[test]
    fn iterable_objects_can_be_looked_up_with_varying_prefix_lengths() {
        let store = ldb();
        let hex_lengths = &[4, 7, 40];
        for (index, oid) in store.iter().map(Result::unwrap).enumerate() {
            for mut candidates in [None, Some(HashSet::default())] {
                let hex_len = hex_lengths[index % hex_lengths.len()];
                let prefix = git_hash::Prefix::new(oid, hex_len).unwrap();
                assert_eq!(
                    store
                        .lookup_prefix(prefix, candidates.as_mut())
                        .unwrap()
                        .expect("object exists")
                        .expect("unambiguous"),
                    oid
                );
                if let Some(candidates) = candidates {
                    assert_eq!(candidates, hashset! {oid});
                }
            }
        }
    }
}

mod find {
    use git_object::{bstr::ByteSlice, tree::EntryMode, BlobRef, CommitRef, Kind, TagRef, TreeRef};

    use crate::{
        hex_to_id,
        store::loose::{ldb, locate_oid, signature},
    };

    fn find<'a>(hex: &str, buf: &'a mut Vec<u8>) -> git_object::Data<'a> {
        locate_oid(hex_to_id(hex), buf)
    }

    #[test]
    fn tag() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("722fe60ad4f0276d5a8121970b5bb9dccdad4ef9", &mut buf);
        assert_eq!(o.kind, Kind::Tag);
        assert_eq!(o.data.len(), 1024);
        let expected = TagRef {
            target: b"ffa700b4aca13b80cb6b98a078e7c96804f8e0ec".as_bstr(),
            name: b"1.0.0".as_bstr(),
            target_kind: Kind::Commit,
            message: b"for the signature".as_bstr(),
            pgp_signature: Some(
                b"-----BEGIN PGP SIGNATURE-----
Comment: GPGTools - https://gpgtools.org

iQIzBAABCgAdFiEEw7xSvXbiwjusbsBqZl+Z+p2ZlmwFAlsapyYACgkQZl+Z+p2Z
lmy6Ug/+KzvzqiNpzz1bMVVAzp8NCbiEO3QGYPyeQc521lBwpaTrRYR+oHJY15r3
OdL5WDysTpjN8N5FNyfmvzkuPdTkK3JlYmO7VRjdA2xu/B6vIZLaOfAowFrhMvKo
8eoqwGcAP3rC5TuWEgzq2qhbjS4JXFLd4NLjWEFqT2Y2UKm+g8TeGOsa/0pF4Nq5
xeW4qCYR0WcQLFedbpkKHxag2GfaXKvzNNJdqYhVQssNa6BeSmsfDvlWYNe617wV
NvsR/zJT0wHb5SSH+h6QmwA7LQIQF//83Vc3aF7kv9D54r3ibXW5TjZ3WoeTUZO7
kefkzJ12EYDCFLPhHvXPog518nO8Ot46dX+okrF0/B4N3RFTvjKr7VAGTzv2D/Dg
DrD531S2F71b+JIRh641eeP7bjWFQi3tWLtrEOtjjsKPJfYRMKpYFnAO4UUJ6Rck
Z5fFXEUCO8d5WT56jzKDjmVoY01lA87O1YsP/J+zQAlc9v1k6jqeQ53LZNgTN+ue
5fJuSPT3T43pSOD1VQSr3aZ2Anc4Qu7K8uX9lkpxF9Sc0tDbeCosFLZMWNVp6m+e
cjHJZXWmV4CcRfmLsXzU8s2cR9A0DBvOxhPD1TlKC2JhBFXigjuL9U4Rbq9tdegB
2n8f2douw6624Tn/6Lm4a7AoxmU+CMiYagDxDL3RuZ8CAfh3bn0=
=aIns
-----END PGP SIGNATURE-----
"
                .as_bstr(),
            ),
            tagger: Some(signature(1528473343)),
        };
        assert_eq!(o.decode()?.as_tag().expect("tag"), &expected);
        Ok(())
    }

    #[test]
    fn commit() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("ffa700b4aca13b80cb6b98a078e7c96804f8e0ec", &mut buf);
        assert_eq!(o.kind, Kind::Commit);
        assert_eq!(o.data.len(), 1084);
        let expected = CommitRef {
            tree: b"6ba2a0ded519f737fd5b8d5ccfb141125ef3176f".as_bstr(),
            parents: vec![].into(),
            author: signature(1528473303),
            committer: signature(1528473303),
            encoding: None,
            message: b"initial commit\n".as_bstr(),
            extra_headers: vec![(b"gpgsig".as_bstr(), b"-----BEGIN PGP SIGNATURE-----\nComment: GPGTools - https://gpgtools.org\n\niQIzBAABCgAdFiEEw7xSvXbiwjusbsBqZl+Z+p2ZlmwFAlsaptwACgkQZl+Z+p2Z\nlmxXSQ//fj6t7aWoEKeMdFigfj6OXWPUyrRbS0N9kpJeOfA0BIOea/6Jbn8J5qh1\nYRfrySOzHPXR5Y+w4GwLiVas66qyhAbk4yeqZM0JxBjHDyPyRGhjUd3y7WjEa6bj\nP0ACAIkYZQ/Q/LDE3eubmhAwEobBH3nZbwE+/zDIG0i265bD5C0iDumVOiKkSelw\ncr6FZVw1HH+GcabFkeLRZLNGmPqGdbeBwYERqb0U1aRCzV1xLYteoKwyWcYaH8E3\n97z1rwhUO/L7o8WUEJtP3CLB0zuocslMxskf6bCeubBnRNJ0YrRmxGarxCP3vn4D\n3a/MwECnl6mnUU9t+OnfvrzLDN73rlq8iasUq6hGe7Sje7waX6b2UGpxHqwykmXg\nVimD6Ah7svJanHryfJn38DvJW/wOMqmAnSUAp+Y8W9EIe0xVntCmtMyoKuqBoY7T\nJlZ1kHJte6ELIM5JOY9Gx7D0ZCSKZJQqyjoqtl36dsomT0I78/+7QS1DP4S6XB7d\nc3BYH0JkW81p7AAFbE543ttN0Z4wKXErMFqUKnPZUIEuybtlNYV+krRdfDBWQysT\n3MBebjguVQ60oGs06PzeYBosKGQrHggAcwduLFuqXhLTJqN4UQ18RkE0vbtG3YA0\n+XtZQM13vURdfwFI5qitAGgw4EzPVrkWWzApzLCrRPEMbvP+b9A=\n=2qqN\n-----END PGP SIGNATURE-----".as_bstr().into())]
        };
        let object = o.decode()?;
        assert_eq!(object.as_commit().expect("commit"), &expected);
        Ok(())
    }

    #[test]
    fn blob_data() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("37d4e6c5c48ba0d245164c4e10d5f41140cab980", &mut buf);
        assert_eq!(o.data.as_bstr(), b"hi there\n".as_bstr());
        Ok(())
    }

    #[test]
    fn blob() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("37d4e6c5c48ba0d245164c4e10d5f41140cab980", &mut buf);
        assert_eq!(
            o.decode()?.as_blob().expect("blob"),
            &BlobRef {
                data: &[104, 105, 32, 116, 104, 101, 114, 101, 10]
            },
            "small blobs are treated similarly to other object types and are read into memory at once when the header is read"
        );
        Ok(())
    }

    #[test]
    fn blob_not_existing() {
        let mut buf = Vec::new();
        assert_eq!(try_locate("37d4e6c5c48ba0d245164c4e10d5f41140cab989", &mut buf), None);
    }

    #[test]
    fn blob_big() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("a706d7cd20fc8ce71489f34b50cf01011c104193", &mut buf);
        assert_eq!(
            o.decode()?.as_blob().expect("blob").data.len(),
            o.data.len(),
            "erm, blobs are the same as raw data?"
        );
        Ok(())
    }

    fn try_locate<'a>(hex: &str, buf: &'a mut Vec<u8>) -> Option<git_object::Data<'a>> {
        ldb().try_find(hex_to_id(hex), buf).ok().flatten()
    }

    pub fn as_id(id: &[u8; 20]) -> &git_hash::oid {
        id.into()
    }

    #[test]
    fn tree() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let o = find("6ba2a0ded519f737fd5b8d5ccfb141125ef3176f", &mut buf);
        assert_eq!(o.kind, Kind::Tree);
        assert_eq!(o.data.len(), 66);

        let expected = TreeRef {
            entries: vec![
                git_object::tree::EntryRef {
                    mode: EntryMode::Tree,
                    filename: b"dir".as_bstr(),
                    oid: as_id(&[
                        150, 174, 134, 139, 53, 57, 245, 81, 200, 143, 213, 240, 35, 148, 208, 34, 88, 27, 17, 176,
                    ]),
                },
                git_object::tree::EntryRef {
                    mode: EntryMode::Blob,
                    filename: b"file.txt".as_bstr(),
                    oid: as_id(&[
                        55, 212, 230, 197, 196, 139, 160, 210, 69, 22, 76, 78, 16, 213, 244, 17, 64, 202, 185, 128,
                    ]),
                },
            ],
        };
        assert_eq!(o.decode()?.as_tree().expect("tree"), &expected);
        Ok(())
    }

    mod header {
        use crate::odb::store::loose::ldb;
        use git_testtools::hex_to_id;

        #[test]
        fn existing() -> crate::Result {
            let db = ldb();
            assert_eq!(
                db.try_header(hex_to_id("a706d7cd20fc8ce71489f34b50cf01011c104193"))?
                    .expect("present"),
                (56915, git_object::Kind::Blob)
            );
            Ok(())
        }

        #[test]
        fn non_existing() -> crate::Result {
            let db = ldb();
            assert_eq!(
                db.try_header(hex_to_id("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))?,
                None,
                "it does not exist"
            );
            Ok(())
        }

        #[test]
        fn all() -> crate::Result {
            let db = ldb();
            let mut buf = Vec::new();
            for id in db.iter() {
                let id = id?;
                let expected = db.try_find(id, &mut buf)?.expect("exists");
                let (size, kind) = db.try_header(id)?.expect("header exists");
                assert_eq!(size, expected.data.len());
                assert_eq!(kind, expected.kind);
            }
            Ok(())
        }
    }
}

fn signature(time: u32) -> git_actor::SignatureRef<'static> {
    git_actor::SignatureRef {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"byronimo@gmail.com".as_bstr(),
        time: Time {
            seconds_since_unix_epoch: time,
            offset_in_seconds: 7200,
            sign: Sign::Plus,
        },
    }
}
