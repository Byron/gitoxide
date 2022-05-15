use std::borrow::Borrow;
use std::{convert::TryFrom, path::Path};

use git_object::bstr::{BStr, BString, ByteSlice};

use crate::{bstr::ByteVec, FullName, FullNameRef, Namespace};

impl TryFrom<&str> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value.as_bytes().as_bstr())?.into()))
    }
}

impl TryFrom<String> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        git_validate::refname(value.as_bytes().as_bstr())?;
        Ok(FullName(value.into()))
    }
}

impl TryFrom<&BStr> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &BStr) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value)?.into()))
    }
}

impl TryFrom<BString> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        git_validate::refname(value.as_ref())?;
        Ok(FullName(value))
    }
}

impl From<FullName> for BString {
    fn from(name: FullName) -> Self {
        name.0
    }
}

impl<'a> From<&'a FullNameRef> for &'a BStr {
    fn from(name: &'a FullNameRef) -> Self {
        &name.0
    }
}

impl<'a> From<&'a FullNameRef> for FullName {
    fn from(value: &'a FullNameRef) -> Self {
        FullName(value.as_bstr().into())
    }
}

impl std::fmt::Display for FullName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl FullName {
    /// Interpret this fully qualified reference name as partial name.
    pub fn to_partial(&self) -> crate::PartialNameCow<'_> {
        crate::PartialNameCow(self.0.as_bstr().into())
    }

    /// Convert this name into the relative path, lossily, identifying the reference location relative to a repository
    pub fn to_path(&self) -> &Path {
        git_path::from_byte_slice(&self.0)
    }

    /// Dissolve this instance and return the buffer.
    pub fn into_inner(self) -> BString {
        self.0
    }

    /// Return ourselves as byte string which is a valid refname
    pub fn as_bstr(&self) -> &BStr {
        self.0.as_bstr()
    }

    /// Modify ourself so that we use `namespace` as prefix, if it is not yet in the `namespace`
    pub fn prefix_namespace(&mut self, namespace: &Namespace) -> &mut Self {
        if !self.0.starts_with_str(&namespace.0) {
            self.0.insert_str(0, &namespace.0);
        }
        self
    }

    /// Strip the given `namespace` off the beginning of this name, if it is in this namespace.
    pub fn strip_namespace(&mut self, namespace: &Namespace) -> &mut Self {
        if self.0.starts_with_str(&namespace.0) {
            let prev_len = self.0.len();
            self.0.copy_within(namespace.0.len().., 0);
            self.0.resize(prev_len - namespace.0.len(), 0);
        }
        self
    }

    /// Strip well-known prefixes from the name and return it.
    ///
    /// If there is no such prefix, the original name is returned.
    pub fn shorten(&self) -> &BStr {
        self.as_ref().shorten()
    }

    /// Classify this name, or return `None` if it's unclassified.
    pub fn category(&self) -> Option<crate::Category<'_>> {
        self.as_ref().category()
    }

    /// Classify this name, or return `None` if it's unclassified. If `Some`,
    /// the shortened name is returned as well.
    pub fn category_and_short_name(&self) -> Option<(crate::Category<'_>, &BStr)> {
        self.as_ref().category_and_short_name()
    }
}

impl FullNameRef {
    /// Return the file name portion of a full name, for instance `main` if the
    /// full name was `refs/heads/main`.
    pub fn file_name(&self) -> &BStr {
        self.0.rsplitn(2, |b| *b == b'/').next().expect("valid ref").as_bstr()
    }
}

impl Borrow<FullNameRef> for FullName {
    #[inline]
    fn borrow(&self) -> &FullNameRef {
        FullNameRef::new_unchecked(self.0.as_bstr())
    }
}

impl AsRef<FullNameRef> for FullName {
    fn as_ref(&self) -> &FullNameRef {
        self.borrow()
    }
}

impl ToOwned for FullNameRef {
    type Owned = FullName;

    fn to_owned(&self) -> Self::Owned {
        FullName(self.0.to_owned())
    }
}

#[cfg(test)]
mod fullname_tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn cow_works() {
        let _x: Cow<'_, FullNameRef> = Cow::Owned(FullName::try_from("HEAD").unwrap());
    }
}
