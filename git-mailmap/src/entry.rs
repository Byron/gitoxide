use crate::Entry;
use bstr::BStr;

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
