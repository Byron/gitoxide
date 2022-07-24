use bstr::BStr;

use crate::Entry;

/// Access
impl<'a> Entry<'a> {
    /// The name to map to.
    pub fn new_name(&self) -> Option<&'a BStr> {
        self.new_name
    }
    /// The email map to.
    pub fn new_email(&self) -> Option<&'a BStr> {
        self.new_email
    }
    /// The name to look for and replace.
    pub fn old_name(&self) -> Option<&'a BStr> {
        self.old_name
    }
    /// The email to look for and replace.
    pub fn old_email(&self) -> &'a BStr {
        self.old_email
    }
}

/// Constructors indicating what kind of mapping is created.
///
/// Only these combinations of values are valid.
#[allow(missing_docs)]
impl<'a> Entry<'a> {
    pub fn change_name_by_email(proper_name: impl Into<&'a BStr>, commit_email: impl Into<&'a BStr>) -> Self {
        Entry {
            new_name: Some(proper_name.into()),
            old_email: commit_email.into(),
            ..Default::default()
        }
    }
    pub fn change_email_by_email(proper_email: impl Into<&'a BStr>, commit_email: impl Into<&'a BStr>) -> Self {
        Entry {
            new_email: Some(proper_email.into()),
            old_email: commit_email.into(),
            ..Default::default()
        }
    }
    pub fn change_name_and_email_by_email(
        proper_name: impl Into<&'a BStr>,
        proper_email: impl Into<&'a BStr>,
        commit_email: impl Into<&'a BStr>,
    ) -> Self {
        Entry {
            new_name: Some(proper_name.into()),
            new_email: Some(proper_email.into()),
            old_email: commit_email.into(),
            ..Default::default()
        }
    }

    pub fn change_name_and_email_by_name_and_email(
        proper_name: impl Into<&'a BStr>,
        proper_email: impl Into<&'a BStr>,
        commit_name: impl Into<&'a BStr>,
        commit_email: impl Into<&'a BStr>,
    ) -> Self {
        Entry {
            new_name: Some(proper_name.into()),
            new_email: Some(proper_email.into()),
            old_name: Some(commit_name.into()),
            old_email: commit_email.into(),
        }
    }
}
