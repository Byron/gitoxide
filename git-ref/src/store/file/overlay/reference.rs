use super::Reference;
use crate::{
    mutable,
    store::{
        file::{log, loose},
        packed,
    },
    FullName,
};
use git_hash::ObjectId;
use std::convert::TryFrom;

impl<'p, 's> TryFrom<Reference<'p, 's>> for crate::file::loose::Reference<'s> {
    type Error = ();

    fn try_from(value: Reference<'p, 's>) -> Result<Self, Self::Error> {
        match value {
            Reference::Loose(l) => Ok(l),
            Reference::Packed(_) => Err(()),
        }
    }
}

impl<'p, 's> Reference<'p, 's> {
    /// For details, see [crate::file::Reference::log_exists()].
    pub fn log_exists(&self) -> Result<bool, loose::reflog::Error> {
        match self {
            Reference::Loose(r) => r.log_exists(),
            Reference::Packed(_) => todo!("packed log exists"),
        }
    }

    /// For details, see [crate::file::Reference::peel_to_id_in_place].
    pub fn peel_to_id_in_place(
        &mut self,
        packed: Option<&packed::Buffer>,
    ) -> Result<ObjectId, crate::store::file::loose::reference::peel::to_id::Error> {
        match self {
            Reference::Loose(r) => r.peel_to_id_in_place(packed).map(ToOwned::to_owned),
            Reference::Packed(p) => {
                if let Some(object) = p.object {
                    p.target = object;
                }
                p.object = None;
                Ok(p.target())
            }
        }
    }

    /// For details, see [crate::file::Reference::peel_one_level].
    pub fn peel_one_level<'p2>(
        &self,
        packed: Option<&'p2 packed::Buffer>,
    ) -> Option<Result<Reference<'p2, 's>, crate::store::file::loose::reference::peel::Error>> {
        match self {
            Reference::Loose(r) => r.peel_one_level(packed),
            Reference::Packed(_) => todo!("packed peel one level (yeah, it's done)"),
        }
    }
    /// Obtain a reverse iterator over logs of this reference. See [crate::file::Reference::log_iter_rev()] for details.
    pub fn log_iter_rev<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<Option<log::iter::Reverse<'b, std::fs::File>>, loose::reflog::Error> {
        match self {
            Reference::Loose(r) => r.log_iter_rev(buf),
            Reference::Packed(_) => todo!("packed log iter rev"),
        }
    }

    /// Obtain an iterator over logs of this reference. See [crate::file::Reference::log_iter()] for details.
    pub fn log_iter<'b>(
        &self,
        buf: &'b mut Vec<u8>,
    ) -> Result<Option<impl Iterator<Item = Result<log::Line<'b>, log::iter::decode::Error>>>, loose::reflog::Error>
    {
        match self {
            Reference::Loose(r) => r.log_iter(buf),
            Reference::Packed(_) => todo!("packed log iter"),
        }
    }

    /// Returns the kind of reference
    pub fn kind(&self) -> crate::Kind {
        match self {
            Reference::Loose(r) => r.kind(),
            Reference::Packed(_) => crate::Kind::Peeled,
        }
    }

    /// Transform this reference into an owned `Target`
    pub fn into_target(self) -> mutable::Target {
        match self {
            Reference::Packed(p) => mutable::Target::Peeled(p.object()),
            Reference::Loose(r) => r.target,
        }
    }

    /// Returns true if this ref is located in a packed ref buffer.
    pub fn is_packed(&self) -> bool {
        match self {
            Reference::Packed(_) => true,
            Reference::Loose(_) => false,
        }
    }

    /// Return the full validated name of the reference. Please note that if the reference is packed, validation can fail here.
    pub fn name(&self) -> FullName<'_> {
        match self {
            Reference::Packed(p) => p.name,
            Reference::Loose(l) => l.name.borrow(),
        }
    }

    /// Return the target to which the reference points to.
    pub fn target(&self) -> mutable::Target {
        match self {
            Reference::Packed(p) => mutable::Target::Peeled(p.target()),
            Reference::Loose(l) => l.target.clone(),
        }
    }
}
