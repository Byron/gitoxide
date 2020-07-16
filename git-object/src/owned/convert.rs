use crate::{borrowed, owned};
use smallvec::SmallVec;
use std::iter::FromIterator;

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
            signature: signature.into(),
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
            pgp_signature,
        } = self;
        owned::Commit {
            tree: owned::Id::from_40_bytes_in_hex(&tree).expect("40 bytes hex sha1"),
            parents: SmallVec::from_iter(
                parents
                    .iter()
                    .map(|parent| owned::Id::from_40_bytes_in_hex(parent).expect("40 bytes hex sha1")),
            ),
            author: author.into(),
            committer: committer.into(),
            encoding: encoding.map(ToOwned::to_owned),
            message: message.to_owned(),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}
