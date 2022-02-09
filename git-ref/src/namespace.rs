use std::{
    borrow::Cow,
    convert::TryInto,
    path::{Path, PathBuf},
};

use git_object::bstr::{BStr, BString, ByteSlice, ByteVec};

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
        self.0.push_str(
            git_features::path::into_bytes(prefix.as_ref()).expect("prefix path doesn't contain ill-formed UTF-8"),
        );
        let path = git_features::path::from_byte_vec(
            git_features::path::convert::to_native_separators({
                let v: Vec<_> = self.0.into();
                v
            })
            .into_owned(),
        );
        path.expect("well-formed UTF-8 on windows")
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
