use crate::{Url, UserExpansion};
use bstr::ByteSlice;
use quick_error::quick_error;
use std::path::{Path, PathBuf};

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

/// Path expansion
impl Url {
    pub fn expand_path_with(
        &self,
        home_for_user: impl FnOnce(&UserExpansion) -> Option<PathBuf>,
    ) -> Result<PathBuf, Error> {
        fn join_relative(mut base: PathBuf, path: &Path) -> PathBuf {
            base.extend(path.components().skip(1));
            base
        }
        let path = self.path.to_path()?;
        Ok(match self.expansion.as_ref() {
            Some(user) => join_relative(home_for_user(user).ok_or(Error::MissingHome)?, path),
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
