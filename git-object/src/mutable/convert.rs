use crate::{immutable, mutable, BlobRef, CommitRef, ObjectRef, TagRef, TreeRef};

impl From<TagRef<'_>> for mutable::Tag {
    fn from(other: TagRef<'_>) -> mutable::Tag {
        let TagRef {
            target,
            name,
            target_kind,
            message,
            tagger: signature,
            pgp_signature,
        } = other;
        mutable::Tag {
            target: git_hash::ObjectId::from_hex(target).expect("40 bytes hex sha1"),
            name: name.to_owned(),
            target_kind,
            message: message.to_owned(),
            signature: signature.map(Into::into),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}

impl From<CommitRef<'_>> for mutable::Commit {
    fn from(other: CommitRef<'_>) -> mutable::Commit {
        let CommitRef {
            tree,
            parents,
            author,
            committer,
            encoding,
            message,
            extra_headers,
        } = other;
        mutable::Commit {
            tree: git_hash::ObjectId::from_hex(tree).expect("40 bytes hex sha1"),
            parents: parents
                .iter()
                .map(|parent| git_hash::ObjectId::from_hex(parent).expect("40 bytes hex sha1"))
                .collect(),
            author: author.into(),
            committer: committer.into(),
            encoding: encoding.map(ToOwned::to_owned),
            message: message.to_owned(),
            extra_headers: extra_headers
                .into_iter()
                .map(|(k, v)| (k.into(), v.into_owned()))
                .collect(),
        }
    }
}

impl<'a> From<BlobRef<'a>> for mutable::Blob {
    fn from(v: BlobRef<'a>) -> Self {
        mutable::Blob {
            data: v.data.to_owned(),
        }
    }
}

impl From<TreeRef<'_>> for mutable::Tree {
    fn from(other: TreeRef<'_>) -> mutable::Tree {
        let TreeRef { entries } = other;
        mutable::Tree {
            entries: entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<immutable::tree::EntryRef<'_>> for mutable::tree::Entry {
    fn from(other: immutable::tree::EntryRef<'_>) -> mutable::tree::Entry {
        let immutable::tree::EntryRef { mode, filename, oid } = other;
        mutable::tree::Entry {
            mode,
            filename: filename.to_owned(),
            oid: oid.into(),
        }
    }
}

impl<'a> From<ObjectRef<'a>> for mutable::Object {
    fn from(v: ObjectRef<'_>) -> Self {
        match v {
            ObjectRef::Tree(v) => mutable::Object::Tree(v.into()),
            ObjectRef::Blob(v) => mutable::Object::Blob(v.into()),
            ObjectRef::Commit(v) => mutable::Object::Commit(v.into()),
            ObjectRef::Tag(v) => mutable::Object::Tag(v.into()),
        }
    }
}
