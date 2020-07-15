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
            target: crate::Id::from_40_bytes_in_hex(&target).expect("40 bytes hex sha1"),
            name: name.to_owned(),
            target_kind,
            message: message.to_owned(),
            signature: signature.into(),
            pgp_signature: pgp_signature.map(ToOwned::to_owned),
        }
    }
}
