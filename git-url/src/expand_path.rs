use crate::{Url, UserExpansion};
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

/// Path expansion
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
