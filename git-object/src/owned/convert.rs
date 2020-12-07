use crate::{borrowed, owned};

impl Into<owned::Signature> for borrowed::Signature<'_> {
    fn into(self) -> owned::Signature {
        let borrowed::Signature { name, email, time } = self;
        owned::Signature {
            name: name.to_owned(),
            email: email.to_owned(),
            time,
        }
    }
}

impl Into<owned::Tag> for borrowed::Tag<'_> {
    fn into(self) -> owned::Tag {
        let borrowed::Tag {
            target,
            name,
            target_kind,
            message,
            signature,
            pgp_signature,
        } = self;
        owned::Tag {
            target: owned::Id::from_40_bytes_in_hex(&target).expect("40 bytes hex sha1"),
            name: name.to_owned(),
            target_kind,
            message: message.to_owned(),
            signature: signature.map(Into::into),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}

impl Into<owned::Commit> for borrowed::Commit<'_> {
    fn into(self) -> owned::Commit {
        let borrowed::Commit {
            tree,
            parents,
            author,
            committer,
            encoding,
            message,
            extra_headers,
        } = self;
        owned::Commit {
            tree: owned::Id::from_40_bytes_in_hex(&tree).expect("40 bytes hex sha1"),
            parents: parents
                .iter()
                .map(|parent| owned::Id::from_40_bytes_in_hex(parent).expect("40 bytes hex sha1"))
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

impl Into<owned::Tree> for borrowed::Tree<'_> {
    fn into(self) -> owned::Tree {
        let borrowed::Tree { entries } = self;
        owned::Tree {
            entries: entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl<'a> From<borrowed::Id<'a>> for owned::Id {
    fn from(v: borrowed::Id<'a>) -> Self {
        owned::Id::from_borrowed_sha1(v.sha1())
    }
}

impl Into<owned::tree::Entry> for borrowed::tree::Entry<'_> {
    fn into(self) -> owned::tree::Entry {
        let borrowed::tree::Entry { mode, filename, oid } = self;
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
