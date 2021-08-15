mod convert {
    use crate::{immutable, Signature};

    impl From<immutable::Signature<'_>> for Signature {
        fn from(other: immutable::Signature<'_>) -> Signature {
            let immutable::Signature { name, email, time } = other;
            Signature {
                name: name.to_owned(),
                email: email.to_owned(),
                time,
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
