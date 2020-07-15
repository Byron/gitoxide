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
        pub enum Error {
            StartsWithDash {
                description("Tags must not start with a dash: '-'")
            }
            InvalidRefName(err: reference::Error) {
                display("The tag name was no valid reference name")
                from()
                cause(err)
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

    pub mod reference {
        use bstr::BStr;

        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                StartsWithDot {
                    description("A ref must not start with a '.'")
                }
                Empty {
                    description("A ref must not be empty")
                }
            }
        }

        pub fn validated_name(name: &BStr) -> Result<&BStr, Error> {
            if name.is_empty() {
                return Err(Error::Empty);
            }
            if name[0] == b'.' {
                return Err(Error::StartsWithDot);
            }
            Ok(name)
        }

        #[cfg(test)]
        mod tests {
            mod validated_name {
                mod valid {
                    use super::super::super::*;
                    use bstr::ByteSlice;

                    macro_rules! mktest {
                        ($name:ident, $input:literal) => {
                            #[test]
                            fn $name() {
                                assert!(validated_name($input.as_bstr()).is_ok())
                            }
                        };
                    }

                    mktest!(dot_in_the_middle, b"token.other");
                    mktest!(dot_at_the_end, b"hello.");
                }
                mod invalid {
                    use super::super::super::*;
                    use bstr::ByteSlice;

                    macro_rules! mktest {
                        ($name:ident, $input:literal, $expected:ident) => {
                            #[test]
                            fn $name() {
                                match validated_name($input.as_bstr()) {
                                    Err(Error::$expected) => {}
                                    got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                                }
                            }
                        };
                    }

                    mktest!(starts_with_dot, b".with-dot", StartsWithDot);
                    mktest!(empty, b"", Empty);
                }
            }
        }
    }

    fn validated_name(name: &BStr) -> Result<&BStr, Error> {
        reference::validated_name(name)?;
        if name[0] == b'-' {
            return Err(Error::StartsWithDash);
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
                fn only_dash() {
                    assert!(validated_name(b"-".as_bstr()).is_err())
                }
                #[test]
                fn leading_dash() {
                    assert!(validated_name(b"-hello".as_bstr()).is_err())
                }
            }

            mod valid {
                use super::super::super::*;
                use bstr::ByteSlice;

                #[test]
                fn version() {
                    for version in &["v1.0.0", "0.2.1", "0-alpha1"] {
                        assert!(validated_name(version.as_bytes().as_bstr()).is_ok())
                    }
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
