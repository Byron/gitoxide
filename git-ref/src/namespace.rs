use crate::PartialName;
use bstr::{BString, ByteSlice, ByteVec};
use std::convert::TryInto;

/// Given a `namespace` 'foo we output 'refs/namespaces/foo', and given 'foo/bar' we output 'refs/namespaces/foo/refs/namespaces/bar'.
///
/// For more information, consult the [git namespace documentation](https://git-scm.com/docs/gitnamespaces).
pub fn expand<'a, Name, E>(namespace: Name) -> Result<crate::mutable::FullName, git_validate::refname::Error>
where
    Name: TryInto<PartialName<'a>, Error = E>,
    git_validate::refname::Error: From<E>,
{
    let namespace = namespace.try_into()?.0;
    let mut out = BString::default();
    for component in namespace.split_str(b"/") {
        out.push_str("refs/namespaces/");
        out.push_str(component);
        out.push_str(b"/");
    }
    out.pop();
    debug_assert!(
        git_validate::reference::name(out.as_ref()).is_ok(),
        "we always produce valid ref names"
    );
    Ok(crate::mutable::FullName(out))
}
