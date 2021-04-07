use crate::{borrowed, owned};

impl From<borrowed::Signature<'_>> for owned::Signature {
    fn from(other: borrowed::Signature<'_>) -> owned::Signature {
        let borrowed::Signature { name, email, time } = other;
        owned::Signature {
            name: name.to_owned(),
            email: email.to_owned(),
            time,
        }
    }
}

impl From<borrowed::Tag<'_>> for owned::Tag {
    fn from(other: borrowed::Tag<'_>) -> owned::Tag {
        let borrowed::Tag {
            target,
            name,
            target_kind,
            message,
            signature,
            pgp_signature,
        } = other;
        owned::Tag {
            target: git_hash::ObjectId::from_hex(&target).expect("40 bytes hex sha1"),
            name: name.to_owned(),
            target_kind,
            message: message.to_owned(),
            signature: signature.map(Into::into),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}

impl From<borrowed::Commit<'_>> for owned::Commit {
    fn from(other: borrowed::Commit<'_>) -> owned::Commit {
        let borrowed::Commit {
            tree,
            parents,
            author,
            committer,
            encoding,
            message,
            extra_headers,
        } = other;
        owned::Commit {
            tree: git_hash::ObjectId::from_hex(&tree).expect("40 bytes hex sha1"),
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

impl<'a> From<borrowed::Blob<'a>> for owned::Blob {
    fn from(v: borrowed::Blob<'a>) -> Self {
        owned::Blob {
            data: v.data.to_owned(),
        }
    }
}

impl From<borrowed::Tree<'_>> for owned::Tree {
    fn from(other: borrowed::Tree<'_>) -> owned::Tree {
        let borrowed::Tree { entries } = other;
        owned::Tree {
            entries: entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<borrowed::tree::Entry<'_>> for owned::tree::Entry {
    fn from(other: borrowed::tree::Entry<'_>) -> owned::tree::Entry {
        let borrowed::tree::Entry { mode, filename, oid } = other;
        owned::tree::Entry {
            mode,
            filename: filename.to_owned(),
            oid: oid.into(),
        }
    }
}

impl<'a> From<borrowed::Object<'a>> for owned::Object {
    fn from(v: borrowed::Object<'_>) -> Self {
        match v {
            borrowed::Object::Tree(v) => owned::Object::Tree(v.into()),
            borrowed::Object::Blob(v) => owned::Object::Blob(v.into()),
            borrowed::Object::Commit(v) => owned::Object::Commit(v.into()),
            borrowed::Object::Tag(v) => owned::Object::Tag(v.into()),
        }
    }
}
