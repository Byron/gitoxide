use bstr::BString;

/// Indicates key or values contain errors that can't be encoded.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("{key:?}={value:?} must not contain null bytes or newlines neither in key nor in value.")]
    Encoding { key: String, value: BString },
}

mod serde {
    use crate::helper::context::Error;
    use bstr::BStr;

    mod write {
        use crate::helper::context::serde::validate;
        use crate::helper::Context;

        impl Context {
            /// Write ourselves to `out` such that [`from_bytes()`][Self::from_bytes()] can decode it losslessly.
            pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
                for (key, value) in [("url", &self.url), ("path", &self.path)] {
                    if let Some(value) = value {
                        validate(key, value.as_slice().into())
                            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                        out.write_all(key.as_bytes())?;
                        out.write_all(b"=")?;
                        out.write_all(value)?;
                        out.write_all(b"\n")?;
                    }
                }
                for (key, value) in [
                    ("protocol", &self.protocol),
                    ("host", &self.host),
                    ("username", &self.username),
                    ("password", &self.password),
                ] {
                    if let Some(value) = value {
                        validate(key, value.as_str().into())
                            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                        out.write_all(key.as_bytes())?;
                        out.write_all(b"=")?;
                        out.write_all(value.as_bytes())?;
                        out.write_all(b"\n")?;
                    }
                }
                Ok(())
            }
        }
    }

    ///
    pub mod decode {
        use crate::helper::context;
        use crate::helper::context::serde::validate;
        use crate::helper::Context;
        use bstr::{BString, ByteSlice};

        /// The error returned by [`from_bytes()`][Context::from_bytes()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Illformed UTF-8 in value of key {key:?}: {value:?}")]
            IllformedUtf8InValue { key: String, value: BString },
            #[error(transparent)]
            Encoding(#[from] context::Error),
            #[error("Invalid format in line {line:?}, expecting key=value")]
            Syntax { line: BString },
        }

        impl Context {
            /// Decode ourselves from `input` which is the format written by [`write_to()`][Self::write_to()].
            pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
                let mut ctx = Context::default();
                for res in input.lines().take_while(|line| !line.is_empty()).map(|line| {
                    let mut it = line.splitn(2, |b| *b == b'=');
                    match (it.next().and_then(|k| k.to_str().ok()), it.next().map(|v| v.as_bstr())) {
                        (Some(key), Some(value)) => validate(key, value)
                            .map(|_| (key, value.to_owned()))
                            .map_err(Into::into),
                        _ => Err(Error::Syntax { line: line.into() }),
                    }
                }) {
                    let (key, value) = res?;
                    match key {
                        "protocol" | "host" | "username" | "password" => {
                            if !value.is_utf8() {
                                return Err(Error::IllformedUtf8InValue { key: key.into(), value });
                            }
                            let value = value.to_string();
                            *match key {
                                "protocol" => &mut ctx.protocol,
                                "host" => &mut ctx.host,
                                "username" => &mut ctx.username,
                                "password" => &mut ctx.password,
                                _ => unreachable!("checked field names in match above"),
                            } = Some(value);
                        }
                        "url" => ctx.url = Some(value),
                        "path" => ctx.path = Some(value),
                        _ => {}
                    }
                }
                Ok(ctx)
            }
        }
    }

    fn validate(key: &str, value: &BStr) -> Result<(), Error> {
        if key.contains('\0') || key.contains('\n') || value.contains(&0) || value.contains(&b'\n') {
            return Err(Error::Encoding {
                key: key.to_owned(),
                value: value.to_owned(),
            });
        }
        Ok(())
    }
}
pub use self::serde::decode;
