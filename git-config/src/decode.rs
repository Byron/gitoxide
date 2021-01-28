use crate::{borrowed, value};
use bstr::BStr;
use quick_error::quick_error;
use std::{borrow::Cow, path::PathBuf};

#[allow(missing_docs)]
quick_error! {
    /// The error returned by [`value()`] or any conversion method within the [decode][crate::decode] module.
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("let's see what can go wrong and how we do it")
        }
        NoValue {
            display("Entry has no value (TODO: much more error information)")
        }
    }
}

/// Decode an entry value - it can be [encoded as described in the git config documentation](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/Documentation/config.txt#L74:L80)
pub fn value(_input: &BStr) -> Result<Cow<'_, BStr>, Error> {
    unimplemented!("decode value from bstr")
}

/// Conversion methods for the value of an entry.
impl<'a> borrowed::Entry<'a> {
    /// Returns the name of this entry.
    pub fn name(&self) -> Result<Cow<'a, BStr>, Error> {
        value(
            self.parent
                .bytes_at(self.parent.token(self.index).as_entry().expect("entry").name),
        )
        .map_err(Into::into)
    }

    /// Returns the entry's value as byte string.
    pub fn as_string(&self) -> Result<Cow<'a, BStr>, Error> {
        value(
            self.parent.bytes_at(
                self.parent
                    .token(self.index)
                    .as_entry()
                    .expect("entry")
                    .value
                    .ok_or(Error::NoValue)?,
            ),
        )
        .map_err(Into::into)
    }
    /// Returns the entry's value and interpret it as integer.
    pub fn as_int(&self) -> Result<i64, Error> {
        unimplemented!("as int")
    }
    /// Returns the entry's value and interpret it as boolean.
    pub fn as_bool(&self) -> Result<bool, Error> {
        unimplemented!("as bool")
    }
    /// Returns the entry's value and interpret it as path after applying [path expansion][crate::value::resolve::path()].
    pub fn as_path(&self) -> Result<PathBuf, Error> {
        unimplemented!("as bool")
    }
    /// Returns the entry's value and interpret it as color.
    pub fn as_color(&self) -> Result<value::Color, Error> {
        unimplemented!("as bool")
    }
}
