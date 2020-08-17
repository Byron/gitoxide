#![forbid(unsafe_code)]

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    File,
    Git,
    Ssh,
    Http,
    Https,
}

pub mod owned {
    use crate::Protocol;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum UserExpansion {
        Current,
        Name(String),
    }

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Url {
        pub protocol: Protocol,
        pub user: Option<String>,
        pub host: Option<String>,
        pub port: Option<u16>,
        pub path: Vec<u8>,
        pub expansion: Option<UserExpansion>,
    }

    impl Default for Url {
        fn default() -> Self {
            Url {
                protocol: Protocol::Ssh,
                user: None,
                host: None,
                port: None,
                path: Vec::new(),
                expansion: None,
            }
        }
    }

    pub mod expand {
        use crate::owned::{Url, UserExpansion};
        use bstr::ByteSlice;
        use quick_error::quick_error;
        use std::{
            borrow::Cow,
            path::{Path, PathBuf},
        };

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Utf8(err: bstr::Utf8Error) {
                    display("UTF8 conversion on non-unix system failed")
                    from()
                    source(err)
                }
                MissingHome {
                    display("Home directory could not be obtained")
                }
            }
        }

        impl Url {
            pub fn expand_path_with(
                &self,
                home_for_user: impl FnOnce(&UserExpansion) -> Option<PathBuf>,
            ) -> Result<PathBuf, Error> {
                fn make_relative(path: &Path) -> Cow<Path> {
                    if path.is_relative() {
                        return path.into();
                    }
                    path.components().skip(1).collect::<PathBuf>().into()
                }
                let path = self.path.to_path()?;
                Ok(match self.expansion.as_ref() {
                    Some(user) => home_for_user(user).ok_or(Error::MissingHome)?.join(make_relative(path)),
                    None => self.path.to_path()?.into(),
                })
            }

            pub fn expand_user(&self) -> Result<PathBuf, Error> {
                self.expand_path_with(|user| match user {
                    UserExpansion::Current => home::home_dir(),
                    UserExpansion::Name(user) => {
                        home::home_dir().and_then(|home| home.parent().map(|home_dirs| home_dirs.join(user)))
                    }
                })
            }
        }
    }
}

#[doc(inline)]
pub use owned::Url as Owned;

pub mod parse;
#[doc(inline)]
pub use parse::parse;
