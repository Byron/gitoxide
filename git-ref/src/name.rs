use std::{convert::TryFrom, ffi::OsStr, path::Path};

use git_object::bstr::{BStr, BString, ByteSlice, ByteVec};

use crate::{Category, FullNameRef, PartialNameCow};

/// The error used in the [`PartialNameCow`][super::PartialNameCow]::try_from(…) implementations.
pub type Error = git_validate::reference::name::Error;

impl<'a> Category<'a> {
    /// Return the prefix that would contain all references of our kind, or an empty string if the reference would
    /// be directly inside of the [`git_dir()`][crate::file::Store::git_dir()].
    pub fn prefix(&self) -> &BStr {
        match self {
            Category::Tag => b"refs/tags/".as_bstr(),
            Category::LocalBranch => b"refs/heads/".as_bstr(),
            Category::RemoteBranch => b"refs/remotes/".as_bstr(),
            Category::Note => b"refs/notes/".as_bstr(),
            Category::MainPseudoRef => b"main-worktree/".as_bstr(),
            Category::MainRef => b"main-worktree/refs/".as_bstr(),
            Category::PseudoRef => b"".as_bstr(),
            Category::LinkedPseudoRef { .. } => b"worktrees/".as_bstr(),
            Category::LinkedRef { .. } => b"worktrees/".as_bstr(),
            Category::Bisect => b"refs/bisect/".as_bstr(),
            Category::Rewritten => b"refs/rewritten/".as_bstr(),
            Category::WorktreePrivate => b"refs/worktree/".as_bstr(),
        }
    }

    /// Returns true if the category is private to their worktrees, and never shared with other worktrees.
    pub fn is_worktree_private(&self) -> bool {
        matches!(
            self,
            Category::MainPseudoRef
                | Category::PseudoRef
                | Category::LinkedPseudoRef { .. }
                | Category::WorktreePrivate
                | Category::Rewritten
                | Category::Bisect
        )
    }
}

impl FullNameRef {
    pub(crate) fn new_unchecked(v: &BStr) -> &Self {
        // SAFETY: FullNameRef2 is transparent and equivalent to a &BStr if provided as reference
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute(v)
        }
    }
}

impl FullNameRef {
    /// Convert this name into the relative path identifying the reference location.
    pub fn to_path(&self) -> &Path {
        git_path::from_byte_slice(&self.0)
    }

    /// Return ourselves as byte string which is a valid refname
    pub fn as_bstr(&self) -> &BStr {
        &self.0
    }

    /// Strip well-known prefixes from the name and return it.
    ///
    /// If there is no such prefix, the original name is returned.
    pub fn shorten(&self) -> &BStr {
        self.category_and_short_name()
            .map(|(_, short)| short)
            .unwrap_or_else(|| self.0.as_bstr())
    }

    /// Classify this name, or return `None` if it's unclassified.
    pub fn category(&self) -> Option<Category<'_>> {
        self.category_and_short_name().map(|(cat, _)| cat)
    }

    /// Classify this name, or return `None` if it's unclassified. If `Some`,
    /// the shortened name is returned as well.
    pub fn category_and_short_name(&self) -> Option<(Category<'_>, &BStr)> {
        let name = self.0.as_bstr();
        for category in &[Category::Tag, Category::LocalBranch, Category::RemoteBranch] {
            if let Some(shortened) = name.strip_prefix(category.prefix().as_ref()) {
                return Some((*category, shortened.as_bstr()));
            }
        }

        for category in &[
            Category::Note,
            Category::Bisect,
            Category::WorktreePrivate,
            Category::Rewritten,
        ] {
            if name.starts_with(category.prefix().as_ref()) {
                return Some((
                    *category,
                    name.strip_prefix(b"refs/")
                        .expect("we checked for refs/* above")
                        .as_bstr(),
                ));
            }
        }

        if is_pseudo_ref(name) {
            Some((Category::PseudoRef, name))
        } else if let Some(shortened) = name.strip_prefix(Category::MainPseudoRef.prefix().as_ref()) {
            if shortened.starts_with_str("refs/") {
                (Category::MainRef, shortened.as_bstr()).into()
            } else {
                is_pseudo_ref(shortened).then(|| (Category::MainPseudoRef, shortened.as_bstr()))
            }
        } else if let Some(shortened_with_worktree_name) =
            name.strip_prefix(Category::LinkedPseudoRef { name: "".into() }.prefix().as_ref())
        {
            let (name, shortened) = shortened_with_worktree_name.find_byte(b'/').map(|pos| {
                (
                    shortened_with_worktree_name[..pos].as_bstr(),
                    shortened_with_worktree_name[pos + 1..].as_bstr(),
                )
            })?;
            if shortened.starts_with_str("refs/") {
                (Category::LinkedRef { name }, shortened.as_bstr()).into()
            } else {
                is_pseudo_ref(shortened).then(|| (Category::LinkedPseudoRef { name }, shortened.as_bstr()))
            }
        } else {
            None
        }
    }
}

