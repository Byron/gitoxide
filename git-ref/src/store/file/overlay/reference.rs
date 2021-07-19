use super::Reference;
use crate::mutable;
use std::convert::TryInto;

impl<'p, 's> Reference<'p, 's> {
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
            Reference::Loose(r) => r.into_target(),
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
    pub fn name(&self) -> Result<mutable::FullName, git_validate::refname::Error> {
        match self {
            Reference::Packed(p) => p.full_name.try_into(),
            Reference::Loose(l) => Ok(l.name()),
        }
    }

    /// Return the target to which the reference points to.
    pub fn target(&self) -> mutable::Target {
        match self {
            Reference::Packed(p) => mutable::Target::Peeled(p.target()),
            Reference::Loose(l) => l.target().to_owned(),
        }
    }
}
