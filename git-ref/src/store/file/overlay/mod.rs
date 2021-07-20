use crate::store::{file, file::loose, packed};
use std::iter::Peekable;

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references.
///
/// All errors will be returned verbatim, while packed errors are depleted first if loose refs also error.
pub struct LooseThenPacked<'p, 's> {
    parent: &'s file::Store,
    packed: Peekable<packed::Iter<'p>>,
    loose: Peekable<loose::iter::SortedLoosePaths>,
    buf: Vec<u8>,
}

/// A reference returned by the [`LooseThenPacked`] iterator.
#[derive(Debug)]
pub enum Reference<'p, 's> {
    /// A reference originating in a pack
    Packed(packed::Reference<'p>),
    /// A reference from the filesystem
    Loose(loose::Reference<'s>),
}

mod reference;

///
pub mod iter;
