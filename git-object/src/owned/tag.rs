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
