use std::{
    borrow::Cow,
    convert::TryInto,
    path::{Path, PathBuf},
};

use git_object::bstr::{BStr, BString, ByteSlice, ByteVec};
use os_str_bytes::OsStrBytes;

use crate::{Namespace, PartialNameRef};

impl Namespace {
    /// Dissolve ourselves into the interior representation
    pub fn into_bstring(self) -> BString {
        self.0
    }
    /// Return ourselves as
    pub fn as_bstr(&self) -> &BStr {
        self.0.as_ref()
    }
    /// Return ourselves as a path for use within the filesystem.
    pub fn to_path(&self) -> Cow<'_, Path> {
        self.0.to_path().expect("UTF-8 conversion succeeds").into()
    }
    /// Append the given `prefix` to this namespace so it becomes usable for prefixed iteration.
    pub fn into_namespaced_prefix(mut self, prefix: impl AsRef<Path>) -> PathBuf {
        self.0.push_str(prefix.as_ref().to_raw_bytes());
        #[cfg(windows)]
        let path = self.0.replace(b"/", b"\\").into_path_buf();
        #[cfg(not(windows))]
        let path = self.0.replace(b"\\", b"/").into_path_buf();
        path.expect("UTF-8 conversion succeeds")
    }
}

/// Given a `namespace` 'foo we output 'refs/namespaces/foo', and given 'foo/bar' we output 'refs/namespaces/foo/refs/namespaces/bar'.
///
/// For more information, consult the [git namespace documentation](https://git-scm.com/docs/gitnamespaces).
pub fn expand<'a, Name, E>(namespace: Name) -> Result<Namespace, git_validate::refname::Error>
where
    Name: TryInto<PartialNameRef<'a>, Error = E>,
    git_validate::refname::Error: From<E>,
{
    let namespace = namespace.try_into()?.0;
    let mut out = BString::default();
    for component in namespace.split_str(b"/") {
        out.push_str("refs/namespaces/");
        out.push_str(component);
        out.push_str(b"/");
    }
    Ok(Namespace(out))
}
