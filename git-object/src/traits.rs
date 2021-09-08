use std::io::Write;

use crate::Kind;

/// Writing of objects to a `Write` implementation
pub trait WriteTo {
    /// Write a representation of this instance to `out`.
    fn write_to(&self, out: impl std::io::Write) -> std::io::Result<()>;

    /// Returns the type of this object.
    fn kind(&self) -> Kind;
}

impl<T> WriteTo for &T
where
    T: WriteTo,
{
    fn write_to(&self, out: impl Write) -> std::io::Result<()> {
        <T as WriteTo>::write_to(self, out)
    }

    fn kind(&self) -> Kind {
        <T as WriteTo>::kind(self)
    }
}
