use crate::fetch;
use bstr::{BStr, BString, ByteSlice};
use git_transport::client;
use std::convert::TryFrom;

pub struct Capabilities {
    pub available: Vec<(BString, Option<BString>)>,
    pub symrefs: Vec<BString>,
}

impl Capabilities {
    pub fn find_first(&self, name: &str) -> Option<&(BString, Option<BString>)> {
        self.available.iter().find(|(n, _)| n == name.as_bytes().as_bstr())
    }
    /// Returns values of capability of the given name, if present.
    /// Useful when handling capabilities of V2 commands.
    pub fn values_of(&self, name: &str) -> Option<impl Iterator<Item = &BStr>> {
        self.find_first(name)
            .and_then(|(_, v)| v.as_ref().map(|v| v.split(|b| *b == b' ').map(|v| v.as_bstr())))
    }
}

impl TryFrom<client::Capabilities> for Capabilities {
    type Error = fetch::Error;

    fn try_from(c: client::Capabilities) -> Result<Self, Self::Error> {
        let (available, symrefs) = {
            let mut caps = Vec::new();
            let mut symrefs = Vec::new();
            for c in c.iter() {
                if c.name() == b"symref".as_bstr() {
                    symrefs.push(c.value().ok_or(fetch::Error::SymrefWithoutValue)?.to_owned());
                } else {
                    caps.push((c.name().to_owned(), c.value().map(|v| v.to_owned())));
                }
            }
            (caps, symrefs)
        };
        Ok(Capabilities { available, symrefs })
    }
}
