//! Owned objects for use with serialization.

mod ser {
    use bstr::{BString, ByteSlice};
    use quick_error::quick_error;
    use std::io;

    quick_error! {
        #[derive(Debug)]
        enum Error {
            NewlineInHeaderValue(value: BString) {
                display("Newlines are not allowed in header values: {:?}", value)
            }
            EmptyValue {
                display("Header values must not be empty")
            }
        }
    }

    impl Into<io::Error> for Error {
        fn into(self) -> io::Error {
            io::Error::new(io::ErrorKind::Other, self)
        }
    }

    const NL: &[u8; 1] = b"\n";
    const SPACE: &[u8; 1] = b" ";

    pub fn trusted_header_field(name: &[u8], value: &[u8], mut out: impl io::Write) -> io::Result<()> {
        out.write_all(name)?;
        out.write_all(&SPACE[..])?;
        out.write_all(value)?;
        out.write_all(&NL[..])
    }
    pub fn header_field(name: &[u8], value: &[u8], out: impl io::Write) -> io::Result<()> {
        if value.is_empty() {
            return Err(Error::EmptyValue.into());
        }
        if value.find(NL).is_some() {
            return Err(Error::NewlineInHeaderValue(value.into()).into());
        }
        trusted_header_field(name, value, out)
    }
}

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
    use crate::{owned, owned::ser, Id};
    use bstr::{BStr, BString};
    use quick_error::quick_error;
    use std::io;

    quick_error! {
        #[derive(Debug)]
        enum Error {
            InvalidTagName(name: BString) {
                display("A tag named '{}' is invalid", name)
            }
        }
    }

    impl From<Error> for io::Error {
        fn from(err: Error) -> Self {
            io::Error::new(io::ErrorKind::Other, err)
        }
    }

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Tag {
        // Target SHA1 in hex, always 40 lower case characters from 0-9 and a-f
        pub target: Id,
        // The name of the tag, e.g. "v1.0"
        pub name: BString,
        pub target_kind: crate::Kind,
        pub message: BString,
        pub signature: owned::object::Signature,
        pub pgp_signature: Option<BString>,
    }

    impl Tag {
        pub fn to_write(&self, mut out: impl io::Write) -> io::Result<()> {
            let mut hex_buf: [u8; 40] = [0; 40];
            self.target
                .encode_to_40_bytes_slice(&mut hex_buf[..])
                .expect("20 to 40 bytes hex encoding to always work");

            ser::trusted_header_field(b"object", &hex_buf, &mut out)?;
            ser::trusted_header_field(b"type", self.target_kind.to_bytes(), &mut out)?;
            ser::header_field(b"type", validated_name(self.name.as_ref())?, &mut out)?;
            unimplemented!("tag to_write")
        }
    }

    fn validated_name(name: &BStr) -> Result<&BStr, Error> {
        if name.is_empty() {
            return Err(Error::InvalidTagName(name.into()));
        }
        if name[0] == b'-' {
            return Err(Error::InvalidTagName(name.into()));
        }
        Ok(name)
    }

    #[cfg(test)]
    mod tests {
        mod validated_name {
            mod invalid {
                use super::super::super::*;
                use bstr::ByteSlice;

                #[test]
                fn leading_dash() {
                    assert!(validated_name(b"-".as_bstr()).is_err())
                }
            }

            mod valid {
                use super::super::super::*;
                use bstr::ByteSlice;

                #[test]
                fn version() {
                    assert!(validated_name(b"v1.0.0".as_bstr()).is_ok())
                }
            }
        }
    }
}

mod convert {
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
}

pub use object::*;
pub use tag::Tag;
