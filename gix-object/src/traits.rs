use std::io::Write;

use crate::Kind;

/// Writing of objects to a `Write` implementation
pub trait WriteTo {
    /// Write a representation of this instance to `out`.
    fn write_to(&self, out: &mut dyn std::io::Write) -> std::io::Result<()>;

    /// Returns the type of this object.
    fn kind(&self) -> Kind;

    /// Returns the size of this object's representation (the amount
    /// of data which would be written by [`write_to`](Self::write_to)).
    ///
    /// [`size`](Self::size)'s value has no bearing on the validity of
    /// the object, as such it's possible for [`size`](Self::size) to
    /// return a sensible value but [`write_to`](Self::write_to) to
    /// fail because the object was not actually valid in some way.
    fn size(&self) -> u64;

    /// Returns a loose object header based on the object's data
    fn loose_header(&self) -> smallvec::SmallVec<[u8; 28]> {
        crate::encode::loose_header(self.kind(), self.size())
    }
}

impl<T> WriteTo for &T
where
    T: WriteTo,
{
    fn write_to(&self, out: &mut dyn Write) -> std::io::Result<()> {
        <T as WriteTo>::write_to(self, out)
    }

    fn kind(&self) -> Kind {
        <T as WriteTo>::kind(self)
    }

    fn size(&self) -> u64 {
        <T as WriteTo>::size(self)
    }
}
