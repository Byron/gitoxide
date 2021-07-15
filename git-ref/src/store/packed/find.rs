use crate::{store::packed, PartialName};
use std::convert::TryInto;

/// packed-refs specific functionality
impl packed::Buffer {
    /// Find a reference with the given `name` and return it.
    pub fn find<'a, Name, E>(&self, name: Name) -> Result<Option<packed::Reference<'_>>, E>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
    {
        let _name = name.try_into()?;
        todo!("actual signature and impl")
    }

    /// Find a reference with the given `name` and return it.
    pub fn find_existing<'a, Name, E>(&self, _name: Name) -> std::io::Result<packed::Reference<'_>>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        todo!("actual signature and impl")
    }
}
