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
        MissingHome(user: Option<String>) {
            display("Home directory could not be obtained for {}", match user {Some(user) => format!("user '{}'", user), None => "current user".into()})
        }
    }
}

/// Path expansion
impl Url {
    pub fn expand_path_with(
        &self,
        home_for_user: impl FnOnce(&UserExpansion) -> Option<PathBuf>,
    ) -> Result<PathBuf, Error> {
        fn make_relative(path: &Path) -> PathBuf {
            path.components().skip(1).collect()
        }
        let path = self.path.to_path()?;
        Ok(match self.expansion.as_ref() {
            Some(user) => home_for_user(user)
                .ok_or_else(|| Error::MissingHome(user.to_owned().into()))?
                .join(make_relative(path)),
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
