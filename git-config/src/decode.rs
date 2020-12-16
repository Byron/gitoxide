use crate::{borrowed, value};
use bstr::BStr;
use quick_error::quick_error;
use std::{borrow::Cow, path::PathBuf};

quick_error! {
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
    pub fn name(&self) -> Result<Cow<'a, BStr>, Error> {
        value(
            self.parent
                .bytes_at(self.parent.token(self.index).as_entry().expect("entry").name),
        )
        .map_err(Into::into)
    }

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
    pub fn as_int(&self) -> Result<i64, Error> {
        unimplemented!("as int")
    }
    pub fn as_bool(&self) -> Result<bool, Error> {
        unimplemented!("as bool")
    }
    pub fn as_path(&self) -> Result<PathBuf, Error> {
        unimplemented!("as bool")
    }
    pub fn as_color(&self) -> Result<value::Color, Error> {
        unimplemented!("as bool")
    }
}
