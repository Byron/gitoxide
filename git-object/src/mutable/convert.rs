use crate::{immutable, mutable};

impl From<immutable::TagRef<'_>> for mutable::Tag {
    fn from(other: immutable::TagRef<'_>) -> mutable::Tag {
        let immutable::TagRef {
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

impl From<immutable::CommitRef<'_>> for mutable::Commit {
    fn from(other: immutable::CommitRef<'_>) -> mutable::Commit {
        let immutable::CommitRef {
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

impl<'a> From<immutable::BlobRef<'a>> for mutable::Blob {
    fn from(v: immutable::BlobRef<'a>) -> Self {
        mutable::Blob {
            data: v.data.to_owned(),
        }
    }
}

impl From<immutable::Tree<'_>> for mutable::Tree {
    fn from(other: immutable::Tree<'_>) -> mutable::Tree {
        let immutable::Tree { entries } = other;
        mutable::Tree {
            entries: entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<immutable::tree::Entry<'_>> for mutable::tree::Entry {
    fn from(other: immutable::tree::Entry<'_>) -> mutable::tree::Entry {
        let immutable::tree::Entry { mode, filename, oid } = other;
        mutable::tree::Entry {
            mode,
            filename: filename.to_owned(),
            oid: oid.into(),
        }
    }
}

impl<'a> From<immutable::ObjectRef<'a>> for mutable::Object {
    fn from(v: immutable::ObjectRef<'_>) -> Self {
        match v {
            immutable::ObjectRef::Tree(v) => mutable::Object::Tree(v.into()),
            immutable::ObjectRef::Blob(v) => mutable::Object::Blob(v.into()),
            immutable::ObjectRef::Commit(v) => mutable::Object::Commit(v.into()),
            immutable::ObjectRef::Tag(v) => mutable::Object::Tag(v.into()),
        }
    }
}
