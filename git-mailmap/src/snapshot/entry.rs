use bstr::BString;

use crate::snapshot::util::{EncodedString, EncodedStringRef};

#[derive(Clone)]
pub(crate) struct NameEntry {
    pub(crate) new_name: Option<BString>,
    pub(crate) new_email: Option<BString>,
    pub(crate) old_name: EncodedString,
}

#[derive(Clone)]
pub(crate) struct EmailEntry {
    pub(crate) new_name: Option<BString>,
    pub(crate) new_email: Option<BString>,
    pub(crate) old_email: EncodedString,

    pub(crate) entries_by_old_name: Vec<NameEntry>,
}

impl EmailEntry {
    pub fn merge(
        &mut self,
        crate::Entry {
            new_name,
            new_email,
            old_name,
            old_email: _,
        }: crate::Entry<'_>,
    ) {
        let new_email = new_email.map(ToOwned::to_owned);
        let new_name = new_name.map(ToOwned::to_owned);
        match old_name {
            None => {
                self.new_email = new_email;
                self.new_name = new_name;
            }
            Some(old_name) => {
                let old_name: EncodedStringRef<'_> = old_name.into();
                match self
                    .entries_by_old_name
                    .binary_search_by(|e| e.old_name.cmp_ref(old_name))
                {
                    Ok(pos) => {
                        let entry = &mut self.entries_by_old_name[pos];
                        entry.new_name = new_name;
                        entry.new_email = new_email;
                    }
                    Err(insert_pos) => self.entries_by_old_name.insert(
                        insert_pos,
                        NameEntry {
                            new_name,
                            new_email,
                            old_name: old_name.into(),
                        },
                    ),
                }
            }
        }
    }
}

impl<'a> From<crate::Entry<'a>> for EmailEntry {
    fn from(
        crate::Entry {
            new_name,
            new_email,
            old_name,
            old_email,
        }: crate::Entry<'a>,
    ) -> Self {
        let mut new_name = new_name.map(ToOwned::to_owned);
        let mut new_email = new_email.map(ToOwned::to_owned);
        let entries_by_old_name = old_name
            .map(|name| {
                vec![NameEntry {
                    new_name: new_name.take(),
                    new_email: new_email.take(),
                    old_name: name.into(),
                }]
            })
            .unwrap_or_default();
        EmailEntry {
            new_name,
            new_email,
            old_email: old_email.into(),
            entries_by_old_name,
        }
    }
}
