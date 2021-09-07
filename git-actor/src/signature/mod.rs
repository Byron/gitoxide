mod _ref {
    use crate::{signature::decode, SignatureRef};

    impl<'a> SignatureRef<'a> {
        /// Deserialize a signature from the given `data`.
        pub fn from_bytes<E>(data: &'a [u8]) -> Result<SignatureRef<'a>, nom::Err<E>>
        where
            E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>,
        {
            decode(data).map(|(_, t)| t)
        }
    }
}

mod convert {
    use crate::{Sign, Signature, SignatureRef, Time};

    impl Default for Signature {
        fn default() -> Self {
            Signature::empty()
        }
    }

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
        /// Borrow this instance as immutable
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

mod init {
    use bstr::BString;

    use crate::{Signature, Time};

    impl Signature {
        /// Return an actor identified `name` and `email` at the current local time, that is a time with a timezone offset from
        /// UTC based on the hosts configuration.
        #[cfg(feature = "local-time-support")]
        pub fn now_local(
            name: impl Into<BString>,
            email: impl Into<BString>,
        ) -> Result<Self, git_features::time::tz::Error> {
            let offset = git_features::time::tz::current_utc_offset()?;
            Ok(Signature {
                name: name.into(),
                email: email.into(),
                time: Time {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("the system time doesn't run backwards that much")
                        .as_secs() as u32,
                    offset,
                    sign: offset.into(),
                },
            })
        }

        /// Return an actor identified `name` and `email` at the current local time, or UTC time if the current time zone could
        /// not be obtained.
        #[cfg(feature = "local-time-support")]
        pub fn now_local_or_utc(name: impl Into<BString>, email: impl Into<BString>) -> Self {
            let offset = git_features::time::tz::current_utc_offset().unwrap_or(0);
            Signature {
                name: name.into(),
                email: email.into(),
                time: Time {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("the system time doesn't run backwards that much")
                        .as_secs() as u32,
                    offset,
                    sign: offset.into(),
                },
            }
        }

        /// Return an actor identified by `name` and `email` at the current time in UTC.
        ///
        /// This would be most useful for bot users, otherwise the [`now_local()`][Signature::now_local()] method should be preferred.
        pub fn now_utc(name: impl Into<BString>, email: impl Into<BString>) -> Self {
            let utc_offset = 0;
            Signature {
                name: name.into(),
                email: email.into(),
                time: Time {
                    time: seconds_since_epoch(),
                    offset: utc_offset,
                    sign: utc_offset.into(),
                },
            }
        }
    }

    fn seconds_since_epoch() -> u32 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("the system time doesn't run backwards that much")
            .as_secs() as u32
    }
}

///
mod decode;
pub use decode::decode;
