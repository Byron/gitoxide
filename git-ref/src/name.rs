use std::{
    convert::{Infallible, TryFrom},
    ffi::OsStr,
    path::Path,
};

use git_object::bstr::{BStr, BString, ByteSlice, ByteVec};

use crate::{Category, FullNameRef, PartialNameRef};

/// The error used in the [`PartialNameRef`][super::PartialNameRef]::try_from(…) implementations.
pub type Error = git_validate::reference::name::Error;

impl Category {
    /// Return the prefix that would contain all references of our kind.
    pub fn prefix(&self) -> &BStr {
        match self {
            Category::Tag => b"refs/tags/".as_bstr(),
            Category::LocalBranch => b"refs/heads/".as_bstr(),
            Category::RemoteBranch => b"refs/remotes/".as_bstr(),
            Category::Note => b"refs/notes/".as_bstr(),
        }
    }
}

impl<'a> FullNameRef<'a> {
    /// Convert this name into the relative path identifying the reference location.
    pub fn to_path(self) -> &'a Path {
        git_features::path::from_byte_slice_or_panic_on_windows(self.0)
    }

    /// Return ourselves as byte string which is a valid refname
    pub fn as_bstr(&self) -> &'a BStr {
        self.0
    }

    /// Strip well-known prefixes from the name and return it.
    ///
    /// If there is no such prefix, the original name is returned.
    pub fn shorten(&self) -> &'a BStr {
        let n = self.as_bstr();
        n.strip_prefix(b"refs/tags/")
            .or_else(|| n.strip_prefix(b"refs/heads/"))
            .or_else(|| n.strip_prefix(b"refs/remotes/"))
            .or_else(|| n.strip_prefix(b"refs/"))
            .map(|n| n.as_bstr())
            .unwrap_or(n)
    }

    /// Classify this name, or return `None` if it's unclassified.
    pub fn category(&self) -> Option<Category> {
        self.category_and_short_name().map(|(cat, _)| cat)
    }

    /// Classify this name, or return `None` if it's unclassified. If `Some`,
    /// the shortened name is returned as well.
    pub fn category_and_short_name(&self) -> Option<(Category, &'a BStr)> {
        for category in &[Category::Tag, Category::LocalBranch, Category::RemoteBranch] {
            if let Some(shortened) = self.0.strip_prefix(category.prefix().as_ref()) {
                return Some((*category, shortened.as_bstr()));
            }
        }

        if self.0.starts_with(Category::Note.prefix()) {
            return Some((
                Category::Note,
                self.0
                    .strip_prefix(b"refs/")
                    .expect("we checked for refs/notes above")
                    .as_bstr(),
            ));
        }
        None
    }
}

impl<'a> PartialNameRef<'a> {
    /// Convert this name into the relative path possibly identifying the reference location.
    /// Note that it may be only a partial path though.
    pub fn to_partial_path(&'a self) -> &'a Path {
        git_features::path::from_byte_slice_or_panic_on_windows(self.0.as_ref())
    }

    /// Provide the name as binary string which is known to be a valid partial ref name.
    pub fn as_bstr(&'a self) -> &'a BStr {
        self.0.as_ref()
    }
}

impl PartialNameRef<'static> {
    /// Append the `component` to ourselves and validate the newly created partial path.
    ///
    /// Note that this method is meant to have an owned starting point as this is considered
    /// the typical usecase.
    pub fn join(self, component: impl AsRef<[u8]>) -> Result<Self, crate::name::Error> {
        let mut b = self.0.into_owned();
        b.push_byte(b'/');
        b.extend(component.as_ref());
        git_validate::reference::name_partial(b.as_ref())?;
        Ok(PartialNameRef(b.into()))
    }
}

impl<'a> TryFrom<&'a BStr> for FullNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(FullNameRef(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<FullNameRef<'a>> for PartialNameRef<'a> {
    type Error = Infallible;

    fn try_from(v: FullNameRef<'a>) -> Result<Self, Self::Error> {
        Ok(PartialNameRef(v.0.into()))
    }
}

impl<'a> TryFrom<&'a OsStr> for PartialNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a OsStr) -> Result<Self, Self::Error> {
        let v = git_features::path::os_str_into_bytes(v)
            .map_err(|_| Error::Tag(git_validate::tag::name::Error::InvalidByte("<unknown encoding>".into())))?;
        Ok(PartialNameRef(
            git_validate::reference::name_partial(v.as_bstr())?.into(),
        ))
    }
}

impl<'a> TryFrom<&'a BStr> for PartialNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(PartialNameRef(git_validate::reference::name_partial(v)?.into()))
    }
}

impl<'a> TryFrom<&'a str> for FullNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullNameRef(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a str> for PartialNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialNameRef(git_validate::reference::name_partial(v)?.into()))
    }
}

impl<'a> TryFrom<&'a String> for FullNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullNameRef(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a String> for PartialNameRef<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialNameRef(git_validate::reference::name_partial(v)?.into()))
    }
}

impl TryFrom<String> for PartialNameRef<'static> {
    type Error = Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        git_validate::reference::name_partial(v.as_bytes().as_bstr())?;
        Ok(PartialNameRef(BString::from(v).into()))
    }
}

impl TryFrom<BString> for PartialNameRef<'static> {
    type Error = Error;

    fn try_from(v: BString) -> Result<Self, Self::Error> {
        git_validate::reference::name_partial(v.as_ref())?;
        Ok(PartialNameRef(v.into()))
    }
}
