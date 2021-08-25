mod _ref {
    use crate::{signature::decode, SignatureRef};

    impl<'a> SignatureRef<'a> {
        /// Deserialize a signature from the given `data`.
        pub fn from_bytes<E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
            data: &'a [u8],
        ) -> Result<SignatureRef<'a>, nom::Err<E>> {
            decode(data).map(|(_, t)| t)
        }
    }
}

mod convert {
    use crate::{Sign, Signature, SignatureRef, Time};

    impl Signature {
        /// An empty signature, similar to 'null'.
        pub fn empty() -> Self {
            Signature {
                name: Default::default(),
                email: Default::default(),
                time: Time {
                    time: 0,
                    offset: 0,
                    sign: Sign::Plus,
                },
            }
        }
    }

    impl From<SignatureRef<'_>> for Signature {
        fn from(other: SignatureRef<'_>) -> Signature {
            let SignatureRef { name, email, time } = other;
            Signature {
                name: name.to_owned(),
                email: email.to_owned(),
                time,
            }
        }
    }

    impl Signature {
        /// Borrow this instance as signature_ref
        pub fn to_ref(&self) -> SignatureRef<'_> {
            SignatureRef {
                name: self.name.as_ref(),
                email: self.email.as_ref(),
                time: self.time,
            }
        }
    }
}

mod write {
    use std::io;

    use bstr::{BStr, ByteSlice};
    use quick_error::quick_error;

    use crate::{Signature, SPACE};

    quick_error! {
        /// The Error produced by [`Signature::write_to()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        enum Error {
            IllegalCharacter {
                display("Signature name or email must not contain '<', '>' or \\n")
            }
        }
    }

    impl From<Error> for io::Error {
        fn from(err: Error) -> Self {
            io::Error::new(io::ErrorKind::Other, err)
        }
    }

    /// Output
    impl Signature {
        /// Serialize this instance to `out` in the git serialization format for actors.
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            out.write_all(validated_token(self.name.as_bstr())?)?;
            out.write_all(SPACE)?;
            out.write_all(&b"<"[..])?;
            out.write_all(validated_token(self.email.as_bstr())?)?;
            out.write_all(&b"> "[..])?;
            self.time.write_to(out)?;
            Ok(())
        }
    }

    fn validated_token(name: &BStr) -> Result<&BStr, Error> {
        if name.find_byteset(b"<>\n").is_some() {
            return Err(Error::IllegalCharacter);
        }
        Ok(name)
    }
}

///
mod decode;
pub use decode::decode;
