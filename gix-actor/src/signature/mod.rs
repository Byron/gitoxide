mod _ref {
    use bstr::{BStr, ByteSlice};

    use crate::{signature::decode, Signature, SignatureRef};

    impl<'a> SignatureRef<'a> {
        /// Deserialize a signature from the given `data`.
        pub fn from_bytes<E>(data: &'a [u8]) -> Result<SignatureRef<'a>, nom::Err<E>>
        where
            E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>,
        {
            decode(data).map(|(_, t)| t)
        }

        /// Create an owned instance from this shared one.
        pub fn to_owned(&self) -> Signature {
            Signature {
                name: self.name.to_owned(),
                email: self.email.to_owned(),
                time: self.time,
            }
        }

        /// Trim whitespace surrounding the name and email and return a new signature.
        pub fn trim(&self) -> SignatureRef<'a> {
            SignatureRef {
                name: self.name.trim().as_bstr(),
                email: self.email.trim().as_bstr(),
                time: self.time,
            }
        }

        /// Return the actor's name and email, effectively excluding the time stamp of this signature.
        pub fn actor(&self) -> (&BStr, &BStr) {
            (self.name, self.email)
        }
    }
}

mod convert {
    use crate::{Signature, SignatureRef};

    impl Signature {
        /// An empty signature, similar to 'null'.
        pub fn empty() -> Self {
            Signature::default()
        }

        /// Borrow this instance as immutable
        pub fn to_ref(&self) -> SignatureRef<'_> {
            SignatureRef {
                name: self.name.as_ref(),
                email: self.email.as_ref(),
                time: self.time,
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

    impl<'a> From<&'a Signature> for SignatureRef<'a> {
        fn from(other: &'a Signature) -> SignatureRef<'a> {
            other.to_ref()
        }
    }
}

mod write {
    use std::io;

    use bstr::{BStr, ByteSlice};
    use quick_error::quick_error;

    use crate::{Signature, SignatureRef};

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
        pub fn write_to(&self, out: impl io::Write) -> io::Result<()> {
            self.to_ref().write_to(out)
        }
        /// Computes the number of bytes necessary to serialize this signature
        pub fn size(&self) -> usize {
            self.to_ref().size()
        }
    }

    impl<'a> SignatureRef<'a> {
        /// Serialize this instance to `out` in the git serialization format for actors.
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            out.write_all(validated_token(self.name)?)?;
            out.write_all(b" ")?;
            out.write_all(b"<")?;
            out.write_all(validated_token(self.email)?)?;
            out.write_all(b"> ")?;
            self.time.write_to(out)
        }
        /// Computes the number of bytes necessary to serialize this signature
        pub fn size(&self) -> usize {
            self.name.len() + 2 /* space <*/ + self.email.len() +  2 /* > space */ + self.time.size()
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
