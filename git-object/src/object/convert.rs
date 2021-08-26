use crate::{tree, Blob, BlobRef, Commit, CommitRef, Object, ObjectRef, Tag, TagRef, Tree, TreeRef};
use std::convert::TryFrom;

impl From<TagRef<'_>> for Tag {
    fn from(other: TagRef<'_>) -> Tag {
        let TagRef {
            target,
            name,
            target_kind,
            message,
            tagger: signature,
            pgp_signature,
        } = other;
        Tag {
            target: git_hash::ObjectId::from_hex(target).expect("40 bytes hex sha1"),
            name: name.to_owned(),
            target_kind,
            message: message.to_owned(),
            signature: signature.map(Into::into),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}

impl From<CommitRef<'_>> for Commit {
    fn from(other: CommitRef<'_>) -> Commit {
        let CommitRef {
            tree,
            parents,
            author,
            committer,
            encoding,
            message,
            extra_headers,
        } = other;
        Commit {
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

impl<'a> From<BlobRef<'a>> for Blob {
    fn from(v: BlobRef<'a>) -> Self {
        Blob {
            data: v.data.to_owned(),
        }
    }
}

impl From<TreeRef<'_>> for Tree {
    fn from(other: TreeRef<'_>) -> Tree {
        let TreeRef { entries } = other;
        Tree {
            entries: entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<tree::EntryRef<'_>> for tree::Entry {
    fn from(other: tree::EntryRef<'_>) -> tree::Entry {
        let tree::EntryRef { mode, filename, oid } = other;
        tree::Entry {
            mode,
            filename: filename.to_owned(),
            oid: oid.into(),
        }
    }
}

impl<'a> From<ObjectRef<'a>> for Object {
    fn from(v: ObjectRef<'_>) -> Self {
        match v {
            ObjectRef::Tree(v) => Object::Tree(v.into()),
            ObjectRef::Blob(v) => Object::Blob(v.into()),
            ObjectRef::Commit(v) => Object::Commit(v.into()),
            ObjectRef::Tag(v) => Object::Tag(v.into()),
        }
    }
}

impl From<Tag> for Object {
    fn from(v: Tag) -> Self {
        Object::Tag(v)
    }
}

impl From<Commit> for Object {
    fn from(v: Commit) -> Self {
        Object::Commit(v)
    }
}

impl From<Tree> for Object {
    fn from(v: Tree) -> Self {
        Object::Tree(v)
    }
}

impl From<Blob> for Object {
    fn from(v: Blob) -> Self {
        Object::Blob(v)
    }
}

impl TryFrom<Object> for Tag {
    type Error = Object;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(match value {
            Object::Tag(v) => v,
            _ => return Err(value),
        })
    }
}

impl TryFrom<Object> for Commit {
    type Error = Object;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(match value {
            Object::Commit(v) => v,
            _ => return Err(value),
        })
    }
}

impl TryFrom<Object> for Tree {
    type Error = Object;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(match value {
            Object::Tree(v) => v,
            _ => return Err(value),
        })
    }
}

impl TryFrom<Object> for Blob {
    type Error = Object;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(match value {
            Object::Blob(v) => v,
            _ => return Err(value),
        })
    }
}