impl<'a> PartialNameCow<'a> {
    pub(crate) fn looks_like_full_name(&self) -> bool {
        let name = self.0.as_bstr();
        name.starts_with_str("refs/")
            || name.starts_with(Category::MainPseudoRef.prefix())
            || name.starts_with(Category::LinkedPseudoRef { name: "".into() }.prefix())
            || is_pseudo_ref(name)
    }
    pub(crate) fn construct_full_name_ref<'buf>(
        &self,
        add_refs_prefix: bool,
        inbetween: &str,
        buf: &'buf mut BString,
    ) -> &'buf FullNameRef {
        buf.clear();
        if add_refs_prefix && !self.looks_like_full_name() {
            buf.push_str("refs/");
        }
        if !inbetween.is_empty() {
            buf.push_str(inbetween);
            buf.push_byte(b'/');
        }
        buf.extend_from_slice(&self.0);
        FullNameRef::new_unchecked(buf.as_bstr())
    }
}

impl<'a> PartialNameCow<'a> {
    /// Convert this name into the relative path possibly identifying the reference location.
    /// Note that it may be only a partial path though.
    pub fn to_partial_path(&'a self) -> &'a Path {
        git_path::from_byte_slice(self.0.as_ref())
    }

    /// Provide the name as binary string which is known to be a valid partial ref name.
    pub fn as_bstr(&'a self) -> &'a BStr {
        self.0.as_ref()
    }

    /// Append the `component` to ourselves and validate the newly created partial path.
    pub fn join(self, component: impl AsRef<[u8]>) -> Result<PartialNameCow<'static>, Error> {
        let mut b = self.0.into_owned();
        b.push_byte(b'/');
        b.extend(component.as_ref());
        git_validate::reference::name_partial(b.as_ref())?;
        Ok(PartialNameCow(b.into()))
    }
}

impl<'a> TryFrom<&'a BStr> for &'a FullNameRef {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(FullNameRef::new_unchecked(git_validate::reference::name(v)?))
    }
}

impl<'a> From<&'a FullNameRef> for PartialNameCow<'a> {
    fn from(v: &'a FullNameRef) -> Self {
        PartialNameCow(v.0.into())
    }
}

impl<'a> TryFrom<&'a OsStr> for PartialNameCow<'a> {
    type Error = Error;

    fn try_from(v: &'a OsStr) -> Result<Self, Self::Error> {
        let v = git_path::os_str_into_bstr(v)
            .map_err(|_| Error::Tag(git_validate::tag::name::Error::InvalidByte("<unknown encoding>".into())))?;
        Ok(PartialNameCow(
            git_validate::reference::name_partial(v.as_bstr())?.into(),
        ))
    }
}

impl<'a> TryFrom<&'a BStr> for PartialNameCow<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(PartialNameCow(git_validate::reference::name_partial(v)?.into()))
    }
}

impl<'a> TryFrom<&'a str> for &'a FullNameRef {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullNameRef::new_unchecked(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a str> for PartialNameCow<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialNameCow(git_validate::reference::name_partial(v)?.into()))
    }
}

impl<'a> TryFrom<&'a String> for &'a FullNameRef {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullNameRef::new_unchecked(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a String> for PartialNameCow<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialNameCow(git_validate::reference::name_partial(v)?.into()))
    }
}

impl TryFrom<String> for PartialNameCow<'static> {
    type Error = Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        git_validate::reference::name_partial(v.as_bytes().as_bstr())?;
        Ok(PartialNameCow(BString::from(v).into()))
    }
}

impl TryFrom<BString> for PartialNameCow<'static> {
    type Error = Error;

    fn try_from(v: BString) -> Result<Self, Self::Error> {
        git_validate::reference::name_partial(v.as_ref())?;
        Ok(PartialNameCow(v.into()))
    }
}

/// Note that this method is disagreeing with git_validate as it allows dashes '-' for some reason.
/// Since partial names cannot be created with dashes inside we adjusted this as it's probably unintended or git creates pseudo-refs
/// which wouldn't pass its safety checks.
pub(crate) fn is_pseudo_ref<'a>(name: impl Into<&'a BStr>) -> bool {
    name.into().bytes().all(|b| b.is_ascii_uppercase() || b == b'_')
}
