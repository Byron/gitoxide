//! Owned objects for use with serialization.
mod object {
    use crate::Time;
    use bstr::BString;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Signature {
        pub name: BString,
        pub email: BString,
        pub time: Time,
    }
}

mod tag {
    use crate::owned::object::Signature;
    use crate::Id;
    use bstr::BString;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Tag {
        // Target SHA1 in hex, always 40 lower case characters from 0-9 and a-f
        pub target: Id,
        // The name of the tag, e.g. "v1.0"
        pub name: BString,
        pub target_kind: crate::Kind,
        pub message: BString,
        pub signature: Signature,
        pub pgp_signature: Option<BString>,
    }
}

mod convert {
    use crate::owned::Signature;
    use crate::{borrowed, owned};

    impl Into<owned::Signature> for borrowed::Signature<'_> {
        fn into(self) -> Signature {
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
}

pub use object::*;
pub use tag::Tag;
