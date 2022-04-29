use crate::Spec;
use bstr::{BStr, ByteSlice, ByteVec};
use std::ffi::OsStr;
use std::str::FromStr;

impl std::convert::TryFrom<&OsStr> for Spec {
    type Error = crate::Utf8Error;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        crate::os_str_into_bstr(value).map(|value| Spec(value.into()))
    }
}

impl FromStr for Spec {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Spec(s.into()))
    }
}

impl Spec {
    /// Return all paths described by this path spec, using slashes on all platforms.
    pub fn items(&self) -> impl Iterator<Item = &BStr> {
        std::iter::once(self.0.as_bstr())
    }
    /// Adjust this path specification according to the given `prefix`, which may be empty to indicate we are the at work-tree root.
    // TODO: this is a hack, needs test and time to do according to spec. This is just a minimum version to have -something-.
    pub fn apply_prefix(&mut self, prefix: &std::path::Path) -> &Self {
        assert!(!self.0.contains_str(b"/../"));
        assert!(!self.0.contains_str(b"/./"));
        assert!(!self.0.starts_with_str(b"../"));
        assert!(!self.0.starts_with_str(b"./"));
        assert!(!self.0.starts_with_str(b"/"));
        // many more things we can't handle. `Path` never ends with trailing path separator.

        let prefix = crate::into_bstr(prefix);
        if !prefix.is_empty() {
            let mut prefix = crate::to_unix_separators_on_windows(prefix);
            {
                let path = prefix.to_mut();
                path.push_byte(b'/');
                path.extend_from_slice(&self.0);
            }
            self.0 = prefix.into_owned();
        }
        self
    }
}
